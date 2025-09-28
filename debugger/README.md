# Pico Debugger/Profiler

Advanced debugging and profiling tools for Pico zkVM programs with interactive debugging capabilities.

## Overview

This tool provides comprehensive debugging and profiling capabilities for Pico zkVM programs, including interactive debugging with breakpoints, step execution, memory inspection, and performance profiling.

## Features

- **Interactive Debugging**: Set breakpoints, step through execution, inspect variables
- **Execution Tracing**: Detailed execution traces with cycle-by-cycle analysis
- **Memory Inspection**: Inspect memory state at any point during execution
- **Performance Profiling**: Profile execution time, memory usage, and cycle counts
- **Memory Analysis**: Track memory allocations, leaks, and usage patterns
- **TUI Interface**: Terminal-based user interface for debugging sessions

## Installation

```bash
cd debugger
cargo build --release
```

## Usage

### Interactive Debugging

```bash
# Start interactive debugging session
cargo run -- debug --program /path/to/program.elf --input test_data.bin

# Debug with specific breakpoints
cargo run -- debug --program /path/to/program.elf --breakpoint 0x1000 --breakpoint 0x2000
```

### Performance Profiling

```bash
# Basic profiling
cargo run -- profile --program /path/to/program.elf --input test_data.bin

# Profiling with memory analysis
cargo run -- profile --program /path/to/program.elf --input test_data.bin --memory

# Multiple iterations profiling
cargo run -- profile --program /path/to/program.elf --input test_data.bin --iterations 100
```

## Command Reference

### `debug`
Start interactive debugging session.

**Options:**
- `--program`: Path to ELF program to debug
- `--input`: Input data for the program
- `--breakpoint`: Set breakpoint at address (can be used multiple times)
- `--watch`: Watch variable or memory address
- `--verbose`: Enable verbose debugging output

### `profile`
Profile program performance and resource usage.

**Options:**
- `--program`: Path to ELF program
- `--input`: Input data for profiling
- `--iterations`: Number of profiling iterations
- `--memory`: Include memory profiling
- `--cycles`: Include cycle counting
- `--output`: Output file for profiling results

## Contributing

1. Add new debugging features in `src/debugger.rs`
2. Implement profiling algorithms in `src/profiler.rs`
3. Enhance TUI interface in `src/ui.rs`
4. Add support for new architectures in `src/arch/`

## Dependencies

- `pico-sdk`: Pico zkVM SDK
- `pico-vm`: Pico virtual machine
- `clap`: Command-line argument parsing
- `ratatui`: Terminal user interface
- `crossterm`: Cross-platform terminal manipulation
- `chrono`: Date and time handling
- `serde`: Serialization framework
