# ZK-Puzzle-Chain: A Simple Blockchain using Zero-Knowledge Proofs for Consensus

ZK-Puzzle-Chain is an experimental blockchain project that combines the concepts of Proof-of-Work (PoW) and Zero-Knowledge Proofs (ZKP) to create a novel consensus mechanism.

This project is built on top of the [RISC-Zero's zkVM](https://risc0.com/).

This project is an experimental exploration of integrating [RISC-Zero's zkVM](https://risc0.com/) into a blockchain system. It's designed for personal learning.


## Features

- Proof-of-Work consensus mechanism
- Zero-Knowledge Proofs for block verification
- JSON-RPC API for interacting with the blockchain
- Storage layer using PostgreSQL
- Just for demo, no transaction, no account, no smart contract, no P2P

## About ZK-POW

[zk-pow](./zk-pow/) is a simple proof-of-work algorithm that is used to secure the blockchain. It is implemented in Rust and uses the RISC Zero zkVM to create and verify proofs of correct computation.

Feel free to play with it and try to break it.

## Getting Started

### Prerequisites

- [Rust programming language](https://rustup.rs/)
- [RISC Zero SDK](https://dev.risczero.com/api/zkvm/install)

### Installation

1. Clone the repository:
   ```
   git clone https://github.com/keroro520/zk-puzzle-chain.git
   cd zk-puzzle-chain
   ```

2. Build the project:
   ```
   cargo build --release
   ```

### Running the Node

```
cargo run --bin client --release
```

### Interacting with the Blockchain

You can interact with the blockchain using the JSON-RPC API. The node will start a JSON-RPC server on `http://localhost:3030`.

### Running the Miner

```
cargo run --bin miner --release
```

## License

This project is licensed under the MIT License. See the `LICENSE` file for more details.