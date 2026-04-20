use crate::block::Block;
use crate::state::State;
use crate::transaction::Transaction;
use crate::mempool::Mempool;

pub struct Blockchain {
    pub chain: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        let genesis = Block {
            index: 0,
            prev_hash: "0".into(),
            transactions: vec![],
            hash: "genesis".into(),
        };

        Self {
            chain: vec![genesis],
        }
    }

    pub fn add_block(&mut self, transactions: Vec<Transaction>) {
        let prev = self.chain.last().unwrap();

        let block = Block::new(
            prev.index + 1,
            prev.hash.clone(),
            transactions,
        );

        self.chain.push(block);
    }

    pub fn validate_chain(&self) -> Result<(), String> {
        let mut state = State::new();

        // Initial allocation (genesis effect)
        state.balances.insert("alice".into(), 100);

        for i in 1..self.chain.len() {
            let prev = &self.chain[i - 1];
            let current = &self.chain[i];

            // Check linkage
            if current.prev_hash != prev.hash {
                return Err("Invalid previous hash".into());
            }

            // Re-execute transactions
            for tx in &current.transactions {
                state.apply_transaction(tx)?;
            }

            // Recompute hash
            let expected_hash = current.calculate_hash();
            if expected_hash != current.hash {
                return Err("Invalid block hash".into());
            }
        }

        Ok(())
    }

    pub fn get_current_state(&self) -> State {
        let mut state = State::new();

        // genesis allocation
        state.balances.insert("alice".into(), 100);

        for i in 1..self.chain.len() {
            let block = &self.chain[i];

            for tx in &block.transactions {
                state.apply_transaction(tx).unwrap();
            }
        }
        state
    }

    pub fn produce_block(&mut self, mempool: &mut Mempool, max_txs: usize) {
        let mut state = self.get_current_state();
        let miner = "miner".to_string();
        let mut total_fees = 0;

        let mut selected = Vec::new();
        let txs = mempool.get_transactions(max_txs);

        for tx in txs {
            if state.apply_transaction(&tx).is_ok() {
                selected.push(tx);
            }
        }

        if selected.is_empty() {
            println!("No valid transactions to include");
            return;
        }

        for tx in &selected {
            total_fees += tx.fee;
        }

        *state.balances.entry(miner).or_insert(0) += total_fees;


        self.add_block(selected);
    }
}