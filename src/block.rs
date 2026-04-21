use std::time::{SystemTime, UNIX_EPOCH};
use sha2::{Sha256, Digest};
use ed25519_dalek::{SigningKey, VerifyingKey};
use crate::transaction::Transaction;
use crate::state::State;

#[derive(Clone)]
pub struct Block {
    pub index: u32,
    pub timestamp: u128,
    pub transactions: Vec<Transaction>,
    pub merkle_root: String,
    pub state_root: String,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
    pub cumulative_difficulty: u64,
    pub author: VerifyingKey,
}

impl Block {
    pub fn new(
        index: u32,
        transactions: Vec<Transaction>,
        previous_hash: String,
        signing_key: &SigningKey,
        verifying_key: &VerifyingKey,
        previous_cumulative_difficulty: u64,
        difficulty: usize,
        state: &State,
    ) -> Block {
        let timestamp = get_timestamp();
        let tx_hashes: Vec<String> = transactions.iter().map(|tx| tx.hash()).collect();
        let merkle_root = merkle_root(&tx_hashes);
        let state_root = state.root();
        let (hash, nonce) = mine(index, &timestamp, &merkle_root, &state_root, &previous_hash, difficulty);
        let cumulative_difficulty = previous_cumulative_difficulty + difficulty as u64;

        Block {
            index,
            timestamp,
            transactions,
            merkle_root,
            state_root,
            previous_hash,
            hash,
            nonce,
            cumulative_difficulty,
            author: *verifying_key,
        }
    }
}

/// SHA-256 hash of all block header fields including nonce.
/// Including nonce allows proof-of-work: keep hashing with different
/// nonces until the hash meets the difficulty target.
pub fn calculate_hash(
    index: &u32,
    timestamp: &u128,
    merkle_root: &String,
    state_root: &String,
    previous_hash: &String,
    nonce: &u64,
) -> String {
    let mut hasher = Sha256::new();
    let input = format!("{}|{}|{}|{}|{}|{}", index, timestamp, merkle_root, state_root, previous_hash, nonce);
    hasher.update(&input);
    format!("{:x}", hasher.finalize())
}

/// Builds a Merkle tree from a list of hashes and returns the root.
/// An odd number of leaves duplicates the last leaf before pairing.
pub fn merkle_root(data: &Vec<String>) -> String {
    if data.is_empty() {
        return String::from("0");
    }

    let mut hashes: Vec<String> = data
        .iter()
        .map(|record| {
            let mut hasher = Sha256::new();
            hasher.update(record);
            format!("{:x}", hasher.finalize())
        })
        .collect();

    while hashes.len() > 1 {
        if hashes.len() % 2 != 0 {
            hashes.push(hashes.last().unwrap().clone());
        }

        let mut new_level = Vec::new();
        for i in (0..hashes.len()).step_by(2) {
            let combined = format!("{}{}", hashes[i], hashes[i + 1]);
            let mut hasher = Sha256::new();
            hasher.update(&combined);
            new_level.push(format!("{:x}", hasher.finalize()));
        }
        hashes = new_level;
    }

    hashes[0].clone()
}

/// Proof-of-Work: increment nonce until hash starts with `difficulty` zeros.
fn mine(
    index: u32,
    timestamp: &u128,
    merkle_root: &String,
    state_root: &String,
    previous_hash: &String,
    difficulty: usize,
) -> (String, u64) {
    let target = "0".repeat(difficulty);
    let mut nonce: u64 = 0;

    loop {
        let hash = calculate_hash(&index, timestamp, merkle_root, state_root, previous_hash, &nonce);
        if hash.starts_with(&target) {
            println!("Mined! nonce={} hash={}", nonce, hash);
            return (hash, nonce);
        }
        nonce += 1;
    }
}

pub fn get_timestamp() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis()
}