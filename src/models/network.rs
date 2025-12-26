use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;
use async_trait::async_trait;

use crate::models::{Block, Node, Transaction, node::NodeError};

pub struct Network {
    pub nodes: Arc<Mutex<HashMap<String, Arc<Mutex<Node>>>>>,
}

impl Network {
    pub fn new() -> Self {
        Network {
            nodes: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn add_node(&self, node_id: String, difficulty: usize) {
        let node = Arc::new(Mutex::new(Node::new(node_id.clone(), difficulty)));
        self.nodes.lock().await.insert(node_id, node);
    }

    pub async fn connect_nodes(&self, node_id1: &str, node_id2: &str) {
        let nodes = self.nodes.lock().await;
        if let (Some(node1), Some(node2)) = (nodes.get(node_id1), nodes.get(node_id2)) {
            node1.lock().await.peers.push(node_id2.to_string());
            node2.lock().await.peers.push(node_id1.to_string());
        }
    }

    pub async fn broadcast_block(&self, node_id: &str, block: Block) {
        let nodes = self.nodes.lock().await;
        if let Some(node) = nodes.get(node_id) {
            let node = node.lock().await;
            for peer in &node.peers {
                if let Some(peer_node) = nodes.get(peer) {
                    let mut peer_node = peer_node.lock().await;
                    if peer_node.receive_block(block.clone()).is_ok(){
                        println!("Node {} accepted block #{} from {}", peer_node.id, block.index, node_id);
                    }
                }
            }
        }
    }

    pub async fn broadcast_chain(&self, node_id: &str) {
        let nodes = self.nodes.lock().await;
        if let Some(node) = nodes.get(node_id) {
            let node = node.lock().await;
            let chain = node.blockchain.blocks.clone();
            let peers = node.peers.clone();
            drop(node);

            for peer in &peers {
                if let Some(peer_node) = nodes.get(peer) {
                    let mut peer_node = peer_node.lock().await;
                    if peer_node.receive_chain(chain.clone()) {
                        println!("Node {} accepted chain from {} (length: {})", peer_node.id, node_id, chain.len());
                    }
                }
            }
        }
    }

    pub async fn add_transaction_to_node(&self, node_id: &str, transaction: Transaction) -> Result<(), NodeError> {
        let nodes = self.nodes.lock().await;
        if let Some(node) = nodes.get(node_id) {
            let mut node = node.lock().await;
            return node.blockchain.add_transaction(transaction).map_err(|_| NodeError::InvalidTransaction);
        } else {
            Err(NodeError::NodeNotFound)
        }
    }
}

#[async_trait]
pub trait DisplayAsync {
    async fn fmt_async(&self) -> String;
}

#[async_trait]
impl DisplayAsync for Network{
    async fn fmt_async(&self) -> String {
        let mut output = String::new();
        let nodes = self.nodes.lock().await;
        output.push_str("\n========== NETWORK STATUS ==========");
        for (node_id, node_arc) in nodes.iter() {
            let node = node_arc.lock().await;
            output.push_str(&format!("\nNode: {}", node_id));
            output.push_str(&format!("\nChain length: {}", node.blockchain.blocks.len()));
            output.push_str(&format!("\nLatest block hash: {:?}", node.blockchain.blocks.last().unwrap().hash));
            output.push_str(&format!("\nPending transactions: {}", node.blockchain.pending_transactions.len()));
            output.push_str(&format!("\nPeers: {:?}", node.peers));
        }
        output.push_str("\n====================================\n");
        output
    }
}