use crate::blockchain::Chain;
use axum::http::status;
use axum::Json;
use axum::{extract::State, response::IntoResponse};
use serde_json::{Map, Value};
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn get_chain(State(shared_chain): State<Arc<Mutex<Chain>>>) -> impl IntoResponse {
    let chain = shared_chain.lock().await;
    match serde_json::to_string_pretty(&chain.get_chain()) {
        Ok(msg) => (status::StatusCode::OK, msg),
        Err(err) => (status::StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
    }
}

pub async fn add_block(
    State(shared_chain): State<Arc<Mutex<Chain>>>,
    data: Json<Map<String, Value>>,
) -> impl IntoResponse {
    let mut chain = shared_chain.lock().await;
    match data.get("data") {
        None => (
            status::StatusCode::NOT_FOUND,
            "il campo data e' obbligatorio",
        ),
        Some(Value::String(data)) => {
            chain.create_block(data.to_string());
            (status::StatusCode::CREATED, "blocco creato con successo")
        }
        _ => (status::StatusCode::NOT_FOUND, "campo data nel formato sbagliato, inviare una string")
    }
}
