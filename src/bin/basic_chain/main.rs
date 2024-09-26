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

#[cfg(test)]
mod tests {
    use serde_json::Value;
    use super::*;

    #[test]
    fn test_add_blocks() {
        let mut chain = Chain::new(2);
        chain.create_block("comune".to_string());
        chain.create_block("comune2".to_string());
        let mut chain_2 = chain.clone();
        chain.create_block("mozzarella".to_string());
        chain.create_block("mozzarella2".to_string());
        let chain_json: Vec<_> = chain
            .get_chain()
            .iter()
            .filter(|x| match x.get("index") {
                Some(index) => match index {
                    Value::Number(a) if a.as_u64().unwrap_or(0) > 3 => true,
                    _ => false,
                },
                None => false,
            })
            .map(|x| x.clone())
            .collect();
        chain_2
            .add_external_blocks(chain_json)
            .expect("aggiunta avvenuta con successo");
        assert_eq!(chain.get_chain(), chain_2.get_chain())
    }

    #[test]
    fn test_add_wrong_blocks() {
        let mut chain = Chain::new(2);
        chain.create_block("comune".to_string());
        chain.create_block("comune2".to_string());
        let mut chain_2 = chain.clone();
        chain_2.create_block("mozzarellaasdfasf".to_string());
        chain.create_block("mozzarella".to_string());
        chain.create_block("mozzarella2".to_string());
        let chain_json: Vec<_> = chain
            .get_chain()
            .iter()
            .filter(|x| match x.get("index") {
                Some(index) => match index {
                    Value::Number(a) if a.as_u64().unwrap_or(0) > 3 => true,
                    _ => false,
                },
                None => false,
            })
            .map(|x| x.clone())
            .collect();
        let _ = chain_2.add_external_blocks(chain_json).expect_err("corrompe la catena");
    }
}
