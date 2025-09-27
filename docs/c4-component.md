# P2P Energy Trading System - C4 Component Diagrams Overview (Actual Implementation)

This document provides an overview of the API Gateway's internal architecture through multiple focused diagrams based on the actual codebase implementation. The complex architecture has been separated into logical layers for better understanding and maintainability.

## 📋 Component Architecture Layers (Actual Implementation)

The API Gateway follows clean architecture principles with clear separation of concerns across multiple layers, implemented in Rust with Axum framework:

### 🎛️ **Controllers Layer (23 REST Endpoints)**
**[View Controllers Diagram →](c4-component-controllers.md)**

- **Purpose**: Handle HTTP requests, validation, and routing through Axum handlers
- **Components**: 23 actual REST endpoints across 9 controllers (Health, Auth, User, Admin, Blockchain, Trading, Meters, Analytics, Department)
- **Implementation**: Rust/Axum with middleware stack (CORS, Tracing, Timeout, Authentication)
- **Key Endpoints**: `/auth/login`, `/blockchain/transactions`, `/trading/orders`, `/meters/readings`, `/analytics/user`

### 🧠 **Services & Business Logic Layer**  
**[View Services Diagram →](c4-component-services.md)**

- **Purpose**: Implement core business logic and orchestration in dedicated service modules
- **Components**: JWT Service, API Key Service, Blockchain Service, User Service, Trading Service, Energy Service, Analytics Service
- **Implementation**: Rust service modules in `src/services/` with dependency injection through AppState
- **Key Integrations**: Solana RPC client, PostgreSQL queries, Redis caching, Kafka event streaming

### 💾 **Data Access & External Integrations**
**[View Data Access Diagram →](c4-component-data.md)**

- **Purpose**: Data persistence, caching, and external system integration based on actual configuration
- **Components**: PostgreSQL connection pool (9 tables), Redis client (session/cache), Kafka producer (events), Solana RPC client, AMI client, REC API client  
- **Implementation**: SQLx for PostgreSQL, Redis client for caching, Kafka for streaming, HTTP clients for external APIs
- **Database Schema**: 9 actual tables with custom enums (user_role, order_type, order_side, order_status)

## 🏗️ **Architecture Patterns Implemented (From Codebase)**

| Pattern | Layer | Actual Implementation |
|---------|-------|----------------------|
| **Clean Architecture** | All Layers | Clear separation: handlers → services → data access with dependency injection |
| **Repository Pattern** | Data Layer | SQLx connection pools with query abstractions |
| **Middleware Pattern** | Controllers | Tower middleware stack: auth, CORS, tracing, timeout, rate limiting |
| **Event-Driven Architecture** | Services | Kafka producer for real-time events and notifications |
| **External Integration Pattern** | Data Layer | HTTP clients for AMI (15 meters) and REC authority APIs |
| **Application State** | Cross-cutting | Shared AppState with DB pools, Redis client, config, JWT services |

## 🔗 **Complete System Architecture (Actual Docker Services)**

The system is deployed using Docker Compose with the following actual services:

### **Container Services (from docker-compose.yml):**
- **p2p-nginx**: Load balancer (ports 80/443)
- **p2p-frontend**: React/TypeScript app (port 3000→80)  
- **p2p-api-gateway**: Rust/Axum API (port 8080)
- **p2p-postgres**: PostgreSQL 18 (port 5432)
- **p2p-redis**: Redis 7 (port 6379)
- **p2p-kafka**: Apache Kafka (port 9092)
- **p2p-anchor-dev**: Solana validator (ports 8898, 8901)
- **p2p-oracle-simulator**: Market data simulation
- **p2p-smart-meter-simulator**: AMI data generation (10 meters)
- **p2p-prometheus**: Metrics collection (port 9090)
- **p2p-grafana**: Monitoring dashboards (port 3001)

### **Key Integration Points (Actual Implementation):**

- **Controllers ↔ Services**: Axum handlers delegate to service modules via dependency injection
- **Services ↔ Data Access**: Business logic uses connection pools and clients for data operations
- **Services ↔ Blockchain**: Direct Solana RPC integration with 5 deployed Anchor programs
- **Middleware ↔ Cross-cutting**: Tower middleware provides authentication, rate limiting, CORS, tracing
- **External ↔ APIs**: HTTP/MQTT clients for 15 smart meters (METER_001-015) and REC authority

## 🎯 **Benefits of Actual Implementation**

- **🔧 Maintainability**: Modular Rust services with clear boundaries and type safety
- **📈 Scalability**: Connection pooling, Redis caching, and Kafka streaming for performance
- **🧪 Testability**: Separated concerns enable comprehensive unit and integration testing
- **🔒 Security**: JWT authentication, API key validation, rate limiting, and audit logging
- **📊 Observability**: Prometheus metrics, Grafana dashboards, and distributed tracing
- **🚀 Performance**: Rust/Axum for high-performance async HTTP handling

## 🔍 **Technical Stack Summary (From Codebase)**

- **Frontend**: React 18 + TypeScript + Vite + TailwindCSS
- **API Gateway**: Rust + Axum + Tower + SQLx + Redis
- **Database**: PostgreSQL 18 with 9 custom tables and enums
- **Blockchain**: Solana + Anchor (5 programs: Registry, Energy Token, Trading, Oracle, Governance)
- **Caching**: Redis 7 for sessions and performance optimization
- **Messaging**: Apache Kafka for event streaming
- **Monitoring**: Prometheus + Grafana for system observability
- **Infrastructure**: Docker Compose with 11 services and persistent volumes

This modular approach ensures the P2P Energy Trading System's API Gateway remains maintainable, scalable, and aligned with clean architecture principles while leveraging Rust's performance and safety benefits.