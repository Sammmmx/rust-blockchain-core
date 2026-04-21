mod block;
mod blockchain;
mod transaction;
mod mempool;
mod state;
mod node;

use ed25519_dalek::SigningKey;
use rand::rngs::OsRng;
use transaction::Transaction;
use blockchain::Blockchain;
use node::Node;

fn generate_keys() -> (SigningKey, ed25519_dalek::VerifyingKey) {
    let signing_key = SigningKey::generate(&mut OsRng);
    let verifying_key = signing_key.verifying_key();
    (signing_key, verifying_key)
}

fn main() {
    let (signing_key1, verifying_key1) = generate_keys();
    let (signing_key2, verifying_key2) = generate_keys();

    let blockchain1 = Blockchain::new(&signing_key1, &verifying_key1, 2);
    let blockchain2 = Blockchain::new(&signing_key2, &verifying_key2, 2);

    let mut node_a = Node::new(blockchain1);
    let mut node_b = Node::new(blockchain2);

    node_a.add_peer("Node B".to_string());
    node_b.add_peer("Node A".to_string());

    // Create transactions
    let tx = Transaction::new(verifying_key1, verifying_key2, 10, 0, &signing_key1);
    let tx1 = Transaction::new(verifying_key2, verifying_key1, 5, 0, &signing_key2);
    let tx2 = Transaction::new(verifying_key1, verifying_key2, 15, 1, &signing_key1);

    // Submit to mempools
    node_a.blockchain.submit_transaction(tx);
    node_b.blockchain.submit_transaction(tx1);

    // Broadcast pending transactions
    let tx_ref = node_a.blockchain.mempool.transactions.front().unwrap();
    node_a.broadcast_transaction(tx_ref);

    let tx1_ref = node_b.blockchain.mempool.transactions.front().unwrap();
    node_b.broadcast_transaction(tx1_ref);

    // Mine pending transactions
    node_a.blockchain.mine_pending(&signing_key1, &verifying_key1);
    node_b.blockchain.mine_pending(&signing_key2, &verifying_key2);

    // Submit and mine another transaction on node A
    node_a.blockchain.submit_transaction(tx2);
    node_a.blockchain.mine_pending(&signing_key1, &verifying_key1);

    // Node B adopts Node A's chain if it has higher cumulative difficulty
    let new_chain = node_a.blockchain.chain.clone();
    node_b.blockchain.replace_chain(new_chain);

    // Broadcast latest block
    let block = node_a.blockchain.chain.last().unwrap();
    node_a.broadcast_block(block);

    println!("\nNode A chain length: {}", node_a.blockchain.chain.len());
    println!("Node B chain length: {}", node_b.blockchain.chain.len());
    println!("Chain valid: {}", Blockchain::is_valid_chain(
        &node_a.blockchain.chain,
        &node_a.blockchain.state,
    ));
}