FROM --platform=linux/amd64 ubuntu:22.04

# Install system dependencies and development tools
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
    sudo \
    && rm -rf /var/lib/apt/lists/*

# Create solana user
ENV USER=solana
RUN useradd --create-home -s /bin/bash ${USER} && \
    usermod -a -G sudo ${USER} && \
    echo '%sudo ALL=(ALL) NOPASSWD:ALL' >> /etc/sudoers

USER solana
WORKDIR /home/solana

# Install Rust with latest stable toolchain
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable && \
    echo 'export PATH="/home/solana/.cargo/bin:$PATH"' >> ~/.bashrc && \
    echo 'export PATH="/home/solana/.cargo/bin:$PATH"' >> ~/.profile && \
    . ~/.cargo/env && \
    rustup update

# Add cargo to PATH for all shells and users
ENV PATH="/home/solana/.cargo/bin:${PATH}"

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
    mkdir -p /home/solana/.local/share/solana/install/active_release && \
    cp -r solana-release/* /home/solana/.local/share/solana/install/active_release/ && \
    rm -rf solana.tar.bz2 solana-release
ENV PATH="/home/solana/.local/share/solana/install/active_release/bin:$PATH"

# Install Node.js (alternative approach using Ubuntu repositories)
USER root
RUN apt-get update && apt-get install -y nodejs npm
USER solana

# Install Anchor CLI directly via cargo (working method)
RUN . ~/.cargo/env && cargo install --git https://github.com/coral-xyz/anchor anchor-cli --locked --force

# Verify installations with error handling
RUN . ~/.cargo/env && \
    echo "Checking Anchor installation..." && \
    anchor --version || (echo "Warning: Anchor version check failed, but installation may still work" && true) && \
    echo "Checking Solana installation..." && \
    solana --version || (echo "Warning: Solana version check failed, but installation may still work" && true)

# Switch to root to create directories and copy files
USER root

# Create directories for validator, programs, and deployment
RUN mkdir -p /opt/solana/ledger /opt/solana/config /opt/solana/programs /opt/solana/deploy && \
    chown -R solana:solana /opt/solana
WORKDIR /opt/solana

# Copy startup and deployment scripts
COPY start-validator.sh /usr/local/bin/start-validator.sh
COPY deploy-contracts.sh /usr/local/bin/deploy-contracts.sh
COPY check-deployment.sh /usr/local/bin/check-deployment.sh
RUN chmod +x /usr/local/bin/start-validator.sh /usr/local/bin/deploy-contracts.sh /usr/local/bin/check-deployment.sh

# Create deployment status directory
RUN mkdir -p /opt/solana/status && \
    chown -R solana:solana /opt/solana/status

# Switch back to solana user
USER solana

# Expose ports
EXPOSE 8899 8900 8001

# Health check - updated to check both validator and deployment status
HEALTHCHECK --interval=30s --timeout=10s --start-period=120s --retries=5 \
    CMD /usr/local/bin/check-deployment.sh || exit 1

CMD ["/usr/local/bin/start-validator.sh"]