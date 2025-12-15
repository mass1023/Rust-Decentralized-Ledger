mod models;

use crate::models::Blockchain;
use crate::models::Transaction;


fn main() {
    let mut bc = Blockchain::new(1);
    println!("Genesis Block: {:?}", bc.blocks[0]);
 
    let tx = Transaction::new("Steve".to_string(), "Martin".to_string(), 10.0);
    println!("Adding transaction: Steve -> Martin : 10.0");
    if let Err(e) = bc.add_transaction(tx) {
        eprintln!("Failed to add transaction: {:?}", e);
        return;
    }

    if let Err(e) = bc.mine_block() {
        eprintln!("Failed to mine block: {:?}", e);
        return;
    }

    println!("Balance of Martin: {}", bc.get_balance("Martin"));
    println!("Balance of Steve: {}", bc.get_balance("Steve"));

    match bc.validate_chain() {
        Ok(_) => println!("Is chain valid? true"),
        Err(e) => println!("Is chain valid? false - Error: {:?}", e),
    }
    println!("/////////////////////////////////////");
    println!("Blockchain state: {:?}", bc);
}
