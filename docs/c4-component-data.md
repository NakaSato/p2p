# P2P Energy Trading System - C4 Component Diagram: Data Access & External Integrations (Actual Implementation)

This diagram focuses on the actual data access layer and external system integrations based on the real database schema, Redis configuration, and external API integrations from the codebase.

```plantuml
@startuml
!include https://raw.githubusercontent.com/plantuml-stdlib/C4-PlantUML/master/C4_Component.puml

title Data Access & External Integrations - P2P Energy Trading System (Actual Implementation)

'=================================
' Services Layer (Upstream - Actual Implementation)
'=================================
Container_Boundary(services_upstream, "Business Logic Services (src/services/)") {
    Component(jwt_service, "JWT Service", "auth::jwt::JwtService", "Token management and validation")
    Component(user_service, "User Service", "services::user", "User account operations")
    Component(trading_service, "Trading Service", "services::trading", "Order management")
    Component(energy_service, "Energy Service", "services::energy", "Energy data processing")
    Component(blockchain_service, "Blockchain Service", "services::blockchain", "Solana integration")
    Component(analytics_service, "Analytics Service", "services::analytics", "System analytics")
}

'=================================
' Data Access Layer (Main Focus - Actual Implementation)
'=================================
Container_Boundary(data_access_layer, "Data Access & Repository Layer (Actual Implementation)") {
    
    Component(postgres_pool, "PostgreSQL Connection Pool", "sqlx::PgPool", "Primary database connection pool\n• Connection pooling\n• Transaction management\n• Query execution\n• Migration support")
    
    Component(redis_client, "Redis Client", "redis::Client", "High-performance caching and session management\n• Session storage\n• Rate limiting counters\n• Query result caching\n• Real-time data cache")
    
    Component(kafka_producer, "Kafka Event Producer", "Apache Kafka Client", "Event streaming and real-time updates\n• User activity events\n• Trading events\n• Energy reading events\n• System notifications")
    
    Component(solana_rpc_client, "Solana RPC Client", "Solana Web3 Client", "Blockchain interaction client\n• Program instruction calls\n• Account data queries\n• Transaction submission\n• Network status monitoring")
    
    Component(ami_client, "AMI Data Client", "HTTP/MQTT Client", "Smart meter data integration\n• METER_001-015 data collection\n• Real-time energy readings\n• Meter status monitoring\n• Data validation")
    
    Component(rec_api_client, "REC Validation Client", "HTTP Client", "External API integration\n• Certificate validation\n• Compliance checking\n• Authority verification\n• Audit trail")
}

'=================================
' Middleware Data Access (From AppState)
'=================================
Container_Boundary(middleware_integration, "Middleware Data Access") {
    Component(auth_middleware, "Authentication Middleware", "auth::middleware", "Session and authentication data access")
    Component(rate_limiter_middleware, "Rate Limiter", "Tower Middleware", "Rate limit tracking and enforcement")
}

'=================================
' External Systems & Data Stores (Actual Configuration)
'=================================
System_Ext(solana_programs, "5 Anchor Programs", "Registry, Energy Token, Trading, Oracle, Governance programs")
System_Ext(smart_meters, "Smart Meters (15 units)", "METER_001-015 AMI network")
System_Ext(rec_authority, "REC Authority API", "External renewable energy certificate system")

ContainerDb(postgres, "PostgreSQL 18", "Primary database with 9 tables:\n• users, user_activities\n• trading_orders\n• meter_assignments\n• energy_readings\n• api_keys\n• blockchain_transactions")
ContainerDb(redis, "Redis 7", "Cache and session store:\n• JWT sessions\n• Rate limiting\n• Query cache\n• Real-time data")
ContainerQueue(kafka, "Apache Kafka", "Event streaming broker:\n• User events\n• Trading events\n• Energy events\n• System notifications")

'=================================
' Database Schema (Actual Tables from Migrations)
'=================================
note right of postgres : Database Schema (9 Tables):\n• users (with custom user_role enum)\n• user_activities\n• api_keys\n• meter_assignments\n• trading_orders (with order enums)\n• energy_readings\n• blockchain_transactions\n• Triggers for updated_at

'=================================
' Services to Data Access (Actual Dependencies)
'=================================
Rel_D(user_service, postgres_pool, "User CRUD operations\n• Registration\n• Profile updates\n• Activity logging")
Rel_D(trading_service, postgres_pool, "Order management\n• Create/cancel orders\n• Trading history\n• Order matching")
Rel_D(energy_service, postgres_pool, "Energy data storage\n• Meter readings\n• Consumption tracking\n• Analytics data")
Rel_D(analytics_service, postgres_pool, "Analytics queries\n• User statistics\n• System metrics\n• Performance data")
Rel_D(blockchain_service, postgres_pool, "Transaction logging\n• Blockchain state\n• Program interactions")

'=================================
' Cache Layer Integration
'=================================
Rel_D(jwt_service, redis_client, "Session management\n• JWT token caching\n• User session data")
Rel_D(user_service, redis_client, "User data caching\n• Profile cache\n• Activity cache")
Rel_D(trading_service, redis_client, "Order caching\n• Market data cache\n• Trading statistics")
Rel_D(analytics_service, redis_client, "Analytics caching\n• Query result cache\n• Computed metrics")

'=================================
' Middleware to Data Access
'=================================
Rel_D(auth_middleware, redis_client, "Session validation\n• JWT verification\n• User context")
Rel_D(rate_limiter_middleware, redis_client, "Rate limit enforcement\n• Request counting\n• Limit tracking")

'=================================
' External System Integrations (From Config)
'=================================
Rel_D(blockchain_service, solana_programs, "Program interactions\n• Registry operations\n• Token operations\n• Trading operations", "Solana RPC")
Rel_D(energy_service, ami_client, "Energy data collection\n• Meter readings\n• Real-time data")
Rel_D(energy_service, rec_api_client, "Certificate validation\n• REC verification\n• Compliance checks")
Rel_D(ami_client, smart_meters, "AMI data ingestion\n• 15 meter data points\n• Real-time monitoring", "HTTPS/MQTT")
Rel_D(rec_api_client, rec_authority, "REC validation API\n• Certificate checks\n• Authority verification", "HTTPS/REST")
Rel_D(solana_rpc_client, solana_programs, "Blockchain operations\n• Account queries\n• Transaction submission", "RPC calls")

'=================================
' Data Persistence (From docker-compose.yml)
'=================================
Rel_D(postgres_pool, postgres, "Database operations\n• SQL queries\n• Transactions\n• Migrations", "TCP/5432")
Rel_D(redis_client, redis, "Cache operations\n• GET/SET operations\n• Expiry management", "TCP/6379")
Rel_D(kafka_producer, kafka, "Event streaming\n• Topic publishing\n• Event processing", "TCP/9092")

'=================================
' Event Streaming Integration
'=================================
Rel_D(user_service, kafka_producer, "User events\n• Registration events\n• Profile changes")
Rel_D(trading_service, kafka_producer, "Trading events\n• Order events\n• Trade events")
Rel_D(energy_service, kafka_producer, "Energy events\n• Meter reading events\n• Generation events")
Rel_D(analytics_service, kafka_producer, "System events\n• Analytics events\n• Performance events")

'=================================
' Styling - Data & Integration Theme (Actual Implementation)
'=================================

' Services (upstream) - Based on actual service structure
UpdateElementStyle(jwt_service, $bgColor="#E8F5E8", $fontColor="#2E7D32", $borderColor="#4CAF50")
UpdateElementStyle(user_service, $bgColor="#E8F5E8", $fontColor="#2E7D32", $borderColor="#4CAF50")
UpdateElementStyle(trading_service, $bgColor="#E3F2FD", $fontColor="#1565C0", $borderColor="#2196F3")
UpdateElementStyle(energy_service, $bgColor="#F1F8E9", $fontColor="#558B2F", $borderColor="#8BC34A")
UpdateElementStyle(blockchain_service, $bgColor="#E0F2F1", $fontColor="#00695C", $borderColor="#009688")
UpdateElementStyle(analytics_service, $bgColor="#E1F5FE", $fontColor="#0277BD", $borderColor="#03A9F4")

' Data access layer - Primary focus (actual implementation)
UpdateElementStyle(postgres_pool, $bgColor="#ECEFF1", $fontColor="#37474F", $borderColor="#607D8B")
UpdateElementStyle(redis_client, $bgColor="#FFEBEE", $fontColor="#C62828", $borderColor="#F44336")
UpdateElementStyle(kafka_producer, $bgColor="#FFF8E1", $fontColor="#E65100", $borderColor="#FBC02D")
UpdateElementStyle(solana_rpc_client, $bgColor="#E0F2F1", $fontColor="#00695C", $borderColor="#009688")
UpdateElementStyle(ami_client, $bgColor="#E8EAF6", $fontColor="#3F51B5", $borderColor="#5C6BC0")
UpdateElementStyle(rec_api_client, $bgColor="#F3E5F5", $fontColor="#8E24AA", $borderColor="#AB47BC")

' Middleware
UpdateElementStyle(auth_middleware, $bgColor="#F3E5F5", $fontColor="#7B1FA2", $borderColor="#9C27B0")
UpdateElementStyle(rate_limiter_middleware, $bgColor="#FFF8E1", $fontColor="#F57F17", $borderColor="#FBC02D")

' External systems (actual deployment targets)
UpdateElementStyle(solana_programs, $bgColor="#E8F5E8", $fontColor="#1B5E20", $borderColor="#388E3C")
UpdateElementStyle(smart_meters, $bgColor="#FFF3E0", $fontColor="#E65100", $borderColor="#FF9800")
UpdateElementStyle(rec_authority, $bgColor="#F3E5F5", $fontColor="#8E24AA", $borderColor="#AB47BC")

' Data stores (actual Docker configuration)
UpdateElementStyle(postgres, $bgColor="#ECEFF1", $fontColor="#263238", $borderColor="#607D8B")
UpdateElementStyle(redis, $bgColor="#FFEBEE", $fontColor="#C62828", $borderColor="#F44336")
UpdateElementStyle(kafka, $bgColor="#FFF8E1", $fontColor="#E65100", $borderColor="#FBC02D")

LAYOUT_WITH_LEGEND()
LAYOUT_TOP_DOWN()

@enduml
```

```plantuml
@startuml
!include https://raw.githubusercontent.com/plantuml-stdlib/C4-PlantUML/master/C4_Component.puml

title Data Access & External Integrations - P2P Energy Trading System

'=================================
' Services Layer (Upstream)
'=================================
Container_Boundary(services_upstream, "Business Logic Services") {
    Component(blockchain_service, "Blockchain Service", "Solana Client", "Blockchain interactions")
    Component(user_service, "User Service", "Business Logic", "User management")
    Component(energy_service, "Energy Service", "Business Logic", "Energy data processing")
    Component(notification_service, "Notification Service", "Business Logic", "Real-time notifications")
}

'=================================
' Data Access Layer (Main Focus)
'=================================
Container_Boundary(data_access_layer, "Data Access & Repository Layer") {
    Component(database_repository, "Database Repository", "SQLx/PostgreSQL", "Data access layer for user and trading data")
    Component(cache_repository, "Cache Repository", "Redis Client", "High-performance caching layer")
    Component(event_publisher, "Event Publisher", "Kafka Producer", "Event streaming and real-time updates")
    Component(meter_client, "Smart Meter Client", "HTTP/MQTT Client", "AMI integration for real-time meter data")
    Component(rec_client, "REC Validation Client", "HTTP Client", "Renewable Energy Certificate validation")
}

'=================================
' Middleware Integration
'=================================
Container_Boundary(middleware_integration, "Middleware Data Access") {
    Component(auth_middleware, "Authentication Middleware", "Tower Middleware", "Session and auth data")
    Component(rate_limiter, "Rate Limiting Middleware", "Tower Middleware", "Rate limit tracking")
}

'=================================
' External Systems & Data Stores
'=================================
System_Ext(solana_programs, "Solana Programs", "Registry, Trading, Oracle, Token, Governance programs")
System_Ext(smart_meters, "Smart Meters", "AMI network for energy data")
System_Ext(rec_authority, "REC Authority", "Certificate validation system")

ContainerDb(postgres, "PostgreSQL", "PostgreSQL 18", "Primary data storage")
ContainerDb(redis, "Redis Cache", "Redis 7", "Session and performance cache")
ContainerQueue(kafka, "Message Queue", "Apache Kafka", "Event streaming")

'=================================
' Services to Data Access
'=================================
Rel_D(user_service, database_repository, "User data persistence")
Rel_D(energy_service, database_repository, "Energy consumption data")
Rel_D(user_service, cache_repository, "User session caching")
Rel_D(notification_service, event_publisher, "Real-time notifications")

'=================================
' Middleware to Data Access
'=================================
Rel_D(auth_middleware, cache_repository, "Session management")
Rel_D(rate_limiter, cache_repository, "Rate limit tracking")

'=================================
' External System Integrations
'=================================
Rel_D(blockchain_service, solana_programs, "Smart contract calls", "RPC")
Rel_D(energy_service, meter_client, "Real-time meter data")
Rel_D(energy_service, rec_client, "Certificate validation")
Rel_D(meter_client, smart_meters, "Meter readings", "HTTPS/MQTT")
Rel_D(rec_client, rec_authority, "REC validation", "HTTPS")

'=================================
' Data Persistence
'=================================
Rel_D(database_repository, postgres, "SQL operations", "TCP")
Rel_D(cache_repository, redis, "Caching operations", "TCP")
Rel_D(event_publisher, kafka, "Event streaming", "TCP")

'=================================
' Styling - Data & Integration Theme
'=================================

' Services (upstream)
UpdateElementStyle(blockchain_service, $bgColor="#E0F2F1", $fontColor="#00695C", $borderColor="#009688")
UpdateElementStyle(user_service, $bgColor="#E8F5E8", $fontColor="#2E7D32", $borderColor="#4CAF50")
UpdateElementStyle(energy_service, $bgColor="#F1F8E9", $fontColor="#558B2F", $borderColor="#8BC34A")
UpdateElementStyle(notification_service, $bgColor="#FCE4EC", $fontColor="#C2185B", $borderColor="#E91E63")

' Data access layer - Primary focus
UpdateElementStyle(database_repository, $bgColor="#ECEFF1", $fontColor="#37474F", $borderColor="#607D8B")
UpdateElementStyle(cache_repository, $bgColor="#FFEBEE", $fontColor="#C62828", $borderColor="#F44336")
UpdateElementStyle(event_publisher, $bgColor="#FFF8E1", $fontColor="#E65100", $borderColor="#FBC02D")
UpdateElementStyle(meter_client, $bgColor="#E8EAF6", $fontColor="#3F51B5", $borderColor="#5C6BC0")
UpdateElementStyle(rec_client, $bgColor="#F3E5F5", $fontColor="#8E24AA", $borderColor="#AB47BC")

' Middleware
UpdateElementStyle(auth_middleware, $bgColor="#F3E5F5", $fontColor="#7B1FA2", $borderColor="#9C27B0")
UpdateElementStyle(rate_limiter, $bgColor="#FFF8E1", $fontColor="#F57F17", $borderColor="#FBC02D")

' External systems
UpdateElementStyle(solana_programs, $bgColor="#E8F5E8", $fontColor="#1B5E20", $borderColor="#388E3C")
UpdateElementStyle(smart_meters, $bgColor="#FFF3E0", $fontColor="#E65100", $borderColor="#FF9800")
UpdateElementStyle(rec_authority, $bgColor="#F3E5F5", $fontColor="#8E24AA", $borderColor="#AB47BC")

LAYOUT_WITH_LEGEND()
LAYOUT_TOP_DOWN()

@enduml
```