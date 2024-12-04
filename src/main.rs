use axum::http::Method;
use axum::{
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tower_http::cors::{AllowOrigin, CorsLayer};

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::exact("http://localhost:5173".parse().unwrap()))
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([axum::http::header::CONTENT_TYPE]);

    let app = Router::new()
        .route("/", get(root))
        .route("/message", post(handle_message))
        .route("/log-command", post(log_command)) // New endpoint for logging commands
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3030));
    println!("Servidor rodando em http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Bem-vindo ao assistente Márcia!"
}

#[derive(Deserialize, Debug)]
struct MessageInput {
    content: String,
}

#[derive(Deserialize, Debug)]
struct CommandLogInput {
    command: String,
}

#[derive(Serialize, Debug)]
struct MessageResponse {
    reply: String,
}

#[derive(Serialize, Debug)]
struct CommandLogResponse {
    status: String,
}

async fn handle_message(Json(payload): Json<MessageInput>) -> Json<MessageResponse> {
    let response = if payload.content.to_lowercase() == "oi marcia" {
        "Olá! Como posso ajudar você hoje?"
    } else {
        "Desculpe, não entendi. Você quis dizer 'Oi Marcia'?"
    };

    Json(MessageResponse {
        reply: response.to_string(),
    })
}

// New endpoint to handle command logging
async fn log_command(Json(payload): Json<CommandLogInput>) -> Json<CommandLogResponse> {
    // Here you can add any logging logic you want
    println!("Logged Command: {}", payload.command);

    Json(CommandLogResponse {
        status: "Command logged successfully".to_string(),
    })
}
