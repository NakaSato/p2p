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
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/* \
    && rm -rf /tmp/* \
    && rm -rf /var/tmp/*

# Install cargo-contract for ink! smart contracts
RUN cargo install --force --locked cargo-contract --version 4.1.1

# Install substrate-contracts-node
RUN cargo install --git https://github.com/paritytech/substrate-contracts-node.git --tag v0.32.0 --force --locked

# Set working directory
WORKDIR /contracts

# Copy contract source files
COPY . .

# Build all contracts
RUN cd contracts/registry && cargo contract build --release
RUN cd contracts/grid-token && cargo contract build --release  
RUN cd contracts/trading && cargo contract build --release
RUN cd contracts/oracle-client && cargo contract build --release

# Create deployment script
COPY deploy_contracts.sh /contracts/
RUN chmod +x deploy_contracts.sh

EXPOSE 9944 9933 30333

CMD ["./deploy_contracts.sh"]
