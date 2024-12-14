use crate::models::message::{MessageInput, MessageResponse};
use crate::services::{weather_service, news_service, database_service::DatabaseService, audio_service};
use crate::utils::date_utils;
use axum::{extract::State, Json}; 
use axum::response::IntoResponse;
use std::sync::Arc; 

pub async fn handle_message(
    State(database_service): State<Arc<DatabaseService>>, 
    Json(payload): Json<MessageInput>
) -> impl IntoResponse {
    if payload.content.to_lowercase() == "oi márcia" {
        let date_info = date_utils::get_current_date_info();

        let weather_info = match weather_service::get_weather_in_osasco().await {
            Ok(weather) => weather,
            Err(_) => weather_service::default_weather_info(),
        };

        let max_temp_c = (weather_info.max_temperature - 32.0) * 5.0 / 9.0;
        let min_temp_c = (weather_info.min_temperature - 32.0) * 5.0 / 9.0;

        let articles = match news_service::get_top_headlines().await {
            Ok(a) => a,
            Err(_) => vec![],
        };

        let news_formatted = news_service::format_news_articles(&articles);

        let response = format!(
            "Olá! Rosiii!! ... tudo bem ?? tudo joinha com você ? ..... dormiu mais que a cama ? ...... Hoje é {} de {}, dia de {}...... vamos ver se o tempo vai estar jururû ou bem humorado..... deixa eu procurar aqui.... Aqui, achei. Em Osasco, a temperatura máxima está {:.0} graus, mínima {:.0} graus, com previsão para o dia: {}, e para a noite: {}. ula ia heim!!! .... agora deixa eu ver aqui o que aconteceu no mundo enquanto você dormia. ... Aqui... la vai algumas notícias do Brasil:{}",
            date_info.day,
            date_utils::get_month_name(date_info.month),
            date_info.weekday,
            max_temp_c,
            min_temp_c,
            weather_info.description_day,
            weather_info.description_night,
            news_formatted 
        );

         let audio_path = audio_service::generate_audio(&response).await;



        database_service.insert_event(
            "message", 
            Some("Oi Marcia"), 
            Some(&payload.content), 
            Some(&response), 
            None, 
            None, 
            Some("success")
        ).await;

        Json(MessageResponse { reply: response })
    } else {
        Json(MessageResponse {
            reply: "Desculpe, não entendi. Você quis dizer 'Oi Marcia'?".to_string(),
        })
    }
}
