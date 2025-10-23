use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use crate::inventory::InventoryService;

// Handler para Venda/Update (POST)
pub async fn realizar_venda_handler(
    State(service): State<InventoryService>,
    Json(payload): Json<(i32, i32)>, // (item_id, quantidade_vendida)
) -> impl IntoResponse {
    let (item_id, quantidade_vendida) = payload; // payload é (i32, i32)

    // Chama o serviço. O serviço vai retornar Result<(), String>
    match service.realizar_venda(item_id, quantidade_vendida).await {
        Ok(produto_atualizado) => (StatusCode::OK, Json(produto_atualizado)), // Retorna o produto atualizado
        Err(e) => (StatusCode::BAD_REQUEST, Json(e)), // Retorna o erro
    }
}

pub async fn previsao_handler(
    State(service): State<InventoryService>,
    Path(item_id): Path<i32>,
) -> Response {
    // 1. Buscar histórico de vendas para o item
    let historico = match service.buscar_historico_vendas(item_id).await {
        Ok(h) => h,
        Err(e) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(format!("Erro ao buscar histórico: {}", e))).into_response();
        }
    };
    
    // 2. Chamar o serviço de previsão com o histórico
    match service.prever_demanda(&historico).await {
        Ok(quantidade_prevista) => {
            if quantidade_prevista > 0.0 {
                (StatusCode::OK, Json(format!("Previsão de demanda: {:.0} unidades", quantidade_prevista))).into_response()
            } else {
                // Se a previsão for 0 ou negativa, não há necessidade de comprar.
                // NO_CONTENT é um bom status para isso, e geralmente não tem corpo.
                (StatusCode::NO_CONTENT).into_response()
            }
        }
        Err(e) => (StatusCode::BAD_REQUEST, Json(e)).into_response(), // Retorna 400 se houver erro lógico (ex: dados insuficientes)
    }
}