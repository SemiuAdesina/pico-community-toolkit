# Pico Testing Framework

Comprehensive testing framework for Pico zkVM programs with async support, performance benchmarking, and coverage reporting.

## Overview

This framework provides a complete testing solution for Pico zkVM programs, including unit tests, integration tests, performance benchmarks, and code coverage analysis.

## Features

- **Async Test Execution**: Run tests asynchronously for better performance
- **Multiple Output Formats**: Export results in JSON, JUnit XML, and text formats
- **Performance Benchmarking**: Built-in performance testing and analysis
- **Code Coverage**: Generate coverage reports for test completeness
- **Test Templates**: Generate test templates for new programs
- **Batch Testing**: Run tests across multiple programs and configurations
- **Validation**: Validate test cases and configurations

## Installation

```bash
cd testing-framework
cargo build --release
```

## Usage

### Run Tests

```bash
# Run all tests in a directory
cargo run -- run --path ./tests --backend koalabear

# Run with specific output format
cargo run -- run --path ./tests --format json --output results.json

# Run with custom backend and timeout
cargo run -- run --path ./tests --backend babybear --timeout 30
```

### Generate Test Templates

```bash
# Generate test template for a program
cargo run -- generate --name fibonacci_test --program fibonacci.elf

# Generate with custom output file
cargo run -- generate --name merkle_test --program merkle.elf --output merkle_test.json
```

### Create Benchmark Suites

```bash
# Create benchmark suite with multiple iterations
cargo run -- benchmark --program fibonacci.elf --iterations 50

# Create benchmark with custom output
cargo run -- benchmark --program fibonacci.elf --iterations 100 --output benchmark.json
```

### Performance Testing

```bash
# Run performance tests
cargo run -- performance --program fibonacci.elf --iterations 10

# Performance test with warmup
cargo run -- performance --program fibonacci.elf --iterations 20 --warmup 5
```

## Command Reference

### `run`
Execute test suites and generate results.

**Options:**
- `--path`: Test directory or file to run
- `--backend`: Backend to use for testing (default: koalabear)
- `--format`: Output format (text, json, junit)
- `--output`: Output file for results
- `--timeout`: Test timeout in seconds
- `--parallel`: Run tests in parallel

### `generate`
Generate test templates for new programs.

**Options:**
- `--name`: Name for the test case
- `--program`: Path to program ELF file
- `--output`: Output file for test template
- `--template`: Use custom template

### `benchmark`
Create benchmark test suites.

**Options:**
- `--program`: Program to benchmark
- `--iterations`: Number of benchmark iterations
- `--output`: Output file for benchmark suite
- `--warmup`: Number of warmup iterations

### `performance`
Run performance tests and benchmarks.

**Options:**
- `--program`: Program to test
- `--iterations`: Number of test iterations
- `--warmup`: Number of warmup iterations
- `--memory`: Include memory profiling
- `--output`: Output file for performance results

## Contributing

1. Add new test types in `src/test_types.rs`
2. Implement performance metrics in `src/performance.rs`
3. Add coverage analysis in `src/coverage.rs`
4. Enhance reporting in `src/reporting.rs`

## Dependencies

- `pico-sdk`: Pico zkVM SDK
- `pico-vm`: Pico virtual machine
- `tokio`: Async runtime
- `clap`: Command-line argument parsing
- `serde`: Serialization framework
- `anyhow`: Error handling
