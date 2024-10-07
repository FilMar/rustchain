use axum::{routing, serve, Router};
use cripto_protocol::CriptoCurrency;
use tokio::net::TcpListener;

mod api;
#[path = "../basic_chain/mod.rs"]
mod basic_chain;
mod cripto_protocol;

#[tokio::main]
async fn main() {
    let name = "0.0.0.0:3000";
    let bitcoin = CriptoCurrency::new(name.to_string(), 4, Vec::new());
    start_server(bitcoin, name).await;
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
        .route(
            "/ntn/add-node",
            routing::post(api::add_external_transaction),
        )
        .with_state(bitcoin);
    let listener = TcpListener::bind(name).await.unwrap();
    serve(listener, router).await.unwrap();
}

#[cfg(test)]
mod tests {
    use core::panic;
    use serde_json::{Map, Value};
    use super::*;
    use reqwest::Client;
    use futures::executor;

    async fn prepare_chain() -> (CriptoCurrency, Client) {
        let name= "0.0.0.0:3000";
        let bitcoin = CriptoCurrency::new(name.to_string(), 3, Vec::new());
        bitcoin.add_transaction("test".to_string(), 32.23, 0.02).await;
        bitcoin.add_transaction("test1".to_string(), 32.23, 0.02).await;
        bitcoin.add_transaction("test2".to_string(), 32.23, 0.02).await;
        let client = Client::new();
        let bitcoin_clone = bitcoin.clone();
        tokio::spawn(async move {
            start_server(bitcoin_clone, name).await;
        });
        (bitcoin, client)
    }
    #[test]
    fn test_get_chain() {
    let  (chain, client) = executor::block_on(prepare_chain());
    let jsonchain = executor::block_on(chain.get_chain());
        let result = match executor::block_on(client.get("0.0.0.0:3000").send()) {
            Ok(res) => match executor::block_on(res.json::<Vec<Map<String, Value>>>()) {
                Ok(res) => res,
                _ => panic!("decoding fallito"),

            },
        _ => panic!("chiamata fallita"),

    };
    assert_eq!(jsonchain, result);
    }

}
