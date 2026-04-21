use crate::block::Block;
use crate::blockchain::Blockchain;
use crate::transaction::Transaction;

pub struct Node {
    pub blockchain: Blockchain,
    pub peers: Vec<String>,
}

impl Node {
    pub fn new(blockchain: Blockchain) -> Self {
        Node {
            blockchain,
            peers: Vec::new(),
        }
    }

    pub fn add_peer(&mut self, peer: String) {
        self.peers.push(peer);
    }

    pub fn broadcast_transaction(&self, tx: &Transaction) {
        for peer in &self.peers {
            println!("Broadcasting tx to {}", peer);
        }
    }

    pub fn broadcast_block(&self, block: &Block) {
        for peer in &self.peers {
            println!("Broadcasting block {} to {}", block.index, peer);
        }
    }
}