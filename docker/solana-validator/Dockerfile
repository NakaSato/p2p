FROM ubuntu:22.04

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

# Install Solana CLI by downloading specific version manually  
RUN curl -sSfL https://github.com/solana-labs/solana/releases/download/v1.18.17/solana-release-x86_64-unknown-linux-gnu.tar.bz2 -o solana.tar.bz2 && \
    tar -xjf solana.tar.bz2 && \
    mkdir -p /home/solana/.local/share/solana/install/active_release && \
    cp -r solana-release/* /home/solana/.local/share/solana/install/active_release/ && \
    rm -rf solana.tar.bz2 solana-release
ENV PATH="/home/solana/.local/share/solana/install/active_release/bin:$PATH"

# Install Node.js
RUN curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash - && \
    sudo apt-get install -y nodejs

# Install Anchor CLI directly via cargo (working method)
RUN . ~/.cargo/env && cargo install --git https://github.com/coral-xyz/anchor anchor-cli --locked --force

# Verify Anchor installation
RUN . ~/.cargo/env && anchor --version

# Switch to root to create directories and copy files
USER root

# Create directories
RUN mkdir -p /opt/solana/ledger /opt/solana/config && \
    chown -R solana:solana /opt/solana
WORKDIR /opt/solana

# Copy startup script
COPY start-validator.sh /usr/local/bin/start-validator.sh
RUN chmod +x /usr/local/bin/start-validator.sh

# Switch back to solana user
USER solana

# Expose ports
EXPOSE 8899 8900 8001

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=60s --retries=5 \
    CMD solana cluster-version --url http://localhost:8899 || exit 1

CMD ["/usr/local/bin/start-validator.sh"]