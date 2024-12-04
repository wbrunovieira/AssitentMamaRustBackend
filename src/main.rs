use axum::http::Method;
use axum::{
    routing::{get, post},
    Json, Router,
};
use chrono::{Datelike, Local, Weekday};
use dotenv::dotenv;
use reqwest;
use serde::{Deserialize, Serialize};
use std::env;
use std::net::SocketAddr;
use tower_http::cors::{AllowOrigin, CorsLayer};

#[tokio::main]
async fn main() {
    // Load environment variables
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

    // Map weekday to Portuguese
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

async fn get_weather_in_osasco() -> Result<WeatherInfo, String> {
    let api_key = env::var("ACCUWEATHER_API_KEY")
        .map_err(|_| "API key not found in environment".to_string())?;
    let location_key = env::var("ACCUWEATHER_OSASCO_LOCATION_KEY")
        .map_err(|_| "Location key not found in environment".to_string())?;

    let url = format!(
        "http://dataservice.accuweather.com/forecasts/v1/daily/1day/{}?apikey={}&language=pt-br&details=true",
        location_key, api_key
    );

    let client = reqwest::Client::new();
    match client.get(&url).send().await {
        Ok(response) => {
            if response.status().is_success() {
                match response.json::<AccuWeatherResponse>().await {
                    Ok(weather_data) => {
                        let forecast = &weather_data.daily_forecasts[0];
                        Ok(WeatherInfo {
                            temperature: forecast.temperature.maximum.value,
                            description: forecast.day.icon_phrase.clone(),
                        })
                    }
                    Err(_) => Err("Falha ao parsear resposta da API".to_string()),
                }
            } else {
                Err("Falha ao buscar dados meteorológicos".to_string())
            }
        }
        Err(_) => Err("Erro de conexão com a API de clima".to_string()),
    }
}

async fn handle_message(Json(payload): Json<MessageInput>) -> Json<MessageResponse> {
    if payload.content.to_lowercase() == "oi marcia" {
        let date_info = get_current_date_info();

        let weather_info = match get_weather_in_osasco().await {
            Ok(weather) => weather,
            Err(e) => {
                println!("Erro ao buscar clima: {}", e);
                WeatherInfo {
                    temperature: 0.0,
                    description: "Informação indisponível".to_string(),
                }
            }
        };

        let response = format!(
            "Olá! Hoje é {} de {}, dia de {}. Em Osasco, a temperatura máxima está {} graus, {}.",
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
            weather_info.temperature,
            weather_info.description
        );

        Json(MessageResponse { reply: response })
    } else {
        Json(MessageResponse {
            reply: "Desculpe, não entendi. Você quis dizer 'Oi Marcia'?".to_string(),
        })
    }
}

#[derive(Deserialize, Debug)]
struct AccuWeatherResponse {
    daily_forecasts: Vec<DailyForecast>,
}

#[derive(Deserialize, Debug)]
struct DailyForecast {
    temperature: Temperature,
    day: DayConditions,
}

#[derive(Deserialize, Debug)]
struct Temperature {
    maximum: TemperatureValue,
}

#[derive(Deserialize, Debug)]
struct TemperatureValue {
    value: f32,
}

#[derive(Deserialize, Debug)]
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
struct WeatherInfo {
    temperature: f32,
    description: String,
}

async fn log_command(Json(payload): Json<CommandLogInput>) -> Json<CommandLogResponse> {
    println!("Logged Command: {}", payload.command);

    Json(CommandLogResponse {
        status: "Command logged successfully".to_string(),
    })
}

#[derive(Deserialize, Debug)]
struct CommandLogInput {
    command: String,
}

#[derive(Serialize, Debug)]
struct CommandLogResponse {
    status: String,
}

async fn root() -> &'static str {
    "Bem-vindo ao assistente Márcia!"
}
