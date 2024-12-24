use chrono::prelude::*;
use sha2::{Digest, Sha256};

#[derive(Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: String,
    pub transactions: Vec<String>,
    pub previous_hash: String,
    pub current_hash: String,
    pub validator: String,
}

impl Block {
    pub fn new(index: u64, transactions: Vec<String>, previous_hash: String, validator: String) -> Self {
        let timestamp = Utc::now().to_rfc3339();
        let mut hasher = Sha256::new();
        hasher.update(format!(
            "{}{:?}{}{}",
            index, transactions, previous_hash, validator
        ));
        let current_hash = format!("{:x}", hasher.finalize());

        Block {
            index,
            timestamp,
            transactions,
            previous_hash,
            current_hash,
            validator,
        }
    }
}
