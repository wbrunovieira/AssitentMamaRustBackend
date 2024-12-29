mod models;
mod routes;
mod services;
mod utils;

use axum::{
   
    http::Method,
    routing::{get, post},
    Router,
};
use dotenv::dotenv;
use hyper::{server::conn::http1, service::service_fn};
use hyper_util::rt::TokioIo;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::cors::{AllowOrigin, CorsLayer};
use tower::util::ServiceExt; // Importa o trait para usar `oneshot`
use services::database_service::DatabaseService;

#[tokio::main]
async fn main() {
    // Carrega variáveis de ambiente do arquivo .env
    dotenv().ok();

    // Inicializa o serviço de banco de dados
    let database_service = Arc::new(DatabaseService::new().await);
    database_service.initialize_database().await;

    // Configura CORS
    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::exact("http://localhost:5173".parse().unwrap()))
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([axum::http::header::CONTENT_TYPE]);

    // Define as rotas do aplicativo
    let app = Router::new()
        .route("/", get(routes::root::handler))
        .route("/message", post(routes::message::handle_message))
        .route("/log-command", post(routes::log::log_command))
        .layer(cors)
        .with_state(database_service.clone());

    // Configura o endereço do servidor
    let addr = SocketAddr::from(([127, 0, 0, 1], 3030));
    println!("[INFO] Servidor rodando em http://{}", addr);

    // Cria um listener para aceitar conexões TCP
    let listener = TcpListener::bind(addr).await.unwrap();

    // Loop de aceitação de conexões
    loop {
        let (stream, _) = listener.accept().await.unwrap();
        let app_clone = app.clone();

        tokio::spawn(async move {
            // Adapta o stream para compatibilidade com hyper
            let io = TokioIo::new(stream);

            // Cria um serviço para lidar com as conexões
            let service = service_fn(move |req| app_clone.clone().oneshot(req));

            // Inicia a conexão com o HTTP/1
            if let Err(err) = http1::Builder::new().serve_connection(io, service).await {
                eprintln!("[ERROR] Falha ao lidar com a conexão: {:?}", err);
            }
        });
    }

    // Este código nunca será alcançado devido ao loop infinito
    // database_service.close().await;
}