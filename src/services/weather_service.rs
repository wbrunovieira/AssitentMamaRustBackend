use crate::models::weather::{AccuWeatherResponse, WeatherInfo};
use reqwest::Client;
use std::env;

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

    if let Some(forecast) = weather_data.daily_forecasts.get(0) {
        Ok(WeatherInfo {
            max_temperature: forecast.temperature.maximum.value,
            min_temperature: forecast.temperature.minimum.value,
            description_day: forecast.day.icon_phrase.clone(),
            description_night: forecast.night.icon_phrase.clone(),
            precipitation_probability_day: forecast.day.precipitation_probability,
            precipitation_probability_night: forecast.night.precipitation_probability,
        })
    } else {
        Err("Nenhuma previsão encontrada na resposta da API".to_string())
    }
}

pub fn default_weather_info() -> WeatherInfo {
    WeatherInfo {
        max_temperature: 0.0,
        min_temperature: 0.0,
        description_day: "Informação indisponível".to_string(),
        description_night: "Informação indisponível".to_string(),
        precipitation_probability_day: 0,
        precipitation_probability_night: 0,
    }
}
