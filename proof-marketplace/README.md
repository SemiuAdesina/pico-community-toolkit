# Pico Proof Marketplace

Decentralized marketplace for Pico zkVM proof generation services with prover discovery, bidding, and reputation systems.

## Overview

This tool provides a decentralized marketplace where proof requesters can find and hire proof providers, enabling efficient resource utilization and competitive pricing for Pico zkVM proof generation services.

## Features

- **Prover Discovery**: Find and compare proof providers
- **Bidding System**: Competitive bidding for proof generation jobs
- **Reputation Management**: Track prover performance and reliability
- **Multi-Backend Support**: Support for all Pico proving backends
- **Pricing Models**: Flexible pricing including pay-per-proof and subscription models
- **Quality Assurance**: Built-in verification and quality checks
- **Market Analytics**: Market statistics and trend analysis

## Installation

```bash
cd proof-marketplace
cargo build --release
```

## Usage

### Register as a Proof Provider

```bash
# Register as a proof provider
cargo run -- register --id my_prover --name "Fast Prover Service" --backends koalabear,babybear

# Register with pricing information
cargo run -- register --id my_prover --name "Fast Prover" --backends koalabear --base-price 1000000 --price-type per_proof
```

### Submit Proof Requests

```bash
# Submit a proof request
cargo run -- request --program-id hash_abc123 --input-data input.bin --backend koalabear

# Submit with specific requirements
cargo run -- request --program-id hash_abc123 --input-data input.bin --backend koalabear --priority high --max-price 2000000
```

### Search for Proof Providers

```bash
# Find provers by backend
cargo run -- provers --backend koalabear

# Find provers by price and reputation
cargo run -- provers --backend koalabear --max-price 1500000 --min-reputation 4.5
```

### View Market Statistics

```bash
# Get overall market statistics
cargo run -- stats

# Get statistics in JSON format
cargo run -- stats --format json --output market_stats.json
```

## Command Reference

### `register`
Register as a proof provider in the marketplace.

**Options:**
- `--id`: Unique prover identifier
- `--name`: Display name for the prover
- `--backends`: Comma-separated list of supported backends
- `--base-price`: Base price for proof generation
- `--price-type`: Pricing model (per_proof, subscription, auction)
- `--reputation`: Initial reputation score

### `request`
Submit a proof generation request to the marketplace.

**Options:**
- `--program-id`: Hash or identifier of the program
- `--input-data`: Path to input data file
- `--backend`: Preferred proving backend
- `--priority`: Request priority (low, normal, high, urgent)
- `--max-price`: Maximum price willing to pay
- `--min-reputation`: Minimum prover reputation requirement
- `--deadline`: Request deadline in minutes

### `provers`
Search and list available proof providers.

**Options:**
- `--backend`: Filter by supported backend
- `--max-price`: Filter by maximum price
- `--min-reputation`: Filter by minimum reputation
- `--status`: Filter by prover status (active, busy, offline)
- `--sort`: Sort order (price, reputation, response_time)

### `stats`
View marketplace statistics and analytics.

**Options:**
- `--format`: Output format (text, json, csv)
- `--output`: Output file path
- `--period`: Time period for statistics (1h, 24h, 7d, 30d)

## Marketplace Architecture

### Participants

#### Proof Requesters
- Submit proof generation requests
- Set requirements and constraints
- Evaluate and select bids
- Receive completed proofs

#### Proof Providers
- Register services and capabilities
- Bid on proof requests
- Generate proofs as requested
- Maintain reputation through quality service

#### Market Operators
- Maintain marketplace infrastructure
- Monitor system health and performance
- Resolve disputes and issues
- Provide analytics and reporting

## Pricing Models

### Pay-per-Proof
- Fixed price per individual proof
- Suitable for one-off or irregular usage
- Simple pricing structure

### Subscription
- Monthly or annual subscription fees
- Unlimited or capped proof generation
- Suitable for regular, high-volume usage

### Auction
- Competitive bidding for proof requests
- Market-driven pricing
- Suitable for time-sensitive or high-value requests

## Contributing

1. Add new pricing models in `src/pricing.rs`
2. Implement reputation algorithms in `src/reputation.rs`
3. Enhance bidding system in `src/bidding.rs`
4. Add new analytics in `src/analytics.rs`

## Dependencies

- `pico-sdk`: Pico zkVM SDK
- `pico-vm`: Pico virtual machine
- `axum`: Web framework for API
- `tokio`: Async runtime
- `chrono`: Date and time handling
- `uuid`: Unique identifier generation
- `serde`: Serialization framework
- `clap`: Command-line argument parsing
