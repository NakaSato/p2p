FROM node:18-slim

WORKDIR /app

# Install basic dependencies for a simple API gateway
RUN npm init -y && \
  npm install express cors helmet morgan dotenv

# Create a basic Express server
COPY server.js .

EXPOSE 8080

CMD ["node", "server.js"]
