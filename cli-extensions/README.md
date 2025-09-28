# Pico CLI Extensions

Extended command-line utilities for enhanced Pico zkVM development workflow.

## Overview

This tool provides additional CLI commands and utilities that extend the standard Pico development workflow with advanced analysis, optimization, and testing capabilities.

## Features

- **ELF Analysis**: Analyze ELF binaries for size, complexity, and optimization opportunities
- **Program Optimization**: Optimize programs for specific backends and use cases
- **Batch Testing**: Run tests across multiple programs and configurations
- **Performance Profiling**: Profile program execution and identify bottlenecks
- **Security Analysis**: Analyze programs for potential security issues

## Installation

```bash
cd cli-extensions
cargo build --release
```

## Usage

### Analyze ELF Files

```bash
# Basic ELF analysis
cargo run -- analyze --elf /path/to/program.elf

# Detailed analysis with optimization suggestions
cargo run -- analyze --elf /path/to/program.elf --format detailed

# Export analysis results
cargo run -- analyze --elf /path/to/program.elf --output analysis.json
```

### Optimize Programs

```bash
# Optimize for specific backend
cargo run -- optimize --elf /path/to/program.elf --backend koalabear

# Optimize with custom settings
cargo run -- optimize --elf /path/to/program.elf --backend babybear --output optimized.elf
```

### Batch Testing

```bash
# Test multiple programs
cargo run -- test --directory ./test_programs --inputs ./test_inputs

# Test with specific configuration
cargo run -- test --directory ./programs --inputs ./inputs --output results.json
```

### Performance Profiling

```bash
# Profile single program
cargo run -- profile --elf /path/to/program.elf --input test_data.bin

# Profile with memory analysis
cargo run -- profile --elf /path/to/program.elf --input test_data.bin --memory

# Profile with multiple iterations
cargo run -- profile --elf /path/to/program.elf --input test_data.bin --iterations 10
```

## Command Reference

### `analyze`
Analyze ELF files for various metrics and optimization opportunities.

**Options:**
- `--elf`: Path to ELF file to analyze
- `--format`: Output format (basic, detailed, json)
- `--output`: Output file for results
- `--detailed`: Include detailed analysis

### `optimize`
Optimize ELF files for specific backends or use cases.

**Options:**
- `--elf`: Path to ELF file to optimize
- `--backend`: Target backend for optimization
- `--output`: Output path for optimized ELF
- `--aggressive`: Use aggressive optimization settings

### `test`
Run batch tests on multiple programs.

**Options:**
- `--directory`: Directory containing programs to test
- `--inputs`: Directory containing test inputs
- `--output`: Output file for test results
- `--parallel`: Run tests in parallel

### `profile`
Profile program performance and resource usage.

**Options:**
- `--elf`: Path to ELF file to profile
- `--input`: Input data for profiling
- `--iterations`: Number of profiling iterations
- `--memory`: Include memory profiling
- `--cycles`: Include cycle counting

## Contributing

1. Add new analysis methods in `src/analyzer.rs`
2. Implement optimization algorithms in `src/optimizer.rs`
3. Add new test frameworks in `src/tester.rs`
4. Enhance profiling capabilities in `src/profiler.rs`

## Dependencies

- `pico-sdk`: Pico zkVM SDK
- `clap`: Command-line argument parsing
- `serde`: Serialization framework
- `anyhow`: Error handling
