use crate::models::message::{CommandLogInput, CommandLogResponse};
use axum::Json;

pub async fn log_command(Json(payload): Json<CommandLogInput>) -> Json<CommandLogResponse> {
    println!("Logged Command: {}", payload.command);

    Json(CommandLogResponse {
        status: "Command logged successfully".to_string(),
    })
}
