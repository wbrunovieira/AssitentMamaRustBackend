use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct AccuWeatherResponse {
    #[serde(rename = "DailyForecasts")]
    pub daily_forecasts: Vec<DailyForecast>,
}

#[derive(Deserialize)]
pub struct DailyForecast {
    pub temperature: Temperature,
    pub day: WeatherDescription,
    pub night: WeatherDescription,
}

#[derive(Deserialize)]
pub struct Temperature {
    pub minimum: Value,
    pub maximum: Value,
}

#[derive(Deserialize)]
pub struct Value {
    pub value: f64,

}

#[derive(Deserialize)]
pub struct WeatherDescription {
    pub icon_phrase: String,
    pub precipitation_probability: u8,
}

#[derive(Serialize, Debug)]
pub struct WeatherInfo {
    pub max_temperature: f64,
    pub min_temperature: f64,
    pub description_day: String,
    pub description_night: String,
    pub precipitation_probability_day: u8,
    pub precipitation_probability_night: u8,
}
