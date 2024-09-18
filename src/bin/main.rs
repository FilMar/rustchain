use std::sync::Arc;

use axum::{extract::State, http::status, response::IntoResponse, routing, serve, Json, Router};
use serde_json::{Map, Value};
use tokio::{net::TcpListener, sync::Mutex};
use crate::cripto_protocol::CriptoCurrency;

mod cripto_protocol;
mod basic_chain;


#[tokio::main]
async fn main() {
    let bitcoin = CriptoCurrency::new("filippo".to_string(), 4, Vec::new());
    let mut bitcoin_clone = bitcoin.clone();
    tokio::spawn(async move {
        bitcoin_clone.start_mining().await;
    });
    let router = Router::new()
        .route("/", routing::get(get_chain))
        .route("/transaction", routing::post(create_transaction))
        .with_state(bitcoin);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    serve(listener, router).await.unwrap();


}


type ArcChain = State<CriptoCurrency>;
// user to node
async fn create_transaction(State(mut criptochain): ArcChain , Json(data): Json<Map<String, Value>>) -> impl IntoResponse {
    let sender = match data.get("from") {
        Some(Value::String(a)) => a,
        _ => return (status::StatusCode::BAD_REQUEST, "serve chi paga"),
    }.to_string();
    let receiver = match data.get("to"){
        Some(Value::String(a)) => a,
        _ => return (status::StatusCode::BAD_REQUEST, "serve chi riceve il pagamento"),
    }.to_string();
    let amount = match data.get("amount"){
        Some(Value::String(a)) => a,
        _ => return (status::StatusCode::BAD_REQUEST, "serve quanto paga"),
    }.parse().unwrap();
    criptochain.add_transaction(sender, receiver, amount, 0.05).await;
    (status::StatusCode::CREATED, "ok")
}

async fn get_chain(State(criptochain): ArcChain) -> impl IntoResponse {
    let res = match serde_json::to_string(&criptochain.get_chain().await) {
        Ok(e) => e,
        Err(e) => return (status::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    };
    (status::StatusCode::OK,
     res)
}
// node to node
async fn recive_new_block(State(criptochain): State<Arc<Mutex<CriptoCurrency>>>) -> impl IntoResponse {}
async fn send_mempool(State(criptochain): State<Arc<Mutex<CriptoCurrency>>>) -> impl IntoResponse {}
async fn add_new_node(State(criptochain): State<Arc<Mutex<CriptoCurrency>>>) -> impl IntoResponse {}
