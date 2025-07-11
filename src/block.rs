use sha2::{Sha256, Digest};
use chrono::Utc;
use serde::Serialize;
use crate::transaction::{Transaction};

#[derive(Serialize, Clone)]
pub struct Block {
    pub index: usize,
    pub timestamp: u64,
    pub transactions: Vec<Transaction>,
    pub prev_hash: String,
    pub hash: String,
    pub nonce: u64,
}

impl Block {
    pub fn new(transactions: Vec<Transaction>, prev_block: &Block, difficulty: usize) -> Self {
        let index = prev_block.index + 1;
        let timestamp = Utc::now().timestamp() as u64;
        let prev_hash = prev_block.hash.clone();

        let tx_data = serde_json::to_string(&transactions).unwrap();

        let (nonce, hash) = Block::mine(index, timestamp, &tx_data, &prev_hash, difficulty);

        Block {
            index,
            timestamp,
            transactions,
            prev_hash,
            hash,
            nonce,
        }
    }

    pub fn mine(index: usize, timestamp: u64, tx_data: &str, prev_hash: &str, difficulty: usize) -> (u64, String) {
        let prefix = "0".repeat(difficulty);
        let mut nonce = 0;

        loop {
            let content = format!("{}{}{}{}{}", index, timestamp, tx_data, prev_hash, nonce);
            let hash = format!("{:x}", sha2::Sha256::digest(content.as_bytes()));

            if hash.starts_with(&prefix) {
                return (nonce, hash);
            }

            nonce += 1;
        }


    }
}