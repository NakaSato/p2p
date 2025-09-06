const express = require('express');
const cors = require('cors');
const helmet = require('helmet');
const morgan = require('morgan');
const compression = require('compression');
const rateLimit = require('express-rate-limit');
const { createProxyMiddleware } = require('http-proxy-middleware');

const app = express();
const PORT = process.env.API_PORT || process.env.PORT || 3000;

// Middleware
app.use(helmet());
app.use(cors());
app.use(compression());
app.use(morgan('combined'));
app.use(express.json());

// Rate limiting
const limiter = rateLimit({
  windowMs: 15 * 60 * 1000, // 15 minutes
  max: 100 // limit each IP to 100 requests per windowMs
});
app.use(limiter);

// Health check endpoint
app.get('/health', (req, res) => {
  res.json({ status: 'healthy', timestamp: new Date().toISOString() });
});

// API Gateway routes
app.get('/api/status', (req, res) => {
  res.json({
    message: 'P2P Energy Trading API Gateway',
    version: '1.0.0',
    timestamp: new Date().toISOString(),
    services: {
      postgres: 'Available',
      timescaledb: 'Available',
      redis: 'Available',
      kafka: 'Available'
    }
  });
});

// Proxy middleware for different services
const services = {
  // Note: substrate-node proxy commented out until image is available
  // '/api/blockchain': {
  //   target: 'http://substrate-node:9944',
  //   changeOrigin: true,
  //   pathRewrite: { '^/api/blockchain': '' }
  // },
  '/api/metrics': {
    target: 'http://prometheus:9090',
    changeOrigin: true,
    pathRewrite: { '^/api/metrics': '' }
  }
};

// Create proxy middlewares
Object.entries(services).forEach(([path, config]) => {
  app.use(path, createProxyMiddleware(config));
});

// Default route
app.get('/', (req, res) => {
  res.json({
    message: 'P2P Energy Trading Platform API Gateway',
    version: '1.0.0',
    endpoints: {
      health: '/health',
      status: '/api/status',
      metrics: '/api/metrics'
    }
  });
});

// Error handling middleware
app.use((err, req, res, next) => {
  console.error(err.stack);
  res.status(500).json({ error: 'Something went wrong!' });
});

// 404 handler
app.use('*', (req, res) => {
  res.status(404).json({ error: 'Route not found' });
});

app.listen(PORT, '0.0.0.0', () => {
  console.log(`API Gateway running on port ${PORT}`);
  console.log(`Environment: ${process.env.NODE_ENV || 'development'}`);
});

module.exports = app;
