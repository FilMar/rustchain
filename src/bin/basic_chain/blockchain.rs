use chrono::{DateTime, Utc};
use hex;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Block {
    index: u64,
    nonce: Option<u32>,
    timestamp: DateTime<Utc>,
    data: String,
    prev_hash: String,
}

pub struct Chain {
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

impl Chain {
    pub fn new(proof: u8) -> Self {
        let mut blockchain = Self {
            chain: vec![],
            proof,
        };
        if std::path::Path::new("blocks").exists() {
            blockchain.reload_chain();
        } else {
            blockchain.genesis_block();
        }
        blockchain
    }

    pub fn get_chain(&self) -> Vec<Map<String, Value>> {
        self.chain
            .iter()
            .map(|block| serde_json::to_string(block).unwrap())
            .map(|block| serde_json::from_str::<Map<String, Value>>(&block).unwrap())
            .collect::<Vec<Map<String, Value>>>()
    }
    pub fn create_block(&mut self, data: String) {
        self.verify_chain();
        let last_block = self.get_last_block();
        let mut new_block = Block::new(last_block.index + 1, data, last_block.hash());
        let validated_block = self.validate_block(&mut new_block);
        self.chain.push(validated_block)
    }

    pub fn replace_chain(&mut self, chain: Vec<Map<String, Value>>) {
        let mut blocks = vec![];
        for block in chain {
            blocks.push(serde_json::from_value::<Block>(Value::Object(block)).unwrap());
        }
        blocks.sort_by(|a, b| a.index.cmp(&b.index));
        self.chain = blocks;
        self.verify_chain();
    }

    fn validate_block(&self, block: &mut Block) -> Block {
        let mut nonce = 1u32;
        loop {
            block.nonce = Some(nonce);
            let hash = block.hash();
            if self.proof_of_work(&hash) {
                // devo creare il file del blocco
                self.crete_block_file(&block);
                return block.clone();
            }
            nonce += 1;
            if block.timestamp.timestamp() < Utc::now().timestamp() {
                println!("Resetting timestamp {}", block.timestamp.timestamp());
                block.timestamp = Utc::now();
                nonce = 1u32;
            }
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

    fn drop_dead_blocks(&mut self, index: u64) {
        while let Some(block) = self.chain.last() {
            if block.index >= index {
                match std::fs::remove_file(format!("blocks/{}", block.hash())) {
                    Err(e) => println!("Error removing block file: {:?}", e),
                    _ => (),
                };
                println!("Dropping block {}", block.index);
                self.chain.pop();
            } else {
                break;
            }
        }
    }

    fn proof_of_work(&self, hash: &str) -> bool {
        hash.starts_with(&"0".repeat(self.proof as usize))
    }

    fn genesis_block(&mut self) {
        let mut genesis_block = Block {
            index: 1,
            nonce: None,
            timestamp: Utc::now(),
            data: "INIZIO BELLO!!!".to_string(),
            prev_hash: hex::encode(b"INIZIO BELLO"),
        };
        let genesis_block = self.validate_block(&mut genesis_block);
        self.chain.push(genesis_block);
    }

    fn crete_block_file(&self, block: &Block) {
        print!("save on file not implemented");
    }

    fn load_block_file(&self, hash: &str) -> Block {
        todo!();
    }

    fn reload_chain(&mut self) {
        todo!();
    }
}
