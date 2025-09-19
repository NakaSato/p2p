FROM ubuntu:22.04

# Install system dependencies
RUN apt-get update && apt-get install -y \
    curl \
    ca-certificates \
    wget \
    bzip2 \
    pkg-config \
    libssl-dev \
    libudev-dev \
    build-essential \
    git \
    jq \
    sudo \
    && rm -rf /var/lib/apt/lists/*

# Create deployer user
ENV USER=deployer
RUN useradd --create-home -s /bin/bash ${USER}
USER ${USER}
WORKDIR /home/${USER}

# Install Rust with latest stable toolchain
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable && \
    echo 'export PATH="/home/deployer/.cargo/bin:$PATH"' >> ~/.bashrc && \
    echo 'export PATH="/home/deployer/.cargo/bin:$PATH"' >> ~/.profile && \
    . ~/.cargo/env && \
    rustup update

# Add cargo to PATH
ENV PATH="/home/deployer/.cargo/bin:${PATH}"

# Install Solana CLI with architecture detection
RUN ARCH=$(dpkg --print-architecture) && \
    if [ "$ARCH" = "amd64" ]; then \
        SOLANA_ARCH="x86_64-unknown-linux-gnu"; \
    elif [ "$ARCH" = "arm64" ]; then \
        SOLANA_ARCH="aarch64-unknown-linux-gnu"; \
    else \
        echo "Unsupported architecture: $ARCH" && exit 1; \
    fi && \
    curl -sSfL "https://github.com/solana-labs/solana/releases/download/v1.18.17/solana-release-$SOLANA_ARCH.tar.bz2" -o solana.tar.bz2 && \
    tar -xjf solana.tar.bz2 && \
    mkdir -p /home/deployer/.local/share/solana/install/active_release && \
    cp -r solana-release/* /home/deployer/.local/share/solana/install/active_release/ && \
    rm -rf solana.tar.bz2 solana-release
ENV PATH="/home/deployer/.local/share/solana/install/active_release/bin:$PATH"

# Install Node.js (alternative approach using Ubuntu repositories)
USER root
RUN apt-get update && apt-get install -y nodejs npm
USER deployer

# Install Anchor CLI
RUN . ~/.cargo/env && cargo install --git https://github.com/coral-xyz/anchor anchor-cli --locked --force

# Verify installations
RUN . ~/.cargo/env && anchor --version && solana --version

# Switch to root to create directories and copy files
USER root

# Create deployment directories
RUN mkdir -p /opt/deployer/workspace /opt/deployer/artifacts /opt/deployer/logs /opt/deployer/config && \
    chown -R deployer:deployer /opt/deployer

# Copy deployment scripts
COPY deploy-all-contracts.sh /usr/local/bin/deploy-all-contracts.sh
COPY build-contracts.sh /usr/local/bin/build-contracts.sh
COPY verify-deployment.sh /usr/local/bin/verify-deployment.sh
COPY wait-for-validator.sh /usr/local/bin/wait-for-validator.sh
COPY setup-poa.sh /usr/local/bin/setup-poa.sh
COPY health-monitor.sh /usr/local/bin/health-monitor.sh
RUN chmod +x /usr/local/bin/*.sh

# Switch back to deployer user
USER deployer
WORKDIR /opt/deployer

# Set environment variables
ENV ANCHOR_PROVIDER_URL=http://solana-validator:8899
ENV ANCHOR_WALLET=/opt/deployer/config/deployer-keypair.json
ENV RUST_LOG=info

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=60s --retries=3 \
    CMD /usr/local/bin/verify-deployment.sh || exit 1

# Default command
CMD ["/usr/local/bin/deploy-all-contracts.sh"]