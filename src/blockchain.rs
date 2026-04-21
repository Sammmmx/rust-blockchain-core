use ed25519_dalek::{SigningKey, VerifyingKey};
use crate::block::{Block, calculate_hash};
use crate::transaction::Transaction;
use crate::mempool::Mempool;
use crate::state::State;

pub struct Blockchain {
    pub chain: Vec<Block>,
    pub mempool: Mempool,
    pub state: State,
    pub difficulty: usize,
}

impl Blockchain {
    pub fn new(signing_key: &SigningKey, verifying_key: &VerifyingKey, difficulty: usize) -> Self {
        let genesis_tx = Transaction::new(*verifying_key, *verifying_key, 0, 0, signing_key);
        let genesis_data = vec![genesis_tx];

        let mut state = State::new();
        let address = format!("{:?}", verifying_key);
        state.balances.insert(address, 1000);

        let genesis = Block::new(
            0,
            genesis_data,
            String::from("0"),
            signing_key,
            verifying_key,
            0,
            difficulty,
            &state,
        );

        Blockchain {
            chain: vec![genesis],
            mempool: Mempool::new(),
            state,
            difficulty,
        }
    }

    pub fn add_block(&mut self, transactions: Vec<Transaction>, signing_key: &SigningKey, verifying_key: &VerifyingKey) {
        let previous_block = self.chain.last().unwrap();
        let previous_hash = previous_block.hash.clone();
        let previous_cumulative = previous_block.cumulative_difficulty;

        let new_block = Block::new(
            self.chain.len() as u32,
            transactions,
            previous_hash,
            signing_key,
            verifying_key,
            previous_cumulative,
            self.difficulty,
            &self.state,
        );

        self.chain.push(new_block);
    }

    pub fn submit_transaction(&mut self, tx: Transaction) {
        self.mempool.add_transaction(tx);
    }

    pub fn mine_pending(&mut self, signing_key: &SigningKey, verifying_key: &VerifyingKey) {
        let txs = self.mempool.get_transactions(10);

        if txs.is_empty() {
            println!("No transactions to mine");
            return;
        }

        let mut temp_state = self.state.clone();
        let mut valid_txs = Vec::new();

        for tx in txs {
            if temp_state.apply_transaction(&tx) {
                valid_txs.push(tx);
            }
        }

        self.add_block(valid_txs, signing_key, verifying_key);
        self.state = temp_state;
    }

    /// Replaces the current chain with a new one if it is valid and has
    /// higher cumulative difficulty (heaviest chain wins).
    pub fn replace_chain(&mut self, new_chain: Vec<Block>) {
        if !Blockchain::is_valid_chain(&new_chain, &self.state) {
            println!("Invalid chain rejected");
            return;
        }

        let current_work = self.chain.last().unwrap().cumulative_difficulty;
        let new_work = new_chain.last().unwrap().cumulative_difficulty;

        if new_work > current_work {
            self.chain = new_chain;
            println!("Chain replaced with higher cumulative difficulty");
        }
    }

    /// Replays the entire chain from genesis verifying:
    /// 1. Hash integrity (no field was tampered)
    /// 2. Chain linkage (each block points to previous)
    /// 3. Transaction signatures
    /// 4. State transitions (balances, nonces)
    /// 5. State root matches post-execution state
    pub fn is_valid_chain(chain: &Vec<Block>, initial_state: &State) -> bool {
        let mut temp_state = initial_state.clone();

        for i in 0..chain.len() {
            let block = &chain[i];

            // 1. Hash integrity
            let recalculated = calculate_hash(
                &block.index,
                &block.timestamp,
                &block.merkle_root,
                &block.state_root,
                &block.previous_hash,
                &block.nonce,
            );
            if block.hash != recalculated {
                println!("Block {}: hash mismatch", i);
                return false;
            }

            // 2. Chain linkage
            if i > 0 && block.previous_hash != chain[i - 1].hash {
                println!("Block {}: previous hash mismatch", i);
                return false;
            }

            // 3. Signatures + state transitions
            for tx in &block.transactions {
                if !tx.verify() {
                    println!("Block {}: invalid transaction signature", i);
                    return false;
                }
                if !temp_state.apply_transaction(tx) {
                    println!("Block {}: invalid state transition", i);
                    return false;
                }
            }

            // 4. State root
            if block.state_root != temp_state.root() {
                println!("Block {}: state root mismatch", i);
                return false;
            }
        }

        true
    }
}