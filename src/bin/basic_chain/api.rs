use crate::blockchain::Chain;
use askama::Template;
use axum::Json;
use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse},
};
use serde_json::{Map, Value};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    chain: Vec<Map<String, Value>>,
}

#[derive(Template)]
#[template(path = "chain.html")]
struct ChainTemplate {
    block: Map<String, Value>,
}

pub async fn get_chain(State(shared_chain): State<Arc<Mutex<Chain>>>) -> impl IntoResponse {
    let chain = shared_chain.lock().await;
    let chain = chain.get_chain();
    let template = IndexTemplate { chain };
    Html(template.render().unwrap())
}

pub async fn add_block(
    State(shared_chain): State<Arc<Mutex<Chain>>>,
    data: Json<Map<String, Value>>,
) -> impl IntoResponse {
    let mut chain = shared_chain.lock().await;
    let data = data.get("data").unwrap().as_str().unwrap();
    chain.create_block(data.to_string());
    let template = ChainTemplate {
        block: chain.get_chain().last().unwrap().clone(),
    };
    Html(template.render().unwrap())
}
