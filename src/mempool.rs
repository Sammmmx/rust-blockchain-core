use std::collections::VecDeque;
use crate::transaction::Transaction;

pub struct Mempool {
    pub transactions: VecDeque<Transaction>,
}

impl Mempool {
    pub fn new() -> Self {
        Mempool {
            transactions: VecDeque::new(),
        }
    }

    pub fn add_transaction(&mut self, tx: Transaction) {
        if tx.verify() {
            self.transactions.push_back(tx);
        } else {
            println!("Transaction verification failed");
        }
    }

    pub fn get_transactions(&mut self, max: usize) -> Vec<Transaction> {
        let mut selected = Vec::new();
        for _ in 0..max {
            if let Some(tx) = self.transactions.pop_front() {
                selected.push(tx);
            } else {
                break;
            }
        }
        selected
    }
}