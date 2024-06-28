use chrono::{DateTime, Utc};
use hex;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

trait BlockChain {
    fn create_block(&mut self, data: String);
    fn validate_block(&self, block: &mut Block) -> Block;
    fn get_last_block(&self) -> Block;
    fn verify_chain(&mut self) -> bool;
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
    chain: Vec<Block>,
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
        let last_block = self.get_last_block();
        let mut new_block = Block::new(last_block.index + 1, data, last_block.hash());
        let validated_block = self.validate_block(&mut new_block);
        self.chain.push(validated_block)
    }

    fn validate_block(&self, block: &mut Block) -> Block {
        let mut nonce = 1u64;
        loop {
            block.nonce = Some(nonce);
            let hash = block.hash();
            if self.proof_of_work(&hash) {
                return block.clone();
            }
            nonce += 1;
        }
    }

    fn get_last_block(&self) -> Block {
        match self.chain.last() {
            Some(block) => block.clone(),
            None => panic!("No block in the chain"),
        }
    }

    fn verify_chain(&mut self) -> bool {
        let mut prev_hash: Option<String> = None;
        for block in self.chain.iter() {
            if !self.proof_of_work(&block.hash()) {
                self.drop_dead_blocks(block.index);
                return false;
            }
            if let Some(prev) = prev_hash {
                if prev != block.prev_hash {
                    self.drop_dead_blocks(block.index);
                    return false;
                }
            }
            prev_hash = Some(block.hash());
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

    fn drop_dead_blocks(&mut self, index: u64) {
        while let Some(block) = self.chain.last() {
            if block.index >= index {
                self.chain.pop();
            } else {
                break;
            }
        }
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
    blockchain.chain[4].data = "Ciao Mondo".to_string();
    println!("{:?}", blockchain.verify_chain());
    println!(
        "{}",
        serde_json::to_string_pretty(&blockchain.chain).unwrap()
    );
}
