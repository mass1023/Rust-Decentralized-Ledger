# Rust-Decentralized-Ledger

Rust Decentralized Ledger

This is the capstone project for the Rust Live Accelerator program. This project implements a decentralized ledger system using Rust, demonstrating core blockchain concepts including blocks, transactions, and cryptographic hashing.

## Features

- Create and manage a blockchain with genesis block
- Add transactions between users
- Mine new blocks with proof-of-work
- Validate blockchain integrity
- Query account balances
- Serialize/deserialize blockchain data

## Installation

Ensure you have Rust installed. Clone the repository and build:

```bash
cargo build
```

## Usage

Run the program to see a demonstration:

```bash
cargo run
```

The program will:

1. Create a genesis block
2. Add a sample transaction
3. Mine a new block
4. Display balances and validate the chain

## Dependencies

- `sha2`: For cryptographic hashing
- `serde` & `serde_json`: For data serialization
- `chrono`: For timestamp handling
- `rand`: For random number generation
