# Pico Backend Comparison Tool

A comprehensive tool for comparing the performance of different Pico zkVM proving backends.

## Overview

This tool allows developers to benchmark and compare the performance of different Pico proving backends (KoalaBear, BabyBear, M31) to choose the optimal backend for their specific use cases.

## Features

- **Multi-Backend Comparison**: Compare KoalaBear, BabyBear, and M31 backends
- **Performance Benchmarking**: Measure proof generation time, verification time, and memory usage
- **Statistical Analysis**: Generate detailed performance reports with averages, percentiles, and comparisons
- **Export Formats**: Export results in JSON format for further analysis
- **Customizable Tests**: Configure iterations, chunk sizes, and other parameters

## Installation

```bash
cd backend-comparison
cargo build --release
```

## Usage

### Compare Backends for a Specific Program

```bash
# Compare all backends for a specific ELF file
cargo run -- compare --elf /path/to/program.elf --iterations 5

# Compare specific backends only
cargo run -- compare --elf /path/to/program.elf --backends koalabear,babybear --iterations 10
```

### Run Comprehensive Benchmark Suite

```bash
# Run benchmarks on multiple test programs
cargo run -- benchmark --directory ./test_programs --iterations 3 --output results.json
```

### Analyze Existing Results

```bash
# Analyze previously generated benchmark results
cargo run -- analyze --input results.json --format detailed
```

## Command Reference

### `compare`
Compare backends for a specific ELF file.

**Options:**
- `--elf`: Path to the ELF file to test
- `--backends`: Comma-separated list of backends (default: all)
- `--iterations`: Number of iterations per backend (default: 5)
- `--output`: Output file for results (default: stdout)

### `benchmark`
Run comprehensive benchmark suite on multiple programs.

**Options:**
- `--directory`: Directory containing ELF files to test
- `--iterations`: Number of iterations per test (default: 3)
- `--output`: Output file for results
- `--recursive`: Search subdirectories recursively

### `analyze`
Analyze existing benchmark results.

**Options:**
- `--input`: Input file containing benchmark results
- `--format`: Output format (summary, detailed, json)
- `--output`: Output file (default: stdout)

## Backend Comparison

| Backend | Strengths | Best For |
|---------|-----------|----------|
| KoalaBear | Fast proof generation | High-throughput applications |
| BabyBear | Memory efficient | Resource-constrained environments |
| M31 | Small proof size | On-chain verification |

## Contributing

1. Add new benchmark scenarios in `src/benchmarks/`
2. Implement new analysis methods in `src/analysis/`
3. Add support for additional backends in `src/backends/`

## Dependencies

- `pico-sdk`: Pico zkVM SDK
- `serde`: Serialization framework
- `clap`: Command-line argument parsing
- `anyhow`: Error handling
