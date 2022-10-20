# cosmwasm-risc0-example
CosmWasm + RISC-V zkVM `gm` example
## Overview

- [CosmWasm](https://github.com/CosmWasm/cosmwasm)
- [RISC Zero](https://github.com/risc0/risc0)

This example exists to explore the patterns of use of CosmWasm and risc0 to build smart contracts that allow the possibility to carry out **general purpose computation** privatel.
## zkVM
A zero-knowledge virtual machine (zkVM) is a virtual machine that runs trusted code and generates proofs that authenticate the zkVM output. RISC Zero's zkVM implementation, based on the RISC-V architecture, executes code and produces a computational receipt.

The codebase was adapted from `risc0-rust-starter` repository and is intended to be a minimal starting point for `CosmWasm` development. In this instance, we perform multiplication to prove that we know the factors of `1337`.

## What's next?
[tic-tac-toe](https://github.com/bmorphism/risc0-cosmwasm-example/issues/1)!
