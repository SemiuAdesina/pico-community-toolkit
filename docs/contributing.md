# Contributing Guide

Thank you for your interest in contributing to the Pico Community Toolkit! This guide will help you get started with contributing to our open-source project.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Contribution Process](#contribution-process)
- [Coding Standards](#coding-standards)
- [Testing Guidelines](#testing-guidelines)
- [Documentation](#documentation)
- [Release Process](#release-process)
- [Community Guidelines](#community-guidelines)

## Code of Conduct

This project follows the [Contributor Covenant Code of Conduct](https://www.contributor-covenant.org/version/2/1/code_of_conduct/). By participating, you agree to uphold this code.

### Our Pledge
We pledge to make participation in our project a harassment-free experience for everyone, regardless of age, body size, disability, ethnicity, gender identity and expression, level of experience, education, socio-economic status, nationality, personal appearance, race, religion, or sexual identity and orientation.

## Getting Started

### Prerequisites
- Rust nightly-2025-08-04 or later
- Git
- Basic understanding of zero-knowledge proofs and Pico zkVM
- Familiarity with Rust programming language

### First Steps
1. **Fork the repository** on GitHub
2. **Clone your fork** locally
3. **Set up your development environment**
4. **Make your changes**
5. **Test your changes**
6. **Submit a pull request**

## Development Setup

### 1. Fork and Clone
```bash
# Fork the repository on GitHub, then:
git clone https://github.com/YOUR_USERNAME/pico-community-toolkit.git
cd pico-community-toolkit

# Add upstream remote
git remote add upstream https://github.com/SemiuAdesina/pico-community-toolkit.git
```

### 2. Install Dependencies
```bash
# Install Rust nightly
rustup install nightly-2025-08-04
rustup component add rust-src --toolchain nightly-2025-08-04
rustup default nightly-2025-08-04

# Install pico-cli
cargo +nightly-2025-08-04 install --git https://github.com/brevis-network/pico pico-cli

# Install toolkit components
./install.sh
```

### 3. Verify Setup
```bash
# Run tests
cargo test --workspace

# Run linting
cargo clippy --workspace

# Format code
cargo fmt --workspace
```

## Contribution Process

### 1. Choose What to Work On

#### Good First Issues
Look for issues labeled with:
- `good first issue`
- `help wanted`
- `documentation`

#### Component Areas
- **Backend Comparison**: Performance benchmarking tools
- **CLI Extensions**: Command-line utilities
- **Debugger**: Debugging and profiling tools
- **EVM Integration**: Ethereum deployment tools
- **Testing Framework**: Testing and validation tools
- **Monitoring**: Performance monitoring
- **Proof Marketplace**: Decentralized proving
- **Coprocessor Registry**: Specialized circuits
- **Learning Academy**: Educational resources

### 2. Create a Branch
```bash
# Create feature branch
git checkout -b feature/your-feature-name

# Or bugfix branch
git checkout -b bugfix/issue-number-description

# Or documentation branch
git checkout -b docs/update-readme
```

### 3. Make Your Changes
- Write clean, readable code
- Follow Rust best practices
- Add tests for new functionality
- Update documentation as needed
- Keep commits focused and atomic

### 4. Test Your Changes
```bash
# Run all tests
cargo test --workspace

# Run specific component tests
cd backend-comparison && cargo test

# Run with verbose output
cargo test --workspace -- --nocapture

# Run performance tests
cargo test --workspace --ignored
```

### 5. Submit Pull Request
```bash
# Push your branch
git push origin feature/your-feature-name

# Create pull request on GitHub
```

## Coding Standards

### Rust Style Guide
- Follow the [official Rust style guide](https://doc.rust-lang.org/1.0.0/style/README.html)
- Use `cargo fmt` for formatting
- Use `cargo clippy` for linting
- Maximum line length: 100 characters

### Naming Conventions
- Use `snake_case` for variables, functions, and modules
- Use `PascalCase` for types and traits
- Use `SCREAMING_SNAKE_CASE` for constants
- Use descriptive names that explain intent

### Code Organization
```rust
// 1. External imports
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

// 2. Internal imports
use crate::types::MyType;
use super::helper_function;

// 3. Public API
pub struct MyStruct {
    // Fields
}

impl MyStruct {
    // Methods
}

// 4. Private implementation
impl MyStruct {
    fn private_method(&self) {
        // Implementation
    }
}

// 5. Tests
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_function() {
        // Test implementation
    }
}
```

### Error Handling
```rust
// Use Result<T, E> for fallible operations
fn parse_data(input: &str) -> Result<ParsedData, ParseError> {
    // Implementation
}

// Use anyhow for application errors
use anyhow::{Result, Context};

fn process_file(path: &Path) -> Result<()> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read file: {}", path.display()))?;
    
    // Process content
    Ok(())
}

// Use custom error types for library code
#[derive(Debug, thiserror::Error)]
pub enum MyError {
    #[error("Invalid input: {input}")]
    InvalidInput { input: String },
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}
```

## Testing Guidelines

### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_name() {
        // Arrange
        let input = "test input";
        let expected = "expected output";
        
        // Act
        let result = function_under_test(input);
        
        // Assert
        assert_eq!(result, expected);
    }

    #[test]
    #[should_panic(expected = "Invalid input")]
    fn test_invalid_input_panics() {
        function_under_test("invalid");
    }
}
```

### Integration Tests
```rust
// tests/integration_test.rs
use pico_toolkit::component_name;

#[test]
fn test_component_integration() {
    // Integration test implementation
}
```

### Performance Tests
```rust
#[test]
#[ignore] // Run with: cargo test --ignored
fn test_performance() {
    let start = std::time::Instant::now();
    
    // Performance-critical code
    
    let duration = start.elapsed();
    assert!(duration.as_millis() < 1000); // 1 second limit
}
```

### Property-Based Tests
```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_property(input in any::<u32>()) {
        let result = function_under_test(input);
        assert!(result >= 0);
    }
}
```

## Documentation

### Code Documentation
```rust
/// Brief description of what the function does.
///
/// Longer description if needed, explaining the purpose,
/// behavior, and any important details.
///
/// # Arguments
///
/// * `input` - Description of the input parameter
/// * `options` - Description of the options parameter
///
/// # Returns
///
/// Description of what is returned, including error conditions.
///
/// # Examples
///
/// ```
/// let result = function_name("input", &options);
/// assert_eq!(result, expected_value);
/// ```
///
/// # Errors
///
/// This function will return an error if:
/// - The input is invalid
/// - The operation times out
pub fn function_name(input: &str, options: &Options) -> Result<Output, Error> {
    // Implementation
}
```

### README Updates
When adding new features:
1. Update the component's README.md
2. Add examples and usage instructions
3. Update the main README.md if needed
4. Include screenshots or diagrams if helpful

### API Documentation
- Document all public APIs
- Include examples in doc comments
- Keep documentation up to date with code changes
- Use `cargo doc` to generate and review documentation

## Release Process

### Version Numbering
We follow [Semantic Versioning](https://semver.org/):
- **MAJOR**: Incompatible API changes
- **MINOR**: New functionality (backward compatible)
- **PATCH**: Bug fixes (backward compatible)

### Release Checklist
- [ ] All tests pass
- [ ] Documentation is updated
- [ ] CHANGELOG.md is updated
- [ ] Version numbers are bumped
- [ ] Release notes are written
- [ ] Release is tagged and published

### Creating a Release
```bash
# Update version in Cargo.toml files
# Update CHANGELOG.md
# Run full test suite
cargo test --workspace --release

# Create release tag
git tag -a v0.1.0 -m "Release v0.1.0"
git push origin v0.1.0

# Create GitHub release
# Upload binaries and documentation
```

## Community Guidelines

### Communication Channels
- **GitHub Issues**: Bug reports and feature requests
- **GitHub Discussions**: General questions and community discussion
- **Discord**: Real-time chat and quick questions
- **Twitter**: Updates and announcements

### Reporting Issues
When reporting issues, please include:
1. **Clear description** of the problem
2. **Steps to reproduce** the issue
3. **Expected behavior** vs actual behavior
4. **Environment details** (OS, Rust version, etc.)
5. **Minimal reproduction case** if possible

### Requesting Features
When requesting features, please include:
1. **Use case** and motivation
2. **Proposed solution** or design
3. **Alternative solutions** considered
4. **Additional context** and examples

### Code Reviews
When reviewing code:
- Be constructive and respectful
- Focus on the code, not the person
- Suggest improvements and alternatives
- Ask questions to understand intent
- Approve when ready, request changes when needed

### Mentoring
- Experienced contributors are encouraged to mentor newcomers
- Help with code reviews and guidance
- Answer questions in discussions and Discord
- Share knowledge and best practices

## Recognition

### Contributors
All contributors are recognized in:
- GitHub contributors page
- Release notes
- Project documentation
- Community announcements

### Types of Contributions
We value all types of contributions:
- **Code**: Bug fixes, new features, performance improvements
- **Documentation**: README updates, API docs, tutorials
- **Testing**: Test cases, bug reports, quality assurance
- **Community**: Helping others, organizing events, promoting the project
- **Design**: UI/UX improvements, logos, graphics

## Getting Help

### Resources
- [Rust Book](https://doc.rust-lang.org/book/) - Learn Rust
- [Pico Documentation](https://docs.brevis.network/) - Learn Pico zkVM
- [Zero-Knowledge Proofs](https://z.cash/technology/zksnarks/) - Learn ZK concepts

### Support
- Create a GitHub issue for bugs
- Start a GitHub discussion for questions
- Join Discord for real-time help
- Follow Twitter for updates

## License

By contributing to this project, you agree that your contributions will be licensed under the same license as the project (MIT License).

## Thank You

Thank you for contributing to the Pico Community Toolkit! Your contributions help make zero-knowledge development more accessible and powerful for everyone.

---

*This contributing guide is a living document. Please suggest improvements and updates as the project evolves.*
