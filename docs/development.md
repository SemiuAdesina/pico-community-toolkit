# Development Guide

## Getting Started

This guide will help you set up your development environment and contribute to the Pico Community Toolkit.

## Prerequisites

### Required Tools
- **Rust**: nightly-2025-08-04 toolchain
- **Git**: For version control
- **Docker**: For EVM tools and testing
- **Node.js**: For web dashboard development (optional)

### Installation
```bash
# Install Rust nightly
rustup install nightly-2025-08-04
rustup component add rust-src --toolchain nightly-2025-08-04
rustup default nightly-2025-08-04

# Install pico-cli
cargo +nightly-2025-08-04 install --git https://github.com/brevis-network/pico pico-cli

# Clone the toolkit
git clone https://github.com/SemiuAdesina/pico-community-toolkit.git
cd pico-community-toolkit
```

## Development Environment Setup

### 1. Install Development Dependencies
```bash
# Install all toolkit components
./install.sh

# Or install individual components
cd backend-comparison && cargo build
cd ../cli-extensions && cargo build
# ... repeat for other components
```

### 2. Verify Installation
```bash
# Test CLI tools
pico-compare --help
pico-ext --help
pico-debug --help
pico-evm --help
pico-test --help
pico-monitor --help
pico-market --help
```

### 3. Run Tests
```bash
# Run all tests
cargo test --workspace

# Run tests for specific component
cd backend-comparison && cargo test
```

## Project Structure

```
pico-community-toolkit/
├── docs/                          # Documentation
│   ├── architecture.md            # System architecture
│   ├── development.md             # This file
│   ├── api-reference.md           # API documentation
│   └── contributing.md            # Contribution guidelines
├── backend-comparison/            # Backend comparison tool
├── cli-extensions/               # CLI extensions
├── debugger/                     # Debugger and profiler
├── evm-helper/                   # EVM integration helper
├── testing-framework/            # Testing framework
├── monitoring-dashboard/         # Monitoring dashboard
├── proof-marketplace/            # Proof marketplace
├── coprocessor-registry/         # Coprocessor registry
├── learning-academy/             # Learning academy
├── install.sh                    # Installation script
├── .gitignore                    # Git ignore rules
└── README.md                     # Main documentation
```

## Development Workflow

### 1. Feature Development
```bash
# Create feature branch
git checkout -b feature/your-feature-name

# Make changes
# ... edit code ...

# Run tests
cargo test --workspace

# Format code
cargo fmt

# Lint code
cargo clippy

# Commit changes
git add .
git commit -m "feat: add your feature description"
```

### 2. Component Development
Each component follows a standard structure:

```
component-name/
├── Cargo.toml                    # Rust package configuration
├── README.md                     # Component documentation
├── src/
│   ├── main.rs                   # CLI entry point
│   ├── lib.rs                    # Library interface
│   └── ...                       # Additional modules
└── tests/                        # Test files
```

### 3. Testing Guidelines

#### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_name() {
        // Test implementation
        assert_eq!(expected, actual);
    }
}
```

#### Integration Tests
```rust
// tests/integration_test.rs
use pico_toolkit::component_name;

#[test]
fn test_component_integration() {
    // Integration test implementation
}
```

#### Performance Tests
```rust
#[test]
#[ignore] // Run with: cargo test --ignored
fn test_performance() {
    // Performance test implementation
    let start = std::time::Instant::now();
    // ... test code ...
    let duration = start.elapsed();
    assert!(duration.as_millis() < 1000); // 1 second limit
}
```

## Coding Standards

### Rust Code Style
- Follow Rust's official style guide
- Use `cargo fmt` for formatting
- Use `cargo clippy` for linting
- Prefer explicit over implicit
- Use meaningful variable and function names

### Documentation
- Document all public APIs
- Use doc comments (`///`) for functions
- Include examples in documentation
- Keep README files up to date

### Error Handling
```rust
// Use Result<T, E> for fallible operations
fn parse_input(input: &str) -> Result<ParsedInput, ParseError> {
    // Implementation
}

// Use anyhow for application errors
use anyhow::{Result, Context};

fn process_data(data: &[u8]) -> Result<ProcessedData> {
    let parsed = parse_input(&String::from_utf8(data.to_vec())?)
        .context("Failed to parse input data")?;
    Ok(ProcessedData::new(parsed))
}
```

## Component-Specific Guidelines

### CLI Tools
- Use `clap` for argument parsing
- Provide comprehensive help text
- Include examples in help output
- Support both short and long flags

```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "pico-tool")]
#[command(about = "Description of the tool")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Command description
    CommandName {
        /// Argument description
        #[arg(short, long)]
        argument: String,
    },
}
```

### Web Services
- Use `axum` for HTTP services
- Implement proper error handling
- Add request/response logging
- Include health check endpoints

```rust
use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};

async fn health_check() -> (StatusCode, Json<serde_json::Value>) {
    (StatusCode::OK, Json(serde_json::json!({"status": "healthy"})))
}

fn create_router() -> Router {
    Router::new().route("/health", get(health_check))
}
```

### Testing Tools
- Support multiple test types
- Provide detailed reporting
- Include performance benchmarks
- Support parallel test execution

## Debugging

### Local Debugging
```bash
# Run with debug output
RUST_LOG=debug cargo run --bin pico-tool

# Run with specific log level
RUST_LOG=info cargo run --bin pico-tool

# Run with backtrace on panic
RUST_BACKTRACE=1 cargo run --bin pico-tool
```

### IDE Setup
Recommended VS Code extensions:
- rust-analyzer
- CodeLLDB (for debugging)
- Better TOML
- GitLens

### Profiling
```bash
# Install profiler
cargo install flamegraph

# Generate flamegraph
cargo flamegraph --bin pico-tool -- args

# Memory profiling
cargo install cargo-valgrind
cargo valgrind test
```

## Performance Optimization

### Build Optimization
```toml
# Cargo.toml
[profile.release]
lto = true
codegen-units = 1
panic = "abort"
strip = true
```

### Runtime Optimization
- Use `release` builds for performance testing
- Profile with `cargo flamegraph`
- Optimize hot paths
- Use appropriate data structures

### Memory Optimization
- Avoid unnecessary allocations
- Use `Box` for large structs
- Consider `Arc<T>` for shared ownership
- Use `Cow<str>` for string handling

## Release Process

### 1. Version Bumping
```bash
# Update version in Cargo.toml files
# Update CHANGELOG.md
# Update README.md if needed
```

### 2. Testing
```bash
# Run full test suite
cargo test --workspace --release

# Run integration tests
cargo test --workspace --ignored

# Run benchmarks
cargo bench --workspace
```

### 3. Release
```bash
# Create release tag
git tag -a v0.1.0 -m "Release v0.1.0"
git push origin v0.1.0

# Publish to crates.io (if applicable)
cargo publish --workspace
```

## Troubleshooting

### Common Issues

#### Build Failures
```bash
# Clean and rebuild
cargo clean
cargo build

# Update dependencies
cargo update

# Check Rust version
rustc --version
```

#### Test Failures
```bash
# Run tests with more output
cargo test --workspace -- --nocapture

# Run specific test
cargo test test_name

# Run ignored tests
cargo test --workspace --ignored
```

#### Performance Issues
```bash
# Profile the application
cargo flamegraph --bin component-name

# Check for memory leaks
cargo test --workspace --release -- --test-threads=1
```

## Getting Help

- **Documentation**: Check component README files
- **Issues**: Create GitHub issues for bugs or questions
- **Discussions**: Use GitHub Discussions for general questions
- **Discord**: Join the Brevis Network Discord for real-time help
- **Twitter**: Follow @brevis_zk for updates

## Contributing

See [CONTRIBUTING.md](contributing.md) for detailed contribution guidelines.

---

*This development guide is a living document. Please contribute improvements and updates as the toolkit evolves.*
