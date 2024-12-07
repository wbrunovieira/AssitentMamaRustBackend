use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct AccuWeatherResponse {
    #[serde(rename = "DailyForecasts")]
    pub daily_forecasts: Vec<DailyForecast>,
}

#[derive(Deserialize, Debug)]
pub struct DailyForecast {
    #[serde(rename = "Date")]
    pub date: String,
    #[serde(rename = "EpochDate")]
    pub epoch_date: i64,
    #[serde(rename = "Sun")]
    pub sun: Sun,
    #[serde(rename = "Moon")]
    pub moon: Moon,
    #[serde(rename = "Temperature")]
    pub temperature: Temperature,
    #[serde(rename = "Day")]
    pub day: WeatherDescription,
    #[serde(rename = "Night")]
    pub night: WeatherDescription,
    
}

#[derive(Deserialize, Debug)]
pub struct Sun {
    #[serde(rename = "Rise")]
    pub rise: String,
    #[serde(rename = "Set")]
    pub set: String,
   
}

#[derive(Deserialize, Debug)]
pub struct Moon {
    #[serde(rename = "Rise")]
    pub rise: String,
    #[serde(rename = "Set")]
    pub set: String,
    #[serde(rename = "Phase")]
    pub phase: String,
    #[serde(rename = "Age")]
    pub age: u8,

}

#[derive(Deserialize, Debug)]
pub struct Temperature {
    #[serde(rename = "Minimum")]
    pub minimum: Value,
    #[serde(rename = "Maximum")]
    pub maximum: Value,
}

#[derive(Deserialize, Debug)]
pub struct Value {
  
    #[serde(rename = "Value")]
    pub value: f64,

}

#[derive(Deserialize, Debug)]
pub struct WeatherDescription {
  
    #[serde(rename = "IconPhrase")]
    pub icon_phrase: String,
    #[serde(rename = "PrecipitationProbability")]
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
