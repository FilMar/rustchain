use axum::{routing, serve, Router};
use cripto_protocol::CriptoCurrency;
use tokio::net::TcpListener;

#[path = "../basic_chain/mod.rs"]
mod basic_chain;
mod cripto_protocol;
mod api;

#[tokio::main]
async fn main() {
    let bitcoin = CriptoCurrency::new("filippo".to_string(), 4, Vec::new());
    let mut bitcoin_clone = bitcoin.clone();
    tokio::spawn(async move {
        bitcoin_clone.start_mining().await;
    });
    let router = Router::new()
        .route("/", routing::get(api::get_chain))
        .route("/transaction", routing::post(api::create_transaction))
        .route("/ntn/add-blocks", routing::post(api::add_external_blocks))
        .route("/ntn/add-transaction", routing::post(api::add_external_transaction))
        .route("/ntn/addnode", routing::post(api::add_external_transaction))
        .with_state(bitcoin);
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    serve(listener, router).await.unwrap();
}


#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::Client;

    fn prepare_chain() {
        let bitcoin = CriptoCurrency::new("test_fil".to_string(), 3, Vec::new());
        let mut bitclone = bitcoin.clone();
        tokio::spawn(async move {
            bitclone
        })


    }



}
