use crate::basic_chain::blockchain::Chain;
use serde::{Deserialize, Serialize};
use serde_json::{to_string_pretty, Map, Value};
use std::mem::drop;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct CriptoCurrency {
    name: String,
    mempool: Arc<Mutex<Vec<Transaction>>>,
    nodes: Arc<Mutex<Vec<String>>>,
    blockchain: Arc<Mutex<Chain>>,
}

impl CriptoCurrency {
    pub fn new(name: String, proof: u8, nodes: Vec<String>) -> Self {
        let mut nodes = nodes;
        nodes.retain(|node| node != &name);
        let crypto = Self {
            name,
            mempool: Arc::new(Mutex::new(Vec::new())),
            nodes: Arc::new(Mutex::new(nodes)),
            blockchain: Arc::new(Mutex::new(Chain::new(proof))),
        };
        crypto
    }

    pub async fn get_chain(&self) -> Vec<Map<String, Value>> {
        self.blockchain.lock().await.get_chain()
    }

    pub async fn add_transaction(
        &mut self,
        sender: String,
        receiver: String,
        amount: f32,
        fee: f32,
    ) {
        let transaction = Transaction {
            sender,
            receiver,
            amount,
            fee,
            timestamp: chrono::Utc::now(),
        };
        let mut mempool = self.mempool.lock().await;
        mempool.push(transaction);
        println!("Transaction added to mempool {}", mempool.len());
    }

    pub async fn start_mining(&mut self) {
        let chain = self.blockchain.clone();
        let mempool = self.mempool.clone();
        tokio::spawn(async move {
            loop {
                let mut temp_mempool = mempool.lock().await;
                match temp_mempool.len() {
                    0 => println!("waiting for transactions.."),
                    1..=5 => {
                        let data = temp_mempool.clone();
                        let mut chain = chain.lock().await;
                        chain.create_block(
                            serde_json::to_string::<Vec<Transaction>>(&data).unwrap(),
                        );
                        let chain = chain.get_chain();
                        print!("{:?}", chain);
                        temp_mempool.clear();
                    }
                    _ => {
                        let data = temp_mempool[0..5].to_vec();
                        let mut chain = chain.lock().await;
                        chain.create_block(
                            serde_json::to_string::<Vec<Transaction>>(&data).unwrap(),
                        );
                        temp_mempool.drain(0..5);
                        let chain = chain.get_chain();
                        print!("{}", to_string_pretty(&chain).unwrap());
                    }
                }
                drop(temp_mempool);
                tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
            }
        });
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Transaction {
    sender: String,
    receiver: String,
    amount: f32,
    fee: f32,
    timestamp: chrono::DateTime<chrono::Utc>,
}
