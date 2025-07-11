use crate::block::Block;
use sha2::{Sha256, Digest};
use serde::Serialize;
use crate::transaction::Transaction;

#[derive(Serialize, Clone)]
pub struct Blockchain { 
    pub chain: Vec<Block>,
}

impl Blockchain {
    pub fn default() -> Self {
        Self::new()
    }

    pub fn new() -> Self {
        let genesis_block = Block {
            index: 0,
            timestamp: chrono::Utc::now().timestamp() as u64,
            transactions: vec![Transaction {
                sender: "genesis".to_string(),
                recipient: "genesis".to_string(),
                amount: 0,
            }],
            prev_hash: "0".to_string(),
            hash: String::new(),
            nonce: 0,
        };

        Blockchain {
            chain: vec![genesis_block],
        }
    }

    pub fn add_block(&mut self, transactions: Vec<Transaction>) {
        let difficulty = 2;
        let prev_block = self.chain.last().unwrap();
        let new_block = Block::new(transactions, prev_block, difficulty);

        self.chain.push(new_block);
    }

    pub fn is_valid(&mut self) -> bool {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];

            // 1. Check chaining
            if current.prev_hash != previous.hash {
                return false;
            }

            // 2. Herbereken de hash van het huidige blok
            let tx_data = serde_json::to_string(&current.transactions).unwrap();
            let content = format!(
                "{}{}{}{}{}",
                current.index,
                current.timestamp,
                tx_data,
                current.prev_hash,
                current.nonce
            );
            let recalculated_hash = format!("{:x}", Sha256::digest(content.as_bytes()));

            if current.hash != recalculated_hash {
                return false;
            }
        }

    true
    }
}