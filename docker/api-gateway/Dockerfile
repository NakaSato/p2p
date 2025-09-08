# Use multi-stage build for optimization
FROM rust:1.82-alpine AS builder

# Install system dependencies
RUN apk add --no-cache \
    musl-dev \
    pkgconfig \
    openssl-dev \
    openssl-libs-static

# Create app directory
WORKDIR /app

# Set environment variables for static linking
ENV RUSTFLAGS="-C target-feature=+crt-static"
ENV TARGET=x86_64-unknown-linux-musl

# Add the target
RUN rustup target add $TARGET

# Copy Cargo files
COPY api-gateway/Cargo.toml ./
COPY api-gateway/Cargo.loc[k] ./

# Create a dummy main.rs to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies (this layer will be cached)
RUN cargo build --release --target $TARGET
RUN rm src/main.rs

# Copy source code
COPY api-gateway/src ./src

# Build the application
RUN cargo build --release --target $TARGET

# Runtime stage
FROM alpine:3.20

# Install CA certificates and other runtime dependencies
RUN apk add --no-cache \
    ca-certificates \
    curl

# Create app user
RUN addgroup -g 1000 appuser && \
    adduser -u 1000 -G appuser -s /bin/sh -D appuser

# Create app directory
WORKDIR /app

# Copy the binary from builder stage
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/p2p-api-gateway .

# Change ownership to app user
RUN chown -R appuser:appuser /app
USER appuser

# Expose port
EXPOSE 3000

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:3000/health || exit 1

# Set environment variables
ENV RUST_LOG=info
ENV PORT=3000

# Run the application
CMD ["./p2p-api-gateway"]
