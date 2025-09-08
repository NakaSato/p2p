FROM rust:1.83-bookworm

# Install dependencies with security updates
RUN apt-get update && apt-get upgrade -y && apt-get install -y \
    build-essential \
    clang \
    cmake \
    curl \
    git \
    libssl-dev \
    llvm \
    pkg-config \
    protobuf-compiler \
    wget \
    ca-certificates \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/* \
    && rm -rf /tmp/* \
    && rm -rf /var/tmp/*

# Install cargo-contract for ink! smart contracts
RUN cargo install --force --locked cargo-contract --version 4.1.1

# Install WASM target required for substrate-contracts-node (with retry logic)
RUN for i in 1 2 3; do \
        rustup target add wasm32-unknown-unknown && break || \
        (echo "Attempt $i failed, retrying..." && sleep 10); \
    done

# Download pre-built substrate-contracts-node binary (faster than building from source)
RUN wget -O substrate-contracts-node-linux.tar.gz \
    https://github.com/paritytech/substrate-contracts-node/releases/download/v0.32.0/substrate-contracts-node-linux.tar.gz && \
    tar -xzf substrate-contracts-node-linux.tar.gz && \
    mv substrate-contracts-node /usr/local/bin/ && \
    chmod +x /usr/local/bin/substrate-contracts-node && \
    rm substrate-contracts-node-linux.tar.gz

# Set working directory
WORKDIR /contracts

# Copy the contract source files from the project root
COPY contracts ./contracts
COPY Cargo.toml ./

# Build all contracts
RUN cd contracts/registry && cargo contract build --release
RUN cd contracts/grid-token && cargo contract build --release  
RUN cd contracts/trading && cargo contract build --release
RUN cd contracts/oracle-client && cargo contract build --release

# Copy and setup deployment script
COPY docker/contracts/deploy_contracts.sh /contracts/
RUN chmod +x deploy_contracts.sh

EXPOSE 9944 9933 30333

CMD ["./deploy_contracts.sh"]
