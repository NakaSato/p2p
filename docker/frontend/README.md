# Frontend Docker Setup

This directory contains Docker configuration for the P2P Energy Trading Platform frontend.

## Files Overview

- `Dockerfile` - Production build with Nginx
- `Dockerfile.dev` - Development build with hot reloading
- `nginx.conf` - Nginx configuration for production
- `docker-compose.yml` - Compose configuration for both dev and prod
- `.env.development` - Development environment variables
- `.env.production` - Production environment variables
- `.dockerignore` - Files to exclude from Docker context

## Quick Start

### Development Mode

```bash
# Build and run development container
docker-compose up frontend-dev

# Or build manually
docker build -f Dockerfile.dev -t p2p-frontend-dev ../../frontend
docker run -p 3000:3000 -v "$(pwd)/../../frontend:/app" -v /app/node_modules p2p-frontend-dev
```

The development server will be available at http://localhost:3000 with hot reloading enabled.

### Production Mode

```bash
# Build and run production container
docker-compose --profile production up frontend-prod

# Or build manually
docker build -f Dockerfile -t p2p-frontend ../../frontend
docker run -p 80:80 p2p-frontend
```

The production app will be available at http://localhost

## Environment Variables

### Development
- `VITE_API_URL` - Backend API URL
- `VITE_WS_URL` - WebSocket URL for real-time features
- `VITE_ENABLE_DEVTOOLS` - Enable React DevTools

### Production
- Same as development but with production URLs
- DevTools disabled for security

## Building for Different Environments

### Local Development
```bash
docker build -f Dockerfile.dev -t p2p-frontend-dev ../../frontend
```

### Production
```bash
docker build -f Dockerfile -t p2p-frontend ../../frontend
```

### With Environment Variables
```bash
# Development
docker run --env-file .env.development -p 3000:3000 p2p-frontend-dev

# Production
docker run --env-file .env.production -p 80:80 p2p-frontend
```

## Nginx Configuration

The production build uses Nginx with:
- SPA routing support (client-side routing)
- Static asset caching
- Gzip compression
- Security headers
- API proxy to backend services
- Health check endpoint at `/health`

## Volume Mounting for Development

For development with live reloading:
```bash
docker run -p 3000:3000 \
  -v "$(pwd)/../../frontend:/app" \
  -v /app/node_modules \
  p2p-frontend-dev
```

## Integration with Full Stack

To run with the complete P2P platform:
```bash
# From the root directory
docker-compose up
```

This will start:
- Frontend (React/Vite)
- Backend API Gateway
- Smart Contracts
- Databases (PostgreSQL, Redis, TimescaleDB)
- Monitoring (Grafana, Prometheus)
- Simulators (Smart Meter, Oracle)

## Troubleshooting

### Port Conflicts
If port 3000 is in use:
```bash
docker run -p 3001:3000 p2p-frontend-dev
```

### Permission Issues on macOS/Linux
```bash
sudo chown -R $(whoami) ../../frontend/node_modules
```

### Clear Docker Cache
```bash
docker system prune -f
docker build --no-cache -f Dockerfile -t p2p-frontend ../../frontend
```

## Security Considerations

- Production build removes source maps
- Environment variables are build-time only
- Nginx security headers are configured
- No sensitive data in client-side code
- API calls go through proxy for CORS handling
