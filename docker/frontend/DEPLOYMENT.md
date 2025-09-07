# P2P Energy Trading Platform - Frontend Docker Configuration

Complete Docker setup for the React/TypeScript/Vite frontend application.

## 📁 File Structure

```
docker/frontend/
├── Dockerfile              # Production build with Nginx
├── Dockerfile.dev          # Development build with hot reload
├── nginx.conf              # Nginx configuration for production
├── docker-compose.yml      # Standalone frontend compose
├── .dockerignore           # Files to exclude from build context
├── .env.development        # Development environment variables
├── .env.production         # Production environment variables
├── healthcheck.sh          # Health check script for containers
├── build.sh               # Build script for Docker images
├── README.md              # This file
└── DEPLOYMENT.md          # Deployment documentation
```

## 🚀 Quick Start

### Development Mode (Recommended for development)
```bash
# From the root project directory
docker-compose --profile dev up frontend-dev

# Or using the frontend-specific compose file
cd docker/frontend
docker-compose up frontend-dev
```

### Production Mode
```bash
# From the root project directory
docker-compose --profile prod up frontend

# Or build and run manually
cd docker/frontend
./build.sh
docker run -p 80:80 p2p-frontend:latest
```

## 🔧 Build Options

### Using Build Script
```bash
cd docker/frontend

# Build both development and production images
./build.sh

# Build with custom tag
./build.sh --tag v1.0.0
```

### Manual Building
```bash
# Development image
docker build -f docker/frontend/Dockerfile.dev -t p2p-frontend:dev frontend/

# Production image
docker build -f docker/frontend/Dockerfile -t p2p-frontend:latest frontend/
```

## 🌍 Environment Configuration

### Development Environment
- Hot reloading enabled
- Source maps included
- React DevTools enabled
- API URL: `http://localhost:3001/api`
- Port: 3000

### Production Environment
- Optimized build with Nginx
- Source maps excluded
- Gzip compression enabled
- Security headers configured
- API proxy configured
- Port: 80

## 📝 Environment Variables

### Frontend Application Variables
```bash
VITE_API_URL=http://localhost:3001/api          # Backend API endpoint
VITE_WS_URL=ws://localhost:3001                # WebSocket endpoint
VITE_APP_ENV=development                       # Application environment
VITE_APP_NAME=P2P Energy Trading Platform     # Application name
VITE_ENABLE_DEVTOOLS=true                      # Enable React DevTools
```

### Docker Container Variables
```bash
NODE_ENV=development                           # Node environment
```

## 🏗️ Docker Configuration Details

### Production Dockerfile (Multi-stage)
1. **Build Stage**: Node.js 22 Alpine for building the application
2. **Production Stage**: Nginx Alpine for serving static files

### Development Dockerfile
- Single stage Node.js 22 Alpine
- Volumes for hot reloading
- Development server on port 3000

### Nginx Configuration
- SPA routing support (client-side routing)
- API proxy to backend services
- Static asset caching with long-term cache headers
- Gzip compression for better performance
- Security headers (XSS, CSRF, Content-Type protection)
- Health check endpoint at `/health`

## 🔍 Health Checks

Production containers include health checks:
- **Endpoint**: `/health`
- **Interval**: 30 seconds
- **Timeout**: 10 seconds
- **Retries**: 3

Check container health:
```bash
docker ps                           # Check container status
curl http://localhost/health        # Test health endpoint
docker exec frontend-container /usr/local/bin/healthcheck.sh
```

## 🔗 Integration with Full Stack

### Using Docker Compose Profiles
```bash
# Development environment
docker-compose --profile dev up

# Production environment
docker-compose --profile prod up

# Specific services
docker-compose up frontend-dev api-gateway postgres
```

### Service Dependencies
- Frontend depends on `api-gateway`
- API calls are proxied through Nginx
- WebSocket connections for real-time features

## 🐛 Troubleshooting

### Common Issues

#### Port Already in Use
```bash
# Check what's using the port
lsof -i :3000

# Use different port
docker run -p 3001:3000 p2p-frontend:dev
```

#### Build Failures
```bash
# Clear Docker cache
docker system prune -f

# Rebuild without cache
docker build --no-cache -f docker/frontend/Dockerfile -t p2p-frontend:latest frontend/
```

#### Permission Issues (macOS/Linux)
```bash
# Fix node_modules permissions
sudo chown -R $(whoami) frontend/node_modules

# Or run with user mapping
docker run --user $(id -u):$(id -g) -p 3000:3000 p2p-frontend:dev
```

#### Hot Reloading Not Working
```bash
# Ensure volumes are properly mounted
docker run -p 3000:3000 \
  -v "$(pwd)/frontend:/app" \
  -v /app/node_modules \
  p2p-frontend:dev
```

### Logs and Debugging
```bash
# View container logs
docker logs p2p-frontend-dev
docker logs p2p-frontend

# Enter container for debugging
docker exec -it p2p-frontend-dev sh
docker exec -it p2p-frontend sh

# Check Nginx configuration
docker exec p2p-frontend nginx -t

# View Nginx logs
docker exec p2p-frontend tail -f /var/log/nginx/access.log
docker exec p2p-frontend tail -f /var/log/nginx/error.log
```

## 🔒 Security Considerations

### Production Security
- No source maps in production build
- Security headers configured in Nginx
- Environment variables are build-time only
- No sensitive data exposed to client
- HTTPS redirect configured (when SSL is available)

### Development Security
- Development server only for local development
- Hot reloading for development convenience
- React DevTools available for debugging

## 📊 Performance Optimizations

### Production Build
- Code splitting and tree shaking
- Asset optimization and minification
- Gzip compression
- Long-term caching headers
- Static asset serving via Nginx

### Docker Optimizations
- Multi-stage builds for smaller images
- Layer caching for faster builds
- .dockerignore for build context optimization
- Alpine Linux base images for smaller size

## 🚀 Deployment

### Development Deployment
```bash
# Local development
docker-compose --profile dev up frontend-dev

# With specific configuration
docker run --env-file docker/frontend/.env.development -p 3000:3000 p2p-frontend:dev
```

### Production Deployment
```bash
# Local production testing
docker-compose --profile prod up frontend

# With load balancer
docker run --env-file docker/frontend/.env.production -p 80:80 p2p-frontend:latest

# With orchestration (Kubernetes, Docker Swarm)
# See deployment-specific documentation
```

## 📋 Maintenance

### Updating Dependencies
```bash
# Update package.json in frontend directory
cd frontend && npm update

# Rebuild Docker images
cd docker/frontend && ./build.sh
```

### Monitoring
- Health checks provide container status
- Nginx access logs for request monitoring
- Integration with Prometheus/Grafana for metrics

## 🔄 CI/CD Integration

The Docker configuration integrates with:
- GitHub Actions workflows
- Automated testing and building
- Multi-environment deployments
- Container registry publishing

For CI/CD configuration, see `.github/workflows/frontend.yml`.
