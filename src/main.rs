use chrono::{DateTime, Utc};
use hex;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

trait BlockChain {
    fn create_block(&mut self, data: String);
    fn validate_block(&self, block: &mut Block) -> (String, Block);
    fn get_last_block(&self) -> (String, Block);
    fn verify_chain(&self) -> bool;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Block {
    index: u64,
    nonce: Option<u64>,
    timestamp: DateTime<Utc>,
    data: String,
    prev_hash: String,
}

struct Chain {
    proof: u8,
    chain: Vec<(String, Block)>,
}

impl Block {
    fn new(index: u64, data: String, prev_hash: String) -> Block {
        Block {
            index,
            nonce: None,
            timestamp: Utc::now(),
            data,
            prev_hash,
        }
    }

    fn hash(&self) -> String {
        let mut hasher = Sha256::new();
        serde_json::to_string(&self)
            .unwrap()
            .bytes()
            .for_each(|byte| hasher.update(&[byte]));
        hex::encode(hasher.finalize())
    }
}

impl BlockChain for Chain {
    fn create_block(&mut self, data: String) {
        let (pre_hash, last_block) = self.get_last_block();
        let mut new_block = Block::new(last_block.index + 1, data, pre_hash);
        let validated_block = self.validate_block(&mut new_block);
        self.chain.push(validated_block)
    }

    fn validate_block(&self, block: &mut Block) -> (String, Block) {
        let mut nonce = 1u64;
        loop {
            block.nonce = Some(nonce);
            let hash = block.hash();
            if self.proof_of_work(&hash) {
                return (hash, block.clone());
            }
            nonce += 1;
        }
    }

    fn get_last_block(&self) -> (String, Block) {
        match self.chain.last() {
            Some((hash, block)) => (hash.clone(), block.clone()),
            None => panic!("No block in the chain"),
        }
    }

    fn verify_chain(&self) -> bool {
        let mut prev_hash: Option<String> = None;
        for (hash, block) in self.chain.iter() {
            if !self.proof_of_work(&hash) {
                return false;
            }
            if let Some(prev) = prev_hash {
                if prev != block.prev_hash {
                    return false;
                }
            }
            prev_hash = Some(hash.clone());
        }
        true
    }
}

impl Chain {
    pub fn new(proof: u8) -> Self {
        let mut genesis_block = Block {
            index: 1,
            nonce: None,
            timestamp: Utc::now(),
            data: "INIZIO BELLO!!!".to_string(),
            prev_hash: hex::encode(b"INIZIO BELLO"),
        };
        let mut blockchain = Self {
            chain: vec![],
            proof,
        };
        let genesis_block = blockchain.validate_block(&mut genesis_block);
        blockchain.chain.push(genesis_block);
        blockchain
    }

    fn proof_of_work(&self, hash: &str) -> bool {
        hash.starts_with(&"0".repeat(self.proof as usize))
    }
}

fn main() {
    let mut blockchain = Chain::new(2);
    blockchain.create_block("Ciao".to_string());
    blockchain.create_block("Mondo".to_string());
    blockchain.create_block("Blockchain".to_string());
    blockchain.create_block("Rust".to_string());
    blockchain.create_block("Ciao".to_string());
    println!("{:?}", blockchain.verify_chain());
    println!(
        "{}",
        serde_json::to_string_pretty(&blockchain.chain).unwrap()
    );
}
