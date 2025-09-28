#!/bin/bash

# Pico Community Toolkit Installation Script
# This script installs all components of the Pico Community Toolkit

set -e

echo "Installing Pico Community Toolkit"
echo "================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if Rust is installed
check_rust() {
    print_status "Checking Rust installation..."
    if ! command -v rustc &> /dev/null; then
        print_error "Rust is not installed. Please install Rust first."
        print_status "Visit: https://rustup.rs/"
        exit 1
    fi
    
    local rust_version=$(rustc --version)
    print_success "Rust found: $rust_version"
}

# Check if required Rust toolchain is installed
check_rust_toolchain() {
    print_status "Checking Rust toolchain..."
    
    if ! rustup toolchain list | grep -q "nightly-2025-08-04"; then
        print_warning "Required Rust toolchain not found. Installing nightly-2025-08-04..."
        rustup install nightly-2025-08-04
        rustup component add rust-src --toolchain nightly-2025-08-04
        print_success "Rust toolchain installed"
    else
        print_success "Required Rust toolchain found"
    fi
}

# Check if pico-cli is installed
check_pico_cli() {
    print_status "Checking pico-cli installation..."
    
    if ! command -v cargo-pico &> /dev/null; then
        print_warning "pico-cli not found. Installing..."
        cargo +nightly-2025-08-04 install --git https://github.com/brevis-network/pico pico-cli
        print_success "pico-cli installed"
    else
        print_success "pico-cli found"
    fi
}

# Install backend comparison tool
install_backend_comparison() {
    print_status "Installing Backend Comparison Tool..."
    cd backend-comparison
    
    if cargo build --release; then
        print_success "Backend Comparison Tool installed"
    else
        print_error "Failed to install Backend Comparison Tool"
        return 1
    fi
    
    cd ..
}

# Install CLI extensions
install_cli_extensions() {
    print_status "Installing CLI Extensions..."
    cd cli-extensions
    
    if cargo build --release; then
        print_success "CLI Extensions installed"
    else
        print_error "Failed to install CLI Extensions"
        return 1
    fi
    
    cd ..
}

# Install coprocessor registry
install_coprocessor_registry() {
    print_status "Installing Coprocessor Registry..."
    cd coprocessor-registry
    
    if cargo build --release; then
        print_success "Coprocessor Registry installed"
    else
        print_error "Failed to install Coprocessor Registry"
        return 1
    fi
    
    cd ..
}

# Create symlinks for easy access
create_symlinks() {
    print_status "Creating symlinks for easy access..."
    
    local target_dir="$HOME/.cargo/bin"
    
    # Create symlinks if they don't exist
    if [ -f "backend-comparison/target/release/pico-compare" ]; then
        ln -sf "$(pwd)/backend-comparison/target/release/pico-compare" "$target_dir/pico-compare" 2>/dev/null || true
    fi
    
    if [ -f "cli-extensions/target/release/pico-ext" ]; then
        ln -sf "$(pwd)/cli-extensions/target/release/pico-ext" "$target_dir/pico-ext" 2>/dev/null || true
    fi
    
    print_success "Symlinks created"
}

# Verify installation
verify_installation() {
    print_status "Verifying installation..."
    
    local tools=("pico-compare" "pico-ext")
    local all_installed=true
    
    for tool in "${tools[@]}"; do
        if command -v "$tool" &> /dev/null; then
            print_success "$tool is available"
        else
            print_warning "$tool is not available"
            all_installed=false
        fi
    done
    
    if [ "$all_installed" = true ]; then
        print_success "All tools installed successfully!"
    else
        print_warning "Some tools may not be available. Check your PATH."
    fi
}

# Main installation function
main() {
    echo "Starting installation process..."
    echo
    
    # Check prerequisites
    check_rust
    check_rust_toolchain
    check_pico_cli
    
    echo
    
    # Install components
    install_backend_comparison
    install_cli_extensions
    install_coprocessor_registry
    
    echo
    
    # Create symlinks and verify
    create_symlinks
    verify_installation
    
    echo
    print_success "Pico Community Toolkit installation complete!"
    echo
    echo "Available tools:"
    echo "  - pico-compare  - Compare proving backends"
    echo "  - pico-ext      - Extended CLI tools"
    echo
    echo "Next steps:"
    echo "  1. Try: pico-compare --help"
    echo "  2. Try: pico-ext --help"
    echo "  3. Explore the learning academy: cd learning-academy"
    echo "  4. Check out examples: cd coprocessor-registry/examples"
    echo
    echo "Happy building with Pico zkVM!"
}

# Run main function
main "$@"
