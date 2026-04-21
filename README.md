# rust-blockchain-core

A blockchain implementation built from scratch in Rust — no frameworks, no shortcuts.

Built to understand how blockchains work at the protocol level: cryptographic hashing,
digital signatures, proof-of-work mining, Merkle trees, account state management,
and a mempool for pending transactions.

---

## Features

- **SHA-256 block hashing** — every block is fingerprinted across all its fields; tampering any value invalidates the hash and breaks the chain
- **Proof-of-Work mining** — adjustable difficulty target; miners increment a nonce until the block hash starts with N leading zeros
- **Merkle tree** — all transactions in a block are summarized into a single root hash; changing any transaction invalidates the block
- **Ed25519 digital signatures** — every transaction is signed with the sender's private key and verified before entering the mempool or a block
- **Account state & state root** — tracks balances and nonces per address; the state root is committed into every block so any balance manipulation is immediately detectable
- **Nonce-based replay protection** — each transaction carries a per-sender nonce, preventing the same transaction from being resubmitted
- **Mempool** — pending transactions queue up, are signature-verified on entry, and validated against current state before being included in the next block
- **Full chain validation** — replays the entire chain from genesis verifying hash integrity, chain linkage, signatures, state transitions, and state root at every block
- **Cumulative difficulty & chain selection** — when two competing chains exist, the one with higher cumulative proof-of-work wins
- **Node & peer structure** — basic node abstraction with peer list and broadcast stubs, laying the groundwork for a P2P network layer

---

## Project structure
src/
├── main.rs          # Demo: two nodes, transactions, mining, chain sync
├── block.rs         # Block struct, SHA-256 hashing, Merkle tree, PoW mining
├── blockchain.rs    # Chain management, validation, chain selection
├── transaction.rs   # Transaction struct, Ed25519 signing and verification
├── mempool.rs       # Pending transaction queue
├── state.rs         # Account balances, nonces, state root
└── node.rs          # Node abstraction, peer list, broadcast stubs

---

## How it works

### Block production
Transactions submitted to mempool
│
▼
Verify signatures + validate against current state
│
▼
Build Merkle root from transaction hashes
│
▼
Compute state root (Merkle root of all balances + nonces)
│
▼
Proof-of-Work: increment nonce until hash starts with N zeros
│
▼
Block appended to chain, state committed

### Chain validation

Every block is checked for:

1. **Hash integrity** — recalculate hash from fields, compare to stored hash
2. **Chain linkage** — `block.previous_hash` must equal `chain[i-1].hash`
3. **Transaction signatures** — every transaction's Ed25519 signature is verified
4. **State transitions** — balances and nonces are replayed; invalid transactions reject the chain
5. **State root** — post-execution state root must match the root committed in the block

### Chain selection

When a node receives a competing chain, it runs full validation and only adopts it
if its cumulative difficulty exceeds the current chain — the same principle Bitcoin uses.

---

## Getting started

**Prerequisites:** Rust toolchain — [rustup.rs](https://rustup.rs)

```bash
git clone https://github.com/Sammmmx/rust-blockchain-core.git
cd rust-blockchain-core
cargo run
```

### Sample output

Mined! nonce=38471 hash=00f3a9c2b74e...
Mined! nonce=91042 hash=00b7e1d4cc3f...
Broadcasting block 2 to Node B
Chain replaced with higher cumulative difficulty
Node A chain length: 3
Node B chain length: 3
Chain valid: true

---

## Dependencies

```toml
sha2 = "0.10"
hex  = "0.4"
ed25519-dalek = { version = "2", features = ["rand_core"] }
rand = "0.8"
```

---

## What is not implemented

This is a single-process simulation. A production blockchain would additionally require:

- Real P2P networking over TCP (e.g. via [libp2p](https://libp2p.io))
- Persistent storage — writing the chain to disk
- Live fork resolution across real network nodes
- Transaction fees and block rewards
- An RPC or API layer for external interaction

---

## Concepts demonstrated

| Concept | Location |
|---|---|
| SHA-256 hashing | `block.rs` → `calculate_hash()` |
| Proof-of-Work | `block.rs` → `mine()` |
| Merkle tree | `block.rs` → `merkle_root()` |
| Ed25519 signing | `transaction.rs` → `sign()`, `verify()` |
| Account state | `state.rs` → `apply_transaction()` |
| State root | `state.rs` → `root()` |
| Chain validation | `blockchain.rs` → `is_valid_chain()` |
| Chain selection | `blockchain.rs` → `replace_chain()` |
| Mempool | `mempool.rs` → `add_transaction()`, `get_transactions()` |

---

## Author

**Samyak Narnaware** — [github.com/Sammmmx](https://github.com/Sammmmx)

Blockchain developer focused on Ethereum smart contracts (Solidity) and low-level
blockchain infrastructure (Rust). Open to junior roles and internships in Web3.
