mod transaction;
mod state;
mod block;
mod blockchain;
mod mempool;

use mempool::Mempool;
use transaction::Transaction;
use blockchain::Blockchain;

fn main() {
    let mut chain = Blockchain::new();
    let mut mempool = Mempool::new();

    // Initial state assumption
    let state = chain.get_current_state(); // you may need to implement this

    let tx1 = Transaction::new("alice".into(), "bob".into(), 10, 0);
    let tx2 = Transaction::new("alice".into(), "bob".into(), 5, 1);

    mempool.add_transaction(tx1, &state).unwrap();
    mempool.add_transaction(tx2, &state).unwrap();

    let txs = mempool.get_transactions(10);

    chain.add_block(txs);

    println!("Done");
}