use std::collections::HashMap;
use crate::transaction::Transaction;

#[derive(Clone, Debug)]
pub struct State {
    pub balances: HashMap<String, u64>,
    pub nonces: HashMap<String, u64>,
}

impl State {
    pub fn new() -> Self {
        Self {
            balances: HashMap::new(),
            nonces: HashMap::new(),
        }
    }

    pub fn get_balance(&self, addr: &String) -> u64 {
        *self.balances.get(addr).unwrap_or(&0)
    }

    pub fn get_nonce(&self, addr: &String) -> u64 {
        *self.nonces.get(addr).unwrap_or(&0)
    }

    pub fn apply_transaction(&mut self, tx: &Transaction) -> Result<(), String> {
        let balance = self.get_balance(&tx.from);
        let nonce = self.get_nonce(&tx.from);

        if balance < tx.amount {
            return Err("Insufficient balance".into());
        }

        if nonce != tx.nonce {
            return Err("Invalid nonce".into());
        }

        // Apply state transition
        self.balances.insert(tx.from.clone(), balance - tx.amount);
        *self.balances.entry(tx.to.clone()).or_insert(0) += tx.amount;
        self.nonces.insert(tx.from.clone(), nonce + 1);

        Ok(())
    }
}