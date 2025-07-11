use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize, Clone)]
pub struct Transaction {
    pub sender: String,
    pub recipient: String,
    pub amount: u64,
}