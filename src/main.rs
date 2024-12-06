mod models;
mod routes;
mod services;
mod utils;

use axum::http::Method;
use axum::routing::{get, post};
use axum::Router;
use dotenv::dotenv;
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
        .route("/", get(routes::root::handler))
        .route("/message", post(routes::message::handle_message))
        .route("/log-command", post(routes::log::log_command))
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3030));
    println!("Servidor rodando em http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
