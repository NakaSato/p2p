#!/bin/sh

# Health check script for frontend container

# Check if nginx is running
if ! pgrep nginx > /dev/null; then
    echo "Nginx is not running"
    exit 1
fi

# Check if essential frontend files exist
STATIC_DIR="/usr/share/nginx/html"

# Check for HTML files
if [ ! -f "$STATIC_DIR/index.html" ]; then
    echo "Missing index.html"
    exit 1
fi

# Check for CSS files (look for any .css file)
if ! find "$STATIC_DIR" -name "*.css" -type f | head -1 | grep -q .; then
    echo "No CSS files found"
    exit 1
fi

# Check for JavaScript files (look for any .js file)
if ! find "$STATIC_DIR" -name "*.js" -type f | head -1 | grep -q .; then
    echo "No JavaScript files found"
    exit 1
fi

# Check if the health endpoint responds
if curl -f http://localhost/health > /dev/null 2>&1; then
    echo "Frontend is healthy"
    exit 0
else
    echo "Frontend health check failed"
    exit 1
fi
