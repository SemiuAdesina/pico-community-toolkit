# Pico EVM Integration Helper

Tools for integrating Pico zkVM proofs with Ethereum Virtual Machine (EVM) smart contracts.

## Overview

This tool simplifies the integration of Pico zkVM proofs with EVM-based blockchains by providing utilities to generate Solidity contracts, format proofs for EVM consumption, and manage verification contracts.

## Features

- **Solidity Contract Generation**: Generate verification contracts for Pico proofs
- **Proof Formatting**: Convert Pico proofs to EVM-compatible formats
- **Gas Estimation**: Estimate gas costs for proof verification
- **Deployment Scripts**: Generate deployment scripts for verification contracts
- **EVM Integration Validation**: Validate proof format compatibility with EVM

## Installation

```bash
cd evm-helper
cargo build --release
```

## Usage

### Generate Solidity Contracts

```bash
# Generate basic verification contract
cargo run -- generate --name FibonacciVerifier --program fibonacci.elf

# Generate with custom output path
cargo run -- generate --name MyVerifier --program my_program.elf --output ./contracts/
```

### Format Proofs for EVM

```bash
# Format proof for Solidity calldata
cargo run -- format --proof proof.bin --format solidity --output formatted_proof.json

# Format for different EVM formats
cargo run -- format --proof proof.bin --format calldata --output proof_calldata.hex
cargo run -- format --proof proof.bin --format json --output proof.json
```

### Estimate Gas Costs

```bash
# Estimate gas for proof verification
cargo run -- gas --contract MyVerifier.sol --proof proof.bin

# Estimate with different proof sizes
cargo run -- gas --contract MyVerifier.sol --proof proof.bin --iterations 100
```

## Command Reference

### `generate`
Generate Solidity verification contracts for Pico proofs.

**Options:**
- `--name`: Name of the verification contract
- `--program`: Path to Pico program ELF file
- `--output`: Output directory for generated contracts
- `--template`: Use custom contract template

### `format`
Format Pico proofs for EVM consumption.

**Options:**
- `--proof`: Path to proof file
- `--format`: Output format (solidity, calldata, json, raw)
- `--output`: Output file path
- `--optimize`: Optimize for gas efficiency

### `gas`
Estimate gas costs for proof verification.

**Options:**
- `--contract`: Path to contract file
- `--proof`: Path to proof file
- `--iterations`: Number of estimation iterations
- `--network`: Network for gas price lookup

## Contributing

1. Add new contract templates in `src/templates/`
2. Implement proof formatting algorithms in `src/formatter.rs`
3. Add support for new EVM versions in `src/evm/`
4. Enhance gas estimation in `src/gas.rs`

## Dependencies

- `pico-sdk`: Pico zkVM SDK
- `pico-vm`: Pico virtual machine
- `ethers`: Ethereum library for contract interaction
- `solang-parser`: Solidity parsing and AST manipulation
- `clap`: Command-line argument parsing
- `serde`: Serialization framework
