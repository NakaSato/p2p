const express = require('express');
const cors = require('cors');
const helmet = require('helmet');
const morgan = require('morgan');
require('dotenv').config();

const app = express();
const PORT = process.env.API_PORT || 8080;

// Middleware
app.use(helmet());
app.use(cors());
app.use(morgan('combined'));
app.use(express.json());

// Health check endpoint
app.get('/health', (req, res) => {
  res.json({
  status: 'healthy',
  timestamp: new Date().toISOString(),
  service: 'p2p-api-gateway'
  });
});

// Metrics endpoint for Prometheus
app.get('/metrics', (req, res) => {
  res.set('Content-Type', 'text/plain');
  res.send(`
# HELP api_requests_total Total number of API requests
# TYPE api_requests_total counter
api_requests_total{method="GET",endpoint="/health"} 1
`);
});

// Basic API endpoints (stubs)
app.get('/api/users', (req, res) => {
  res.json({ message: 'Users endpoint - implement database connection' });
});

app.get('/api/meters', (req, res) => {
  res.json({ message: 'Smart meters endpoint - implement database connection' });
});

app.get('/api/market', (req, res) => {
  res.json({ 
  message: 'Market data endpoint',
  price: Math.random() * 0.1 + 0.1,
  volume: Math.random() * 500 + 100
  });
});

app.listen(PORT, '0.0.0.0', () => {
  console.log(`API Gateway running on port ${PORT}`);
});
