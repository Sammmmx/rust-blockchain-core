mod transaction;
mod state;
mod block;
mod blockchain;

use transaction::Transaction;
use blockchain::Blockchain;

fn main() {
    let mut chain = Blockchain::new();

    let tx1 = Transaction::new("alice".into(), "bob".into(), 10, 0);
    let tx2 = Transaction::new("alice".into(), "bob".into(), 5, 1);

    chain.add_block(vec![tx1, tx2]);

    match chain.validate_chain() {
        Ok(_) => println!("Chain is valid"),
        Err(e) => println!("Invalid chain: {}", e),
    }
}