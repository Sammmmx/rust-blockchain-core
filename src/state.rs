use std::collections::HashMap;
use crate::transaction::Transaction;
use crate::block::merkle_root;

#[derive(Clone)]
pub struct State {
    pub balances: HashMap<String, u64>,
    pub nonces: HashMap<String, u64>,
}

impl State {
    pub fn new() -> Self {
        State {
            balances: HashMap::new(),
            nonces: HashMap::new(),
        }
    }

    pub fn get_balance(&self, address: &String) -> u64 {
        *self.balances.get(address).unwrap_or(&0)
    }

    pub fn get_nonce(&self, address: &String) -> u64 {
        *self.nonces.get(address).unwrap_or(&0u64)
    }

    pub fn apply_transaction(&mut self, tx: &Transaction) -> bool {
        let sender = format!("{:?}", tx.sender);
        let receiver = format!("{:?}", tx.receiver);

        let sender_balance = self.get_balance(&sender);
        let sender_nonce = self.get_nonce(&sender);
        let receiver_balance = self.get_balance(&receiver);

        if sender_balance < tx.amount {
            println!("Insufficient funds");
            return false;
        }

        if sender_nonce != tx.nonce {
            println!("Incorrect nonce");
            return false;
        }

        self.balances.insert(sender.clone(), sender_balance - tx.amount);
        self.balances.insert(receiver.clone(), receiver_balance + tx.amount);
        self.nonces.insert(sender, sender_nonce + 1);

        true
    }

    /// Returns a Merkle root over all (address, balance, nonce) entries.
    /// Committing this into every block makes any balance manipulation detectable.
    pub fn root(&self) -> String {
        let mut entries: Vec<String> = self.balances
            .iter()
            .map(|(k, v)| {
                let nonce = self.nonces.get(k).unwrap_or(&0);
                format!("{}:{}:{}", k, v, nonce)
            })
            .collect();

        if entries.is_empty() {
            return String::from("0");
        }

        entries.sort();
        merkle_root(&entries)
    }
}