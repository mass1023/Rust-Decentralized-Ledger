use chrono::Utc;
use sha2::{Digest, Sha256};

use crate::models::Transaction;

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub index: u64,
    // pub data: String,
    pub previous_hash: [u8; 32],
    pub hash: [u8; 32],
    pub nonce: u64,
    pub transactions: Vec<Transaction>,
    pub timestamp: u64,
}

impl Block {
    pub fn new(index: u64, previous_hash: [u8; 32], transactions: Vec<Transaction>) -> Self {
        Block {
            index,
            previous_hash,
            hash: [0u8; 32],
            nonce: 0,
            transactions,
            timestamp: Utc::now().timestamp() as u64,
        }
    }

    pub fn hash(&self) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(&self.previous_hash);
        hasher.update(self.nonce.to_string().as_bytes());
        hasher.update(self.timestamp.to_string().as_bytes());
        for transaction in &self.transactions {
            let tx = format!(
                "{} -> {} : {} on {}",
                transaction.sender,
                transaction.receiver,
                transaction.amount,
                transaction.timestamp
            );
            hasher.update(tx.as_bytes());
            // // Alternatively, you can update each field separately:
            // hasher.update(transaction.sender.clone());
            // hasher.update(transaction.receiver.clone());
            // hasher.update(transaction.amount.to_string());
            // hasher.update(transaction.timestamp.to_string());
        }
        hasher.finalize().into()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_block() {
        let tx = Transaction::new("Alice".to_string(), "Bob".to_string(), 10.0);
        let transactions = vec![tx];
        let prev_hash = [b'p'; 32];
        let block = Block::new(1, prev_hash, transactions.clone());
        assert_eq!(block.index, 1);
        assert_eq!(block.previous_hash, prev_hash);
        assert_eq!(block.hash, [0u8; 32]);
        assert_eq!(block.nonce, 0);
        assert_eq!(block.transactions, transactions);
        // Check timestamp is set (approximately current time)
        let now = Utc::now().timestamp() as u64;
        assert!(block.timestamp <= now && block.timestamp >= now - 10); // within 10 seconds
    }

    #[test]
    fn test_hash() {
        let mut tx = Transaction::new("Alice".to_string(), "Bob".to_string(), 10.0);
        tx.timestamp = 1234567890;
        let transactions = vec![tx];
        let prev_hash = [b'p'; 32];
        let mut block = Block::new(1, prev_hash, transactions);
        block.nonce = 42;
        block.timestamp = 1609459200;
        let hash = block.hash();
        // Compute expected hash
        let mut hasher = Sha256::new();
        hasher.update(&prev_hash);
        hasher.update("42".as_bytes());
        hasher.update("1609459200".as_bytes());
        let tx_str = "Alice -> Bob : 10 on 1234567890";
        hasher.update(tx_str.as_bytes());
        let expected: [u8; 32] = hasher.finalize().into();
        assert_eq!(hash, expected);
    }
}