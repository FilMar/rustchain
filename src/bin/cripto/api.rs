use crate::cripto_protocol::CriptoCurrency;
use axum::{extract::State, http::status, response::IntoResponse, Json};
use serde::Deserialize;
use serde_json::{Map, Value};

type ArcChain = State<CriptoCurrency>;
// user to node

#[derive(Debug, Deserialize)]
pub struct CreateTransaction {
    to: String,
    amount: f32,
}

pub async fn create_transaction(
    State(mut criptochain): ArcChain,
    Json(data): Json<CreateTransaction>,
) -> impl IntoResponse {
    criptochain
        .add_transaction(data.to, data.amount, 0.05)
        .await;
    (status::StatusCode::CREATED, "ok")
}

pub async fn get_chain(State(criptochain): ArcChain) -> impl IntoResponse {
    let res = match serde_json::to_string(&criptochain.get_chain().await) {
        Ok(e) => e,
        Err(e) => return (status::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    };
    (status::StatusCode::OK, res)
}
// node to node
pub async fn add_external_blocks(
    State(mut criptochain): ArcChain,
    Json(data): Json<Vec<Map<String, Value>>>,
) -> impl IntoResponse {
    match criptochain.add_external_blocks(data).await {
        Ok(_) => (status::StatusCode::NO_CONTENT, "").into_response(),
        Err(e) => (status::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

#[derive(Debug, Deserialize)]
pub struct ExternalNode {
    url: String,
}

pub async fn add_external_node(
    State(criptochain): ArcChain,
    Json(data): Json<ExternalNode>,
) -> impl IntoResponse {
    criptochain.add_node(&data.url).await;
    (
        status::StatusCode::OK,
        format!("nodo {} aggiunto con successo", data.url),
    )
        .into_response()
}

#[derive(Debug, Deserialize)]
pub struct ExternalTransaction {
    sender: String,
    receiver: String,
    amount: f32,
}

// WARNING: per ora riceve una transazione per volta
pub async fn add_external_transaction(
    State(mut criptochain): ArcChain,
    Json(transaction): Json<ExternalTransaction>,
) -> impl IntoResponse {
    criptochain
        .add_external_transaction(
            transaction.sender,
            transaction.receiver,
            transaction.amount,
            0.05,
        )
        .await;
    (status::StatusCode::CREATED, "").into_response()
}
