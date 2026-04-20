mod transaction;
mod state;
mod block;
mod blockchain;
mod mempool;

use transaction::Transaction;
use blockchain::Blockchain;
use mempool::Mempool;

fn main() {
    let mut chain = Blockchain::new();
    let mut mempool = Mempool::new();

    // Initial state (from chain)
    let state = chain.get_current_state();

    // Create transactions
    let tx1 = Transaction::new("alice".into(), "bob".into(), 10, 0, 2);
    let tx2 = Transaction::new("alice".into(), "bob".into(), 5, 1, 5);

    // Add to mempool
    mempool.add_transaction(tx1, &state).unwrap();
    mempool.add_transaction(tx2, &state).unwrap();

    // Produce block from mempool
    chain.produce_block(&mut mempool, 10);

    // Validate chain
    match chain.validate_chain() {
        Ok(_) => println!("Chain is valid"),
        Err(e) => println!("Invalid chain: {}", e),
    }

    // Print final balances
    let final_state = chain.get_current_state();
    println!("Alice balance: {}", final_state.get_balance(&"alice".into()));
    println!("Bob balance: {}", final_state.get_balance(&"bob".into()));
}