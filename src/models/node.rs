use crate::models::{Block, Blockchain};

pub struct Node {
    pub id: String,
    pub blockchain: Blockchain,
    pub peers: Vec<String>,
}

impl Node {
    pub fn new(id: String, difficulty: usize) -> Self {
        Node {
            id,
            blockchain: Blockchain::new(difficulty),
            peers: Vec::new(),
        }
    }

    pub fn add_peer(&mut self, peer_id: String) {
        if !self.peers.contains(&peer_id) && self.id != peer_id {
            self.peers.push(peer_id);
        }
    }

    pub fn receive_block(&mut self, block: Block) -> Result<(), NodeError> {
        let latest = self.blockchain.blocks.last().unwrap();

        if block.previous_hash == latest.hash && block.index == latest.index + 1 {
            if block.hash[..self.blockchain.difficulty].iter().all(|&b| b == 0) {
                self.blockchain.blocks.push(block);
                return Ok(());
            }
        }


        Err(NodeError::InvalidBlockHash)
    }

    pub fn receive_chain(&mut self, chain: Vec<Block>) -> bool {
        self.blockchain.replace_chain(chain)
    }

    pub fn mine_block(&mut self) -> Result<Block, super::blockchain::BlockchainError> {
        self.blockchain.mine_block()
    }
}

#[derive(Debug)]
pub enum NodeError {
    InvalidBlockHash,
    NodeNotFound,
    InvalidTransaction,
}