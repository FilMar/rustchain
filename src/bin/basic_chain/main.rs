use crate::blockchain::Chain;
use axum::{routing, Router};
use std::sync::Arc;
use tokio::sync::Mutex;

mod api;
mod blockchain;

#[tokio::main]
async fn main() {
    println!("blockchain serve on localhost:3000...");
    let shared_chain = Arc::new(Mutex::new(Chain::new(4)));
    let app = Router::new()
        .route("/", routing::get(api::get_chain))
        .route("/", routing::post(api::add_block))
        .with_state(shared_chain);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
