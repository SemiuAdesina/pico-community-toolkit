# Pico Coprocessor Registry

A community-maintained registry of optimized coprocessors and precompiles for Pico zkVM. These specialized circuits accelerate cryptographic operations and domain-specific computations.

## 🔧 Available Coprocessors

### Cryptographic Hash Functions
- **Keccak256**: SHA-3 hash function optimized for zkVM
- **Poseidon**: ZK-friendly hash function
- **Blake3**: High-performance hash function
- **SHA256**: Standard SHA-2 hash function

### Elliptic Curve Operations
- **Ed25519**: Edwards curve digital signature algorithm
- **Secp256k1**: Bitcoin-compatible signature verification
- **BLS12-381**: Pairing-friendly elliptic curve
- **Curve25519**: High-performance elliptic curve

### Advanced Cryptography
- **Merkle Tree**: Efficient Merkle tree operations
- **Range Proofs**: Zero-knowledge range proofs
- **Set Membership**: Efficient set membership proofs
- **Polynomial Commitments**: KZG and other commitment schemes

## 📦 Installation

### Using Cargo
```toml
[dependencies]
pico-coprocessor-registry = { git = "https://github.com/YOUR_USERNAME/pico-community-toolkit" }
```

### Individual Coprocessors
```toml
[dependencies]
pico-keccak256 = { git = "https://github.com/YOUR_USERNAME/pico-community-toolkit", package = "keccak256" }
pico-poseidon = { git = "https://github.com/YOUR_USERNAME/pico-community-toolkit", package = "poseidon" }
```

## 🚀 Quick Start

### Basic Usage
```rust
use pico_coprocessor_registry::keccak256::Keccak256;
use pico_sdk::io::{read_vec, commit_bytes};

fn main() {
    // Read input data
    let data: Vec<u8> = read_vec();
    
    // Compute Keccak256 hash
    let hash = Keccak256::hash(&data);
    
    // Commit the result
    commit_bytes(&hash);
}
```

### Advanced Usage
```rust
use pico_coprocessor_registry::{
    keccak256::Keccak256,
    merkle_tree::MerkleTree,
    ed25519::Ed25519,
};

fn main() {
    // Hash multiple inputs
    let inputs = vec![b"hello", b"world", b"zkvm"];
    let hashes: Vec<[u8; 32]> = inputs
        .iter()
        .map(|input| Keccak256::hash(input))
        .collect();
    
    // Build Merkle tree
    let tree = MerkleTree::new(hashes);
    let root = tree.root();
    
    // Verify signature
    let message = b"important message";
    let signature = read_signature();
    let public_key = read_public_key();
    
    let is_valid = Ed25519::verify(&public_key, message, &signature);
    
    // Commit results
    commit_bytes(&root);
    commit_bytes(&[is_valid as u8]);
}
```

## 📊 Performance Benchmarks

| Coprocessor | Cycles | Memory | Backend | Use Case |
|-------------|--------|--------|---------|----------|
| Keccak256 | 1,024 | 2KB | KoalaBear | General hashing |
| Poseidon | 512 | 1KB | BabyBear | ZK-friendly hashing |
| Ed25519 | 2,048 | 4KB | KoalaBear | Signature verification |
| Merkle Tree | 4,096 | 8KB | BabyBear | Tree operations |
| BLS12-381 | 8,192 | 16KB | KoalaBear | Pairing operations |

## 🛠️ Development

### Adding New Coprocessors
1. Create a new directory under `coprocessors/`
2. Implement the coprocessor interface
3. Add comprehensive tests
4. Update the registry

### Testing
```bash
# Run all tests
cargo test

# Test specific coprocessor
cargo test -p keccak256

# Benchmark performance
cargo bench
```

## 📚 Documentation

- [API Reference](docs/api-reference.md)
- [Performance Guide](docs/performance-guide.md)
- [Security Considerations](docs/security.md)
- [Contributing Guide](docs/contributing.md)

## 🤝 Contributing

We welcome contributions! Whether you're adding new coprocessors, optimizing existing ones, or improving documentation, your contributions help the community.

### How to Contribute
1. Fork the repository
2. Create a new branch
3. Implement your coprocessor
4. Add tests and documentation
5. Submit a pull request

### Guidelines
- Follow Rust best practices
- Include comprehensive tests
- Document all public APIs
- Provide performance benchmarks
- Consider security implications

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Brevis Network for the Pico zkVM
- The cryptographic community for research and implementations
- All contributors who make this registry possible

---

**Built for the Pico zkVM community**
