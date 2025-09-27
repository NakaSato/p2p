# P2P Energy Trading System - C4 Component Diagram: Services & Business Logic (Actual Implementation)

This diagram focuses on the actual Services layer implementation showing the business logic components and their interactions based on the real codebase structure.

```plantuml
@startuml
!include https://raw.githubusercontent.com/plantuml-stdlib/C4-PlantUML/master/C4_Component.puml

title Services & Business Logic Layer - P2P Energy Trading System (Actual Implementation)

'=================================
' Controllers (Upstream - From main.rs)
'=================================
Container_Boundary(controllers_upstream, "Controllers Layer (23 REST Endpoints)") {
    Component(auth_controller, "Authentication Controller", "Axum Handler", "JWT authentication and profile management")
    Component(blockchain_controller, "Blockchain Controller", "Axum Handler", "Solana program interactions")
    Component(trading_controller, "Trading Controller", "Axum Handler", "Order management and market data")
    Component(meters_controller, "Meters Controller", "Axum Handler", "AMI energy data processing")
    Component(analytics_controller, "Analytics Controller", "Axum Handler", "Energy and trading analytics")
}

'=================================
' Services Layer (Main Focus - Actual Implementation)
'=================================
Container_Boundary(services_layer, "Business Logic Services - Rust (src/services/)") {
    
    Component(jwt_service, "JWT Service", "auth::jwt::JwtService", "JWT token generation and validation\n• Token signing with secret\n• Claims extraction\n• Token expiry management")
    
    Component(api_key_service, "API Key Service", "auth::jwt::ApiKeyService", "API key management and validation\n• Key generation\n• Engineering API key validation\n• Service authentication")
    
    Component(blockchain_service, "Blockchain Service", "services::blockchain", "Solana blockchain integration\n• RPC client management\n• Program interaction (5 programs)\n• Transaction submission\n• Account management")
    
    Component(user_service, "User Service", "services::user", "User account management\n• Registration processing\n• Profile validation\n• Wallet address management\n• Department verification")
    
    Component(trading_service, "Trading Service", "services::trading", "Trading operations and order management\n• Order validation\n• Market data aggregation\n• Trading statistics\n• Order history management")
    
    Component(energy_service, "Energy Service", "services::energy", "Energy data processing and validation\n• Meter reading validation\n• AMI data processing\n• Energy analytics\n• Consumption tracking")
    
    Component(analytics_service, "Analytics Service", "services::analytics", "System and user analytics\n• Energy usage analytics\n• Trading performance metrics\n• System health metrics\n• User behavior analysis")
}

'=================================
' Middleware Integration (Actual Implementation)
'=================================
Container_Boundary(middleware_layer, "Middleware Services") {
    Component(auth_middleware, "Auth Middleware", "auth::middleware", "Request authentication\n• JWT validation\n• User context extraction\n• Role-based access control")
    
    Component(rate_limiter, "Rate Limiter", "Tower Middleware", "Request rate limiting\n• Redis-backed rate limiting\n• Per-user limits\n• API protection")
}

'=================================
' Data Access Layer (Downstream)
'=================================
Container_Boundary(data_layer, "Data Access Layer (Actual Implementation)") {
    Component(postgres_pool, "PostgreSQL Pool", "sqlx::PgPool", "Primary database connection pool\n• User data\n• Trading orders\n• Activities\n• Meter assignments")
    
    Component(redis_client, "Redis Client", "redis::Client", "Cache and session management\n• JWT session cache\n• Rate limiting counters\n• Performance optimization")
    
    Component(kafka_producer, "Kafka Producer", "Apache Kafka Client", "Event streaming\n• Real-time notifications\n• System events\n• Audit trail")
}

'=================================
' External Systems (From AppState)
'=================================
System_Ext(solana_programs, "5 Anchor Programs", "Registry, Energy Token, Trading, Oracle, Governance on Solana")
ContainerDb(postgres, "PostgreSQL 18", "Primary database with 9 tables and custom types")
ContainerDb(redis, "Redis 7", "Session cache and rate limiting store")
ContainerQueue(kafka, "Apache Kafka", "Event streaming and real-time updates")

'=================================
' Controllers to Services Flow (Actual Handler Implementations)
'=================================
Rel_D(auth_controller, jwt_service, "JWT operations")
Rel_D(auth_controller, api_key_service, "API key validation")
Rel_D(blockchain_controller, blockchain_service, "Solana program calls")
Rel_D(trading_controller, trading_service, "Order processing")
Rel_D(meters_controller, energy_service, "Energy data validation")
Rel_D(analytics_controller, analytics_service, "Analytics computation")

'=================================
' Inter-Service Interactions (Actual Dependencies)
'=================================
Rel_R(user_service, blockchain_service, "User blockchain registration")
Rel_R(trading_service, blockchain_service, "Order blockchain operations")
Rel_R(energy_service, blockchain_service, "Token minting for energy generation")
Rel_R(analytics_service, blockchain_service, "Blockchain state queries")

'=================================
' Middleware Integration
'=================================
Rel_U(auth_middleware, jwt_service, "Token validation")
Rel_U(rate_limiter, redis_client, "Rate limit tracking")

'=================================
' Services to Data Layer (Actual Database Operations)
'=================================
Rel_D(user_service, postgres_pool, "User CRUD operations")
Rel_D(trading_service, postgres_pool, "Order management")
Rel_D(energy_service, postgres_pool, "Meter data storage")
Rel_D(analytics_service, postgres_pool, "Analytics queries")
Rel_D(jwt_service, redis_client, "Session caching")
Rel_D(user_service, redis_client, "User cache management")
Rel_D(analytics_service, kafka_producer, "Event publishing")

'=================================
' External System Connections (From Config)
'=================================
Rel_D(blockchain_service, solana_programs, "RPC calls to 5 programs", "Solana RPC")
Rel_D(postgres_pool, postgres, "Database operations", "TCP/5432")
Rel_D(redis_client, redis, "Cache operations", "TCP/6379")
Rel_D(kafka_producer, kafka, "Event streaming", "TCP/9092")

'=================================
' Styling - Services Implementation Theme
'=================================

' Controllers (upstream) - Based on actual endpoint grouping
UpdateElementStyle(auth_controller, $bgColor="#E3F2FD", $fontColor="#1976D2", $borderColor="#2196F3")
UpdateElementStyle(blockchain_controller, $bgColor="#E0F2F1", $fontColor="#00695C", $borderColor="#009688")
UpdateElementStyle(trading_controller, $bgColor="#FFF3E0", $fontColor="#F57C00", $borderColor="#FF9800")
UpdateElementStyle(meters_controller, $bgColor="#F3E5F5", $fontColor="#7B1FA2", $borderColor="#9C27B0")
UpdateElementStyle(analytics_controller, $bgColor="#E1F5FE", $fontColor="#0277BD", $borderColor="#0288D1")

' Services - Business logic implementation
UpdateElementStyle(jwt_service, $bgColor="#E8F5E8", $fontColor="#2E7D32", $borderColor="#4CAF50")
UpdateElementStyle(api_key_service, $bgColor="#F1F8E9", $fontColor="#558B2F", $borderColor="#8BC34A")
UpdateElementStyle(blockchain_service, $bgColor="#E0F2F1", $fontColor="#00695C", $borderColor="#009688")
UpdateElementStyle(user_service, $bgColor="#E8F5E8", $fontColor="#2E7D32", $borderColor="#4CAF50")
UpdateElementStyle(trading_service, $bgColor="#E3F2FD", $fontColor="#1565C0", $borderColor="#2196F3")
UpdateElementStyle(energy_service, $bgColor="#F1F8E9", $fontColor="#558B2F", $borderColor="#8BC34A")
UpdateElementStyle(analytics_service, $bgColor="#E1F5FE", $fontColor="#0277BD", $borderColor="#03A9F4")

' Middleware
UpdateElementStyle(auth_middleware, $bgColor="#F3E5F5", $fontColor="#7B1FA2", $borderColor="#9C27B0")
UpdateElementStyle(rate_limiter, $bgColor="#FFF8E1", $fontColor="#F57F17", $borderColor="#FBC02D")

' Data access layer
UpdateElementStyle(postgres_pool, $bgColor="#ECEFF1", $fontColor="#37474F", $borderColor="#607D8B")
UpdateElementStyle(redis_client, $bgColor="#FFEBEE", $fontColor="#C62828", $borderColor="#F44336")
UpdateElementStyle(kafka_producer, $bgColor="#FFF8E1", $fontColor="#E65100", $borderColor="#FBC02D")

' External systems
UpdateElementStyle(solana_programs, $bgColor="#E8F5E8", $fontColor="#1B5E20", $borderColor="#388E3C")

LAYOUT_WITH_LEGEND()
LAYOUT_TOP_DOWN()

@enduml
```

```plantuml
@startuml
!include https://raw.githubusercontent.com/plantuml-stdlib/C4-PlantUML/master/C4_Component.puml

title Services & Business Logic Layer - P2P Energy Trading System

'=================================
' Controllers (Upstream)
'=================================
Container_Boundary(controllers_upstream, "Controllers Layer") {
    Component(auth_controller, "Authentication Controller", "Axum Handler", "JWT token validation")
    Component(trading_controller, "Trading Controller", "Axum Handler", "Order management")
    Component(energy_controller, "Energy Controller", "Axum Handler", "Energy data processing")
}

'=================================
' Services Layer (Main Focus)
'=================================
Container_Boundary(services_layer, "Business Logic Services - Rust") {
    Component(blockchain_service, "Blockchain Service", "Solana Client", "Interaction with Solana programs and transaction management")
    Component(user_service, "User Service", "Business Logic", "User account management and validation")
    Component(trading_service, "Trading Service", "Business Logic", "Order management and trading operations")
    Component(energy_service, "Energy Service", "Business Logic", "Energy data processing and REC validation")
    Component(oracle_service, "Oracle Service", "Business Logic", "External data integration and market clearing")
    Component(notification_service, "Notification Service", "Business Logic", "Real-time notifications and alerts")
}

'=================================
' Data Layer (Downstream)
'=================================
Container_Boundary(data_layer, "Data Access Layer") {
    Component(database_repository, "Database Repository", "SQLx/PostgreSQL", "Data access layer for user and trading data")
    Component(cache_repository, "Cache Repository", "Redis Client", "High-performance caching layer")
    Component(event_publisher, "Event Publisher", "Kafka Producer", "Event streaming and real-time updates")
}

'=================================
' External Systems
'=================================
System_Ext(solana_programs, "Solana Programs", "Registry, Trading, Oracle, Token, Governance programs")
ContainerDb(postgres, "PostgreSQL", "PostgreSQL 18", "Primary data storage")
ContainerDb(redis, "Redis Cache", "Redis 7", "Session and performance cache")
ContainerQueue(kafka, "Message Queue", "Apache Kafka", "Event streaming")

'=================================
' Controllers to Services Flow
'=================================
Rel_D(auth_controller, user_service, "User authentication")
Rel_D(trading_controller, trading_service, "Trading operations")
Rel_D(energy_controller, energy_service, "Energy data processing")

'=================================
' Inter-Service Interactions
'=================================
Rel_R(user_service, blockchain_service, "User registration on blockchain")
Rel_R(trading_service, blockchain_service, "Order execution on blockchain")
Rel_R(energy_service, blockchain_service, "Token minting/burning")
Rel_R(oracle_service, blockchain_service, "Data feeds and market clearing")
Rel_D(notification_service, event_publisher, "Real-time notifications")

'=================================
' Services to Data Layer
'=================================
Rel_D(user_service, database_repository, "User data persistence")
Rel_D(trading_service, database_repository, "Trading history")
Rel_D(energy_service, database_repository, "Energy consumption data")
Rel_D(user_service, cache_repository, "User session caching")
Rel_D(trading_service, cache_repository, "Order caching")

'=================================
' External System Connections
'=================================
Rel_D(blockchain_service, solana_programs, "Smart contract calls", "RPC")
Rel_D(database_repository, postgres, "SQL operations", "TCP")
Rel_D(cache_repository, redis, "Caching operations", "TCP")
Rel_D(event_publisher, kafka, "Event streaming", "TCP")

'=================================
' Styling - Services Theme
'=================================

' Controllers (upstream)
UpdateElementStyle(auth_controller, $bgColor="#E3F2FD", $fontColor="#1976D2", $borderColor="#2196F3")
UpdateElementStyle(trading_controller, $bgColor="#FFF3E0", $fontColor="#F57C00", $borderColor="#FF9800")
UpdateElementStyle(energy_controller, $bgColor="#F3E5F5", $fontColor="#7B1FA2", $borderColor="#9C27B0")

' Services - Business logic theme
UpdateElementStyle(blockchain_service, $bgColor="#E0F2F1", $fontColor="#00695C", $borderColor="#009688")
UpdateElementStyle(user_service, $bgColor="#E8F5E8", $fontColor="#2E7D32", $borderColor="#4CAF50")
UpdateElementStyle(trading_service, $bgColor="#E3F2FD", $fontColor="#1565C0", $borderColor="#2196F3")
UpdateElementStyle(energy_service, $bgColor="#F1F8E9", $fontColor="#558B2F", $borderColor="#8BC34A")
UpdateElementStyle(oracle_service, $bgColor="#FFF3E0", $fontColor="#E65100", $borderColor="#FF9800")
UpdateElementStyle(notification_service, $bgColor="#FCE4EC", $fontColor="#C2185B", $borderColor="#E91E63")

' Data access layer
UpdateElementStyle(database_repository, $bgColor="#ECEFF1", $fontColor="#37474F", $borderColor="#607D8B")
UpdateElementStyle(cache_repository, $bgColor="#FFEBEE", $fontColor="#C62828", $borderColor="#F44336")
UpdateElementStyle(event_publisher, $bgColor="#FFF8E1", $fontColor="#E65100", $borderColor="#FBC02D")

LAYOUT_WITH_LEGEND()
LAYOUT_TOP_DOWN()

@enduml
```