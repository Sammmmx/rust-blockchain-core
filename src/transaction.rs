use sha2::{Sha256, Digest};
use ed25519_dalek::{SigningKey, VerifyingKey, Signer, Verifier, Signature};

#[derive(Clone)]
pub struct Transaction {
    pub sender: VerifyingKey,
    pub receiver: VerifyingKey,
    pub amount: u64,
    pub nonce: u64,
    pub signature: Signature,
}

impl Transaction {
    pub fn new(
        sender: VerifyingKey,
        receiver: VerifyingKey,
        amount: u64,
        nonce: u64,
        signing_key: &SigningKey,
    ) -> Self {
        let mut tx = Transaction {
            sender,
            receiver,
            amount,
            nonce,
            signature: Signature::from_bytes(&[0; 64]),
        };
        tx.sign(signing_key);
        tx
    }

    pub fn hash(&self) -> String {
        let mut hasher = Sha256::new();
        let input = format!("{:?}{:?}{}{}", self.sender, self.receiver, self.amount, self.nonce);
        hasher.update(input);
        format!("{:x}", hasher.finalize())
    }

    pub fn sign(&mut self, signing_key: &SigningKey) {
        let hash = self.hash();
        self.signature = signing_key.sign(hash.as_bytes());
    }

    pub fn verify(&self) -> bool {
        self.sender.verify(self.hash().as_bytes(), &self.signature).is_ok()
    }
}