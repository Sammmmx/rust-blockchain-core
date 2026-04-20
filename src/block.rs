use sha2::{Sha256, Digest};
use crate::transaction::Transaction;

#[derive(Clone, Debug)]
pub struct Block {
    pub index: u64,
    pub prev_hash: String,
    pub transactions: Vec<Transaction>,
    pub hash: String,
}

impl Block {
    pub fn new(index: u64, prev_hash: String, transactions: Vec<Transaction>) -> Self {
        let mut block = Self {
            index,
            prev_hash,
            transactions,
            hash: String::new(),
        };

        block.hash = block.calculate_hash();
        block
    }

    pub fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();

        let tx_hashes: String = self
            .transactions
            .iter()
            .map(|tx| tx.hash())
            .collect();

        hasher.update(format!(
            "{}{}{}",
            self.index,
            self.prev_hash,
            tx_hashes
        ));

        format!("{:x}", hasher.finalize())
    }
}