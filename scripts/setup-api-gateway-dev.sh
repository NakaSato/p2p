#!/bin/bash

# API Gateway Development Environment Setup Script
# P2P Energy Trading System - Engineering Department

set -e

echo "🔧 Setting up API Gateway development environment..."

# Check if Rust is installed
if ! command -v rustc &> /dev/null; then
    echo "❌ Rust is not installed. Please install Rust first:"
    echo "   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

# Check Rust version
RUST_VERSION=$(rustc --version | cut -d' ' -f2)
echo "✅ Rust version: $RUST_VERSION"

# Ensure we have the latest stable toolchain
echo "🔄 Updating Rust toolchain..."
rustup update stable
rustup default stable

# Install required components
echo "📦 Installing required Rust components..."
rustup component add rustfmt
rustup component add clippy
rustup component add llvm-tools-preview

# Install cargo tools for development
echo "🛠️  Installing development tools..."
cargo install cargo-watch --version 8.4.0 || echo "cargo-watch already installed"
cargo install cargo-audit --version 0.18.3 || echo "cargo-audit already installed"
cargo install sqlx-cli --no-default-features --features rustls,postgres || echo "sqlx-cli already installed"

# Check if Docker is running
if ! docker info &> /dev/null; then
    echo "⚠️  Docker is not running. Please start Docker Desktop."
    echo "   Database and Redis will not be available without Docker."
else
    echo "✅ Docker is running"
fi

# Check if PostgreSQL tools are available
if ! command -v psql &> /dev/null; then
    echo "⚠️  PostgreSQL client (psql) not found. Installing..."
    if [[ "$OSTYPE" == "darwin"* ]]; then
        # macOS
        if command -v brew &> /dev/null; then
            brew install postgresql@15
        else
            echo "❌ Homebrew not found. Please install PostgreSQL manually."
        fi
    elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
        # Linux
        sudo apt-get update && sudo apt-get install -y postgresql-client-15
    fi
fi

# Set up environment file
if [ ! -f "api-gateway/.env" ]; then
    echo "📝 Creating environment file..."
    cp .env.example api-gateway/.env
    echo "⚠️  Please review and update api-gateway/.env with your configuration"
fi

# Create logs directory
mkdir -p logs/api-gateway

# Verify cargo build works
echo "🔨 Testing cargo build..."
cd api-gateway
if cargo check; then
    echo "✅ Cargo build check passed"
else
    echo "❌ Cargo build check failed"
    exit 1
fi

# Run cargo clippy for linting
echo "🔍 Running cargo clippy..."
cargo clippy --all-targets --all-features -- -D warnings || echo "⚠️  Clippy warnings found"

# Format code
echo "🎨 Formatting code..."
cargo fmt

echo ""
echo "🎉 API Gateway development environment setup complete!"
echo ""
echo "Next steps:"
echo "1. Review and update api-gateway/.env"
echo "2. Start database: docker-compose up -d postgres redis"
echo "3. Run database migrations: cd api-gateway && sqlx migrate run"
echo "4. Start development server: cargo run"
echo "5. Or use watch mode: cargo watch -x run"
echo ""
echo "Available commands:"
echo "  cargo run                    # Start the server"
echo "  cargo test                   # Run tests"
echo "  cargo watch -x run          # Auto-reload on changes"
echo "  cargo audit                  # Security audit"
echo "  cargo clippy                 # Linting"
echo "  cargo fmt                    # Code formatting"