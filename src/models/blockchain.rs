use chrono::Utc;
use std::sync::Arc;

use crate::models::{Block, Transaction};

#[derive(Debug, Clone, PartialEq)]
pub struct Blockchain {
    pub blocks: Arc<Vec<Block>>,
    pub pending_transactions: Vec<Transaction>,
    pub difficulty: usize,
}

impl Blockchain {
    pub fn new(difficulty: usize) -> Self {
        let mut genesis_block = Block {
            index: 0,
            previous_hash: [0u8; 32],
            hash: [0u8; 32],
            nonce: 0,
            transactions: vec![],
            timestamp: Utc::now().timestamp() as u64,
        };
        genesis_block.hash = genesis_block.hash();

        Blockchain {
            blocks: Arc::new(vec![genesis_block]),
            pending_transactions: vec![],
            difficulty,

        }
    }

    pub fn add_transaction(&mut self, transaction: Transaction) -> Result<(), BlockchainError> {
        let sender_balance = self.get_balance(&transaction.sender);

        if sender_balance < transaction.amount {
            return Err(BlockchainError::InsufficientBalance);
        }
        self.pending_transactions.push(transaction);
        Ok(())
    }

    pub fn get_balance(&self, address: &str) -> f64 {
        let mut balance = 100.0;

        for block in &*self.blocks {
            for transaction in &block.transactions {
                if transaction.sender == address {
                    balance -= transaction.amount;
                }
                if transaction.receiver == address {
                    balance += transaction.amount;
                }
            }
        }

        balance
    }

    pub fn mine_block(&mut self) -> Result<Block, BlockchainError> {
        if self.pending_transactions.is_empty() {
            return Err(BlockchainError::EmptyTransactions);
        }

        let last_block = self.blocks.last().unwrap();
        let mut new_block = Block::new(self.blocks.len() as u64, last_block.hash.clone(), self.pending_transactions.clone());

        // mine the block (proof of work)
        let mut hash = new_block.hash();
        while !hash.iter().take(self.difficulty).all(|&b| b == 0) {
            new_block.nonce += 1;
            hash = new_block.hash();
        }

        // adds block to chain
        new_block.hash = hash;
        let mut new_blocks = (*self.blocks).clone();
        new_blocks.push(new_block.clone());
        self.blocks = Arc::new(new_blocks);
        // clear pending transactions
        self.pending_transactions.clear();

        Ok(new_block)
    }

    pub fn validate_chain(&self) -> Result<(), BlockchainError> {
        for (i, current_block) in self.blocks.iter().enumerate().skip(1) {
            let previous_block = &self.blocks[i - 1];

            // Check if the current block's previous hash matches the previous block's hash
            if current_block.previous_hash != previous_block.hash {
                return Err(BlockchainError::PreviousHashDoesNotMatch);
            }

            // Recalculate the hash of the current block and compare
            if current_block.hash != current_block.hash() {
                return Err(BlockchainError::IncorrectProof);
            }
        }
        Ok(())
    }

    pub fn replace_chain(&mut self, new_chain: Arc<Vec<Block>>) -> bool {
        if new_chain.len() <= self.blocks.len() {
            return false;
        }

        let temp_blockchain = Blockchain {
            blocks: new_chain.clone(),
            pending_transactions: vec![],
            difficulty: self.difficulty,
        };
        if let Err(_) = temp_blockchain.validate_chain() {
            return false;
        }

        self.blocks = new_chain;
        true
    }
}


#[derive(Debug, PartialEq)]
pub enum BlockchainError {
    IncorrectProof,
    PreviousHashDoesNotMatch,
    EmptyTransactions,
    InsufficientBalance,

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_blockchain() {
        let bc = Blockchain::new(2);
        assert_eq!((*bc.blocks).len(), 1);
        assert_eq!((*bc.blocks)[0].index, 0);
        assert_eq!((*bc.blocks)[0].previous_hash, [0u8; 32]);
        assert_ne!((*bc.blocks)[0].hash, [0u8; 32]); // hash is computed
        assert_eq!((*bc.blocks)[0].nonce, 0);
        assert!((*bc.blocks)[0].transactions.is_empty());
        assert!(bc.pending_transactions.is_empty());
        assert_eq!(bc.difficulty, 2);
    }

    #[test]
    fn test_add_transaction() {
        let mut bc = Blockchain::new(1);
        let tx = Transaction::new("Alice".to_string(), "Bob".to_string(), 0.0); // amount 0 to bypass balance check
        let result = bc.add_transaction(tx.clone());
        assert!(result.is_ok());
        assert_eq!(bc.pending_transactions.len(), 1);
        assert_eq!(bc.pending_transactions[0], tx);
    }

    #[test]
    fn test_mine_block() {
        let mut bc = Blockchain::new(1); // low difficulty
        // Add a transaction to give Alice balance
        let tx_genesis = Transaction::new("Genesis".to_string(), "Alice".to_string(), 100.0);
        let mut genesis_block = Block::new(1, (*bc.blocks)[0].hash.clone(), vec![tx_genesis]);
        genesis_block.nonce = 1;
        genesis_block.hash = genesis_block.hash();
        let mut v = (*bc.blocks).clone();
        v.push(genesis_block);
        bc.blocks = Arc::new(v);
        // Now add transaction from Alice
        let tx = Transaction::new("Alice".to_string(), "Bob".to_string(), 10.0);
        let result = bc.add_transaction(tx.clone());
        assert!(result.is_ok());
        let result = bc.mine_block();
        assert!(result.is_ok());
        let mined_block = result.unwrap();
        assert_eq!((*bc.blocks).len(), 3);
        assert_eq!((*bc.blocks)[2], mined_block);
        assert!(bc.pending_transactions.is_empty());
        assert!(mined_block.hash.iter().take(bc.difficulty).all(|&b| b == 0));
    }

    #[test]
    fn test_validate_chain_valid() {
        let mut bc = Blockchain::new(1);
        // Add balance
        let tx_genesis = Transaction::new("Genesis".to_string(), "Alice".to_string(), 100.0);
        let mut genesis_block = Block::new(1, (*bc.blocks)[0].hash.clone(), vec![tx_genesis]);
        genesis_block.nonce = 1;
        genesis_block.hash = genesis_block.hash();
        let mut v = (*bc.blocks).clone();
        v.push(genesis_block);
        bc.blocks = Arc::new(v);
        // Add transaction
        let tx = Transaction::new("Alice".to_string(), "Bob".to_string(), 10.0);
        let result = bc.add_transaction(tx);
        assert!(result.is_ok());
        let result = bc.mine_block();
        assert!(result.is_ok());
        assert!(bc.validate_chain().is_ok());
    }

    #[test]
    fn test_validate_chain_invalid() {
        let mut bc = Blockchain::new(1);
        // Add balance
        let tx_genesis = Transaction::new("Genesis".to_string(), "Alice".to_string(), 100.0);
        let mut genesis_block = Block::new(1, (*bc.blocks)[0].hash.clone(), vec![tx_genesis]);
        genesis_block.nonce = 1;
        genesis_block.hash = genesis_block.hash();
        let mut v = (*bc.blocks).clone();
        v.push(genesis_block);
        bc.blocks = Arc::new(v);
        // Add transaction
        let tx = Transaction::new("Alice".to_string(), "Bob".to_string(), 10.0);
        let result = bc.add_transaction(tx);
        assert!(result.is_ok());
        let result = bc.mine_block();
        assert!(result.is_ok());
        // Tamper with hash
        let mut v = (*bc.blocks).clone();
        v[2].hash = [b't'; 32];
        bc.blocks = Arc::new(v);
        assert!(bc.validate_chain().is_err());
    }

    #[test]
    fn test_get_balance() {
        let mut bc = Blockchain::new(1);
        // Manually add a block with transactions
        let tx1 = Transaction::new("Alice".to_string(), "Bob".to_string(), 50.0);
        let tx2 = Transaction::new("Bob".to_string(), "Charlie".to_string(), 20.0);
        let mut block = Block::new(1, (*bc.blocks)[0].hash.clone(), vec![tx1, tx2]);
        block.nonce = 1;
        block.hash = block.hash(); // compute hash
        let mut v = (*bc.blocks).clone();
        v.push(block);
        bc.blocks = Arc::new(v);

        assert_eq!(bc.get_balance("Alice"), 50.0); // 100 - 50
        assert_eq!(bc.get_balance("Bob"), 130.0); // 100 + 50 - 20
        assert_eq!(bc.get_balance("Charlie"), 120.0); // 100 + 20
        assert_eq!(bc.get_balance("Dave"), 100.0); // 100 (no transactions)
    }
}