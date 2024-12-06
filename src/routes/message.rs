use crate::models::message::{MessageInput, MessageResponse};
use crate::services::weather_service;
use crate::utils::date_utils;
use axum::Json;

pub async fn handle_message(Json(payload): Json<MessageInput>) -> Json<MessageResponse> {
    if payload.content.to_lowercase() == "oi márcia" {
        let date_info = date_utils::get_current_date_info();

        let weather_info = match weather_service::get_weather_in_osasco().await {
            Ok(weather) => weather,
            Err(_) => weather_service::default_weather_info(),
        };

        let response = format!(
            "Olá! Hoje é {} de {}, dia de {}. Em Osasco, a temperatura máxima está {} graus, mínima {} graus, com previsão para o dia: {}, e para a noite: {}.",
            date_info.day,
            date_utils::get_month_name(date_info.month),
            date_info.weekday,
            weather_info.max_temperature,
            weather_info.min_temperature,
            weather_info.description_day,
            weather_info.description_night
        );

        Json(MessageResponse { reply: response })
    } else {
        Json(MessageResponse {
            reply: "Desculpe, não entendi. Você quis dizer 'Oi Marcia'?".to_string(),
        })
    }
}
