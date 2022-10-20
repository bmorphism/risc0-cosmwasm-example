# cosmwasm-risc0: RISC-V zkVM + CosmWasm 🐈‍⬛
## What is this?
[CosmWasm](https://github.com/CosmWasm/cosmwasm) is for building smart contracts targeting server-side wasm
[RISC Zero](https://github.com/risc0/risc0) is a verifiable **general computing platform** based on **zk-STARKs** and the **RISC-V** microarchitecture.

This example exists to establish patterns of use of CosmWasm and risc0 to build smart contracts with robust privacy and correctness guarantees. The possibility to carry out **general purpose computation** privately in a way that is verifiably the codebase intended, and using RISC-V instruction set that will could be accelerated using hardware in the future is a very exciting and powerful combination that we believe is worth exploring.
## zkVM
A zero-knowledge virtual machine (zkVM) is a virtual machine that runs trusted code and generates proofs that authenticate the zkVM output. RISC Zero's zkVM implementation, based on the RISC-V architecture, executes code and produces a computational receipt.

The codebase was adapted from `risc0-rust-starter` repository and is intended to be a minimal starting point for `zkCosmWasm` development. In this instance, we perform multiplication to prove that we know the factors of `1337`.

## What's next?
Let's implement a CosmWasm [tic-tac-toe](https://github.com/bmorphism/risc0-cosmwasm-example/issues/1) as a foundation for more complex games and protocols!

## Authors
- [Jake Hartnell](https://twitter.com/jakehartnell)
- [Barton Rhodes](https://twitter.com/bmorphism)