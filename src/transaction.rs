use sha2::{Sha256, Digest};

pub type Address = String;

#[derive(Clone, Debug)]
pub struct Transaction {
    pub from: Address,
    pub to: Address,
    pub amount: u64,
    pub nonce: u64,
    pub fee: u64,
}

impl Transaction {
    pub fn new(from: Address, to: Address, amount: u64, nonce: u64, fee: u64) -> Self {
        Self { from, to, amount, nonce, fee }
    }

    pub fn hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(format!(
            "{}{}{}{}",
            self.from, self.to, self.amount, self.nonce
        ));
        format!("{:x}", hasher.finalize())
    }
}