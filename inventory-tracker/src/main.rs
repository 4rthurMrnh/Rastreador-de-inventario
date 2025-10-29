mod database;
mod inventory;
mod models;
mod api;
mod prediction;

use axum::{
    routing::{get, post},
    Router,
};
use crate::database::Database;
use crate::inventory::InventoryService;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Carrega variáveis de ambiente do arquivo .env
    dotenvy::dotenv().ok();

    // Configuração de Logging
    tracing_subscriber::fmt::init();

    // Configuração da conexão DB
    let db_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL não definida");
    let db_pool = database::establish_connection().await?;

    // Inicialização das dependências
    let db_wrapper = Database::new(db_pool);
    let inventory_service = InventoryService::new(db_wrapper);

    // Configuração das Rotas (Endpoints)
    let app = Router::new()
        .route("/vendas", post(inventory_handlers::realizar_venda_handler))
        .route("/previsao/:item_id", get(inventory_handlers::previsao_handler))
        .with_state(inventory_service) // Injeção de Estado
        .layer(TraceLayer::new_for_http()); // Aplicação do Middleware de Logging

    // Inicialização do Servidor
    let addr = std::env::var("SERVER_ADDR")
        .unwrap_or_else(|_| "127.0.0.1:3000".to_string());
    tracing::info!("Iniciando o servidor em {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}