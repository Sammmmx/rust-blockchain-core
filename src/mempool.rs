use std::collections::HashMap;

use crate::state::State;
use crate::transaction::Transaction;

pub struct Mempool {
    // pending transactions grouped by sender
    pool: HashMap<String, Vec<Transaction>>,
}

impl Mempool {
    pub fn new() -> Self {
        Self {
            pool: HashMap::new(),
        }
    }

    // Validate + insert transaction
    pub fn add_transaction(
        &mut self,
        tx: Transaction,
        state: &State,
    ) -> Result<(), String> {
        let sender = tx.from.clone();

        // Get current state values
        let base_nonce = state.get_nonce(&sender);
        let base_balance = state.get_balance(&sender);

        // Get sender's pending txs
        let pending = self.pool.entry(sender.clone()).or_insert(vec![]);

        // Compute expected nonce after pending txs
        let expected_nonce = base_nonce + pending.len() as u64;

        if tx.nonce != expected_nonce {
            return Err(format!(
                "Invalid nonce. Expected {}, got {}",
                expected_nonce, tx.nonce
            ));
        }

        // Compute total pending spend
        let pending_spend: u64 = pending.iter().map(|t| t.amount).sum();

        if base_balance < pending_spend + tx.amount {
            return Err("Insufficient balance (including pending txs)".into());
        }

        pending.push(tx);
        Ok(())
    }

    // Get transactions for block (simple FIFO per sender)
    pub fn get_transactions(&mut self, max: usize) -> Vec<Transaction> {
        let mut selected = Vec::new();

        for txs in self.pool.values_mut() {
            while !txs.is_empty() && selected.len() < max {
                selected.push(txs.remove(0));
            }
        }

        selected.sort_by(|a, b| b.fee.cmp(&a.fee));
        selected
    }

    pub fn is_empty(&self) -> bool {
        self.pool.values().all(|v| v.is_empty())
    }
}