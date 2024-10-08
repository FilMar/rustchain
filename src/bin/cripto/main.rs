use axum::{routing, serve, Router};
use cripto_protocol::CriptoCurrency;
use std::env;
use tokio::net::TcpListener;

mod api;
#[path = "../basic_chain/mod.rs"]
mod basic_chain;
mod cripto_protocol;

#[tokio::main]
async fn main() {
    let (name, proof, nodes) = match (env::var("NAME"), env::var("PROVE"), env::var("NODES")) {
        (Ok(name), Ok(proof), Ok(nodes)) => (
            format!("{name}:3000"),
            proof.parse::<u8>().unwrap_or(4),
            nodes
                .split(' ')
                .into_iter()
                .map(|n| format!("{n}:3000"))
                .collect(),
        ),
        _ => ("0.0.0.0:3000".to_string(), 4, Vec::new()),
    };
    let bitcoin = CriptoCurrency::new(name.to_string(), proof, nodes);
    start_server(bitcoin, &name).await;
}

async fn start_server(bitcoin: CriptoCurrency, name: &str) {
    let router = Router::new()
        .route("/", routing::get(api::get_chain))
        .route("/transaction", routing::post(api::create_transaction))
        .route("/ntn/add-blocks", routing::post(api::add_external_blocks))
        .route(
            "/ntn/add-transaction",
            routing::post(api::add_external_transaction),
        )
        .route("/ntn/add-node", routing::post(api::add_external_node))
        .with_state(bitcoin);
    let listener = TcpListener::bind(name).await.unwrap();
    serve(listener, router).await.unwrap();
}
