mod models;

use std::sync::Arc;
use std::time::Duration;

use tokio::time::sleep;

use crate::models::Network;
use crate::models::Transaction;
use crate::models::network::DisplayAsync;

#[tokio::main]
async fn main() {
    println!("🚀 Starting Async Blockchain Network Simulation\n");

    let network = Arc::new(Network::new());
    let difficulty = 3;

    // Add 4 nodes
    println!("⚙️  Initializing nodes...");
    network.add_node("Node_A".to_string(), difficulty).await;
    network.add_node("Node_B".to_string(), difficulty).await;
    network.add_node("Node_C".to_string(), difficulty).await;
    network.add_node("Node_D".to_string(), difficulty).await;

    // Connect nodes in mesh topology
    network.connect_nodes("Node_A", "Node_B").await;
    network.connect_nodes("Node_A", "Node_C").await;
    network.connect_nodes("Node_B", "Node_C").await;
    network.connect_nodes("Node_B", "Node_D").await;
    network.connect_nodes("Node_C", "Node_D").await;

    println!("✅ Network initialized with 4 nodes\n");

    // Add transactions
    println!("📝 Adding transactions...\n");
    match network.add_transaction_to_node(
        "Node_A",
        Transaction::new("Alice".to_string(), "Bob".to_string(), 10.0),
    ).await {
        Ok(()) => println!("✅ Transaction added to Node_A successfully"),
        Err(e) => println!("❌ Failed to add transaction to Node_A: {:?}", e),
    }
    match network.add_transaction_to_node(
        "Node_B",
        Transaction::new("Bob".to_string(), "Charlie".to_string(), 5.0),
    ).await {
        Ok(()) => println!("✅ Transaction added to Node_B successfully"),
        Err(e) => println!("❌ Failed to add transaction to Node_B: {:?}", e),
    }

    // Simulate concurrent mining with async tasks
    println!("⛏️  Starting concurrent mining simulation...\n");

    let net_a = Arc::clone(&network);
    let task_a = tokio::spawn(async move {
        sleep(Duration::from_millis(100)).await;
        let node_arc_clone = {
            let nodes = net_a.nodes.read().unwrap();
            let Some(node_arc) = nodes.get("Node_A") else { return };
            Arc::clone(&node_arc)
        };
        let block = tokio::task::spawn_blocking(move || {
            let mut node = node_arc_clone.lock().unwrap();
            node.mine_block()
        }).await.ok().and_then(|r| r.ok());
        if let Some(block) = block {
            println!("[Node_A] ✅ Mined block #{}", block.index);
            net_a.broadcast_block("Node_A", block).await;
        }
    });

    let net_b = Arc::clone(&network);
    let task_b = tokio::spawn(async move {
        sleep(Duration::from_millis(150)).await;
        let node_arc_clone = {
            let nodes = net_b.nodes.read().unwrap();
            let Some(node_arc) = nodes.get("Node_B") else { return };
            Arc::clone(&node_arc)
        };
        let block = tokio::task::spawn_blocking(move || {
            let mut node = node_arc_clone.lock().unwrap();
            node.mine_block()
        }).await.ok().and_then(|r| r.ok());
        if let Some(block) = block {
            println!("[Node_B] ✅ Mined block #{}", block.index);
            net_b.broadcast_block("Node_B", block).await;
        }
    });

    // Wait for mining tasks
    let _ = tokio::join!(task_a, task_b);

    sleep(Duration::from_millis(500)).await;
    println!("{}", network.fmt_async().await);

    // Add more transactions and mine
    println!("📝 Adding more transactions...\n");
    match network.add_transaction_to_node(
        "Node_C",
        Transaction::new("Charlie".to_string(), "Dave".to_string(), 15.0),
    ).await {
        Ok(()) => println!("✅ Transaction added to Node_C successfully"),
        Err(e) => println!("❌ Failed to add transaction to Node_C: {:?}", e),
    }

    println!("⛏️  Node C mining...\n");
    let node_arc_clone = {
        let nodes = network.nodes.read().unwrap();
        nodes.get("Node_C").map(|node_arc| Arc::clone(&node_arc))
    };
    let block = if let Some(node_arc_clone) = node_arc_clone {
        tokio::task::spawn_blocking(move || {
            let mut node = node_arc_clone.lock().unwrap();
            node.mine_block()
        }).await.ok().and_then(|r| r.ok())
    } else {
        None
    };
    if let Some(block) = block {
        println!("[Node_C] ✅ Mined block #{}", block.index);
        network.broadcast_block("Node_C", block).await;
    }

    sleep(Duration::from_millis(500)).await;
    println!("{}", network.fmt_async().await);

    // Demonstrate longest chain wins
    println!("🔄 Demonstrating longest-chain-wins...\n");
    network.broadcast_chain("Node_D").await;

    sleep(Duration::from_millis(200)).await;
    println!("{}", network.fmt_async().await);

    println!("✅ Simulation complete!");
}
