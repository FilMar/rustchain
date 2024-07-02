use crate::blockchain::Chain;
use axum::{extract::Json, extract::State};
use serde_json::{json, Map, Value};
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn get_chain(State(shared_chain): State<Arc<Mutex<Chain>>>) -> Json<Value> {
    let chain = shared_chain.lock().await;
    let chain = chain.get_chain();
    Json(json!({ "chain": chain }))
}

pub async fn add_block(
    State(shared_chain): State<Arc<Mutex<Chain>>>,
    data: Json<Map<String, Value>>,
) {
    let mut chain = shared_chain.lock().await;
    let data = data.get("data").unwrap().as_str().unwrap();
    chain.create_block(data.to_string());
}