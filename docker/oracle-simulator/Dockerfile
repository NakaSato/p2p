FROM node:18-slim

WORKDIR /app

# Install system dependencies
RUN apt-get update && apt-get install -y \
    python3 \
    make \
    g++ \
    && rm -rf /var/lib/apt/lists/*

# Copy package files
COPY package*.json ./

# Install dependencies
RUN npm install

# Copy application code
COPY . .

# Create non-root user
RUN useradd --create-home --shell /bin/bash oracle && \
    chown -R oracle:oracle /app
USER oracle

# Command to run the application
CMD ["node", "oracle-simulator.js"]
