# Rust API Gateway for P2P Energy Trading Platform

A high-performance, secure API Gateway built with Rust and Axum, providing RESTful endpoints for the P2P Energy Trading Platform.

## Features

- **High Performance**: Built with Rust and Axum for maximum throughput and minimal latency
- **Security**: Comprehensive security headers, CORS protection, and request validation
- **Observability**: Structured logging with tracing and metrics endpoint
- **Hot Reload**: Development environment with cargo-watch for fast iteration
- **Containerized**: Production-ready Docker setup with multi-stage builds
- **Auto-scaling**: Resource limits and health checks for production deployment

## Quick Start

### Option 1: Using Quick Start Script (Recommended)

```bash
# Navigate to the docker directory
cd docker/api-gateway

# Development with hot reload
./start.sh development

# Production mode  
./start.sh production
```

### Option 2: Using Docker Compose Directly

```bash
# Navigate to the docker directory
cd docker/api-gateway

# Development with hot reload
docker compose --profile dev up --build

# Production mode
docker compose --profile prod up --build

# Or use override files for more control
docker compose -f docker-compose.yml -f docker-compose.dev.yml up
docker compose -f docker-compose.yml -f docker-compose.prod.yml up
```

### Option 3: Traditional Setup

```bash
# Navigate to the docker directory
cd docker/api-gateway

# Copy environment variables
cp ../../api-gateway/.env.example ../../api-gateway/.env

# Create network
docker network create p2p-network

# Start the development server with hot reload
docker compose --profile dev up
```

The API will be available at `http://localhost:3000`

### Local Development (without Docker)

```bash
# Install dependencies
cargo build

# Run with hot reload
cargo install cargo-watch
cargo watch -x run

# Or run normally
cargo run
```

## Docker Architecture

### Multi-stage Alpine Production Build

```dockerfile
# Builder stage - compiles Rust code with static linking
FROM rust:1.82-alpine as builder
# ... optimized build process with musl ...

# Runtime stage - minimal Alpine Linux (only ~6.67MB!)
FROM alpine:3.20
# ... ultra-lightweight final image ...
```

### Development Setup

- **Hot Reload**: Uses `cargo-watch` for instant rebuilds
- **Volume Mounts**: Source code mounted for editing
- **Cargo Cache**: Persistent volume for faster rebuilds
- **Debug Logging**: Detailed logs for development

### Production Features

- **Ultra-Lightweight**: Alpine Linux base with only ~6.67MB final image size
- **Static Linking**: Fully static binary with musl libc for maximum compatibility
- **Security**: Non-root user, minimal attack surface, latest Alpine packages
- **Health Checks**: Built-in health monitoring
- **Resource Limits**: CPU and memory constraints
- **Structured Logging**: JSON logs optimized for monitoring

## Build Options

### Using Build Script

```bash
# Navigate to the docker directory
cd docker/api-gateway

# Development build
./build.sh --mode development

# Production build
./build.sh --mode production

# Build and push to registry
./build.sh --mode production --push

# Multi-platform build
./build.sh --platform linux/amd64,linux/arm64

# No cache build
./build.sh --no-cache
```

### Manual Docker Build

```bash
# From the project root directory
# Development image
docker build -f docker/api-gateway/Dockerfile.dev -t rust-api-gateway:dev .

# Production image
docker build -f docker/api-gateway/Dockerfile -t rust-api-gateway:prod .
```

## API Endpoints

### Health & Metrics
- `GET /health` - Health check endpoint
- `GET /metrics` - Application metrics

### User Management
- `GET /api/users` - List all users
- `GET /api/users/:id` - Get user by ID
- `POST /api/users` - Create new user
- `PUT /api/users/:id` - Update user
- `DELETE /api/users/:id` - Delete user

### Meter Management
- `GET /api/meters` - List all meters
- `GET /api/meters/:id` - Get meter by ID
- `POST /api/meters` - Create new meter
- `PUT /api/meters/:id` - Update meter
- `DELETE /api/meters/:id` - Delete meter

### Market & Trading
- `GET /api/market` - Get market data
- `GET /api/market/orders` - List all orders
- `GET /api/market/orders/:id` - Get order by ID
- `POST /api/market/orders` - Create new order
- `PUT /api/market/orders/:id` - Update order
- `DELETE /api/market/orders/:id` - Cancel order

## Request/Response Examples

### Create User
```bash
curl -X POST http://localhost:3000/api/users \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Alice Johnson",
    "email": "alice@example.com",
    "role": "producer"
  }'
```

### Get Market Data
```bash
curl http://localhost:3000/api/market
```

### Create Trading Order
```bash
curl -X POST http://localhost:3000/api/market/orders \
  -H "Content-Type: application/json" \
  -d '{
    "user_id": "1",
    "order_type": "sell",
    "amount": "10.5 kWh",
    "price": "0.12 USD/kWh"
  }'
```

## Configuration

### Environment Variables

Copy `.env.example` to `.env` and adjust:

```env
# Server Configuration
PORT=3000
HOST=0.0.0.0

# Logging Configuration
RUST_LOG=p2p_api_gateway=debug,tower_http=debug

# Database Configuration
DATABASE_URL=postgresql://user:password@localhost:5432/p2p_energy
REDIS_URL=redis://localhost:6379

# Security Configuration
JWT_SECRET=your-super-secret-jwt-key
```

### Docker Environment

Development and production environments are configured separately:

- **Development**: Debug logging, hot reload, development dependencies
- **Production**: Optimized logging, resource limits, security hardening

## Testing

```bash
# Run unit tests
cargo test

# Run with coverage
cargo install cargo-tarpaulin
cargo tarpaulin --out html

# Test specific module
cargo test handlers

# Integration tests
cargo test --test integration
```

## Monitoring and Health

### Health Checks

```bash
# Basic health check
curl http://localhost:3000/health

# Response:
{
  "status": "healthy",
  "timestamp": "2024-01-20T10:30:00Z",
  "service": "p2p-api-gateway",
  "version": "1.0.0"
}
```

### Metrics

```bash
# Prometheus metrics
curl http://localhost:3000/metrics

# Application metrics
{
  "uptime": "5m 30s",
  "requests_total": 1234,
  "requests_per_second": 15.2,
  "memory_usage": "45MB",
  "cpu_usage": "12%"
}
```

### Docker Health Checks

All containers include health checks:

```bash
# Check container health
docker ps
# Shows (healthy) or (unhealthy) status

# View health check logs
docker inspect rust-api-gateway | grep Health -A 10
```

## Production Deployment

### Resource Requirements

**Minimum:**
- CPU: 0.5 cores
- Memory: 256MB
- Disk: 100MB

**Recommended:**
- CPU: 1.0 cores  
- Memory: 512MB
- Disk: 1GB

### Scaling

```bash
# Scale horizontally
docker compose up --scale api-gateway=3

# With load balancer
# Configure nginx or similar to balance across instances
```

### Production Checklist

- [ ] Set production JWT secrets
- [ ] Configure database connections
- [ ] Set up monitoring (Prometheus/Grafana)
- [ ] Configure log aggregation
- [ ] Set resource limits
- [ ] Enable health checks
- [ ] Configure HTTPS/TLS
- [ ] Set up backup strategies

## Security Features

- **Memory Safety**: Rust prevents buffer overflows and memory leaks
- **Security Headers**: XSS protection, CSRF protection, etc.
- **CORS Configuration**: Configurable cross-origin policies
- **Request Validation**: Strong typing prevents injection attacks
- **Non-root Container**: Runs as unprivileged user
- **Minimal Attack Surface**: Distroless runtime image

## Performance

Expected performance characteristics:

- **Throughput**: 5,000+ requests/second
- **Latency**: <20ms average response time
- **Memory**: <50MB runtime usage
- **Startup**: <1 second cold start
- **CPU**: <15% usage under normal load

## Troubleshooting

### Common Issues

1. **Port Conflicts**
   ```bash
   # Check what's using port 3000
   lsof -i :3000
   ```

2. **Docker Build Issues**
   ```bash
   # Clean build
   docker builder prune
   ./build.sh --no-cache
   ```

3. **Database Connection**
   ```bash
   # Test database connectivity
   docker exec rust-api-gateway curl -f http://localhost:3000/health
   ```

### Logs and Debugging

```bash
# View container logs
docker compose logs api-gateway

# Follow logs
docker compose logs -f api-gateway

# Debug level logging
RUST_LOG=debug docker compose up

# Structured logs in production
docker compose logs --format json api-gateway
```

## Development

### Adding New Endpoints

1. Add models in `src/models.rs`
2. Implement handlers in `src/handlers.rs`
3. Update routing in `src/main.rs`
4. Add tests
5. Update documentation

### Code Structure

```
src/
├── main.rs           # Application entry point and routing
├── handlers.rs       # Request handlers for all endpoints
├── middleware.rs     # Custom middleware (security, logging)
├── models.rs         # Request/response models and validation
└── error.rs          # Error types and handling
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make changes with tests
4. Run `cargo test` and `cargo fmt`
5. Submit a pull request

## License

MIT License - see LICENSE file for details

## Support

For issues and questions:
- Create an issue in the repository
- Check the troubleshooting section
- Review logs with `docker compose logs api-gateway`
