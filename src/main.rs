use axum::http::Method;
use axum::{
    routing::{get, post},
    Json, Router,
};
use chrono::{Datelike, Local, Weekday};
use dotenv::dotenv;

use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use std::net::SocketAddr;
use tower_http::cors::{AllowOrigin, CorsLayer};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::exact("http://localhost:5173".parse().unwrap()))
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([axum::http::header::CONTENT_TYPE]);

    let app = Router::new()
        .route("/", get(root))
        .route("/message", post(handle_message))
        .route("/log-command", post(log_command))
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3030));
    println!("Servidor rodando em http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn get_current_date_info() -> DateInfo {
    let now = Local::now();

    let weekday = match now.weekday() {
        Weekday::Mon => "Segunda-feira",
        Weekday::Tue => "Terça-feira",
        Weekday::Wed => "Quarta-feira",
        Weekday::Thu => "Quinta-feira",
        Weekday::Fri => "Sexta-feira",
        Weekday::Sat => "Sábado",
        Weekday::Sun => "Domingo",
    };

    DateInfo {
        day: now.day(),
        month: now.month(),
        weekday: weekday.to_string(),
    }
}

#[derive(Deserialize)]
struct AccuWeatherResponse {
    DailyForecasts: Vec<DailyForecast>,
}

#[derive(Deserialize)]
struct DailyForecast {
    Temperature: Temperature,
    Day: WeatherDescription,
    Night: WeatherDescription,
}

#[derive(Deserialize)]
struct Temperature {
    Minimum: Value,
    Maximum: Value,
}

#[derive(Deserialize)]
struct Value {
    Value: f64,
    Unit: String,
}

#[derive(Deserialize)]
struct WeatherDescription {
    IconPhrase: String,
    PrecipitationProbability: u8,
}

#[derive(Serialize, Debug)]
struct WeatherInfo {
    max_temperature: f64,
    min_temperature: f64,
    description_day: String,
    description_night: String,
    precipitation_probability_day: u8,
    precipitation_probability_night: u8,
}

struct DayConditions {
    icon_phrase: String,
}

#[derive(Deserialize, Debug)]
struct MessageInput {
    content: String,
}

#[derive(Serialize, Debug)]
struct MessageResponse {
    reply: String,
}

#[derive(Deserialize, Debug)]
struct DateInfo {
    day: u32,
    month: u32,
    weekday: String,
}

#[derive(Deserialize, Debug)]
struct TemperatureValue {
    value: f32,
}

#[derive(Deserialize, Debug)]
struct CommandLogInput {
    command: String,
}

#[derive(Serialize, Debug)]
struct CommandLogResponse {
    status: String,
}

pub async fn get_weather_in_osasco() -> Result<WeatherInfo, String> {
    let api_key = env::var("ACCUWEATHER_API_KEY")
        .map_err(|_| "API key not found in environment".to_string())?;
    let location_key = env::var("ACCUWEATHER_OSASCO_LOCATION_KEY")
        .map_err(|_| "Location key not found in environment".to_string())?;

    let url = format!(
        "http://dataservice.accuweather.com/forecasts/v1/daily/1day/{}?apikey={}&language=pt-br&details=true",
        location_key, api_key
    );

    let client = Client::new();

    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|err| format!("Erro de conexão com a API de clima: {}", err))?;

    if !response.status().is_success() {
        return Err(format!(
            "Falha ao buscar dados meteorológicos: HTTP {}",
            response.status()
        ));
    }

    let weather_data: AccuWeatherResponse = response
        .json()
        .await
        .map_err(|err| format!("Falha ao parsear resposta da API: {}", err))?;

    if let Some(forecast) = weather_data.DailyForecasts.get(0) {
        Ok(WeatherInfo {
            max_temperature: forecast.Temperature.Maximum.Value,
            min_temperature: forecast.Temperature.Minimum.Value,
            description_day: forecast.Day.IconPhrase.clone(),
            description_night: forecast.Night.IconPhrase.clone(),
            precipitation_probability_day: forecast.Day.PrecipitationProbability,
            precipitation_probability_night: forecast.Night.PrecipitationProbability,
        })
    } else {
        Err("Nenhuma previsão encontrada na resposta da API".to_string())
    }
}

async fn handle_message(Json(payload): Json<MessageInput>) -> Json<MessageResponse> {
    println!("Comando recebido: {:?}", payload.content.to_lowercase());
    println!("Payload recebido: {:?}", payload);

    if payload.content.to_lowercase() == "oi márcia" {
        let date_info = get_current_date_info();
        println!("Informações da data: {:?}", date_info);

        let weather_info = match get_weather_in_osasco().await {
            Ok(weather) => {
                println!("Informações do clima: {:?}", weather);
                weather
            }
            Err(e) => {
                println!("Erro ao buscar informações meteorológicas: {}", e);
                WeatherInfo {
                    max_temperature: 0.0,
                    min_temperature: 0.0,
                    description_day: "Informação indisponível".to_string(),
                    description_night: "Informação indisponível".to_string(),
                    precipitation_probability_day: 0,
                    precipitation_probability_night: 0,
                }
            }
        };

        let response = format!(
            "Olá! Hoje é {} de {}, dia de {}. Em Osasco, a temperatura máxima está {} graus, mínima {} graus, com previsão para o dia: {}, e para a noite: {}.",
            date_info.day,
            match date_info.month {
                1 => "Janeiro",
                2 => "Fevereiro",
                3 => "Março",
                4 => "Abril",
                5 => "Maio",
                6 => "Junho",
                7 => "Julho",
                8 => "Agosto",
                9 => "Setembro",
                10 => "Outubro",
                11 => "Novembro",
                12 => "Dezembro",
                _ => "Mês Desconhecido",
            },
            date_info.weekday,
            weather_info.max_temperature,
            weather_info.min_temperature,
            weather_info.description_day,
            weather_info.description_night
        );

        println!("Resposta gerada: {}", response);
        println!("Resposta final: {:?}", response);
        Json(MessageResponse { reply: response })
    } else {
        println!("Comando não reconhecido. Enviando mensagem padrão.");
        Json(MessageResponse {
            reply: "Desculpe, não entendi. Você quis dizer 'Oi Marcia'?".to_string(),
        })
    }
}

async fn log_command(Json(payload): Json<CommandLogInput>) -> Json<CommandLogResponse> {
    println!("Logged Command: {}", payload.command);

    Json(CommandLogResponse {
        status: "Command logged successfully".to_string(),
    })
}

async fn root() -> &'static str {
    "Bem-vindo ao assistente Márcia!"
}
