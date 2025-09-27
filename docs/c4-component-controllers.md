# P2P Energy Trading System - C4 Component Diagram: Controllers Layer (Actual Implementation)

This diagram focuses on the actual Controllers layer implementation in the API Gateway, showing the 23 REST endpoints and their interactions with middleware and services based on the real codebase.

```plantuml
@startuml
!include https://raw.githubusercontent.com/plantuml-stdlib/C4-PlantUML/master/C4_Component.puml

title Controllers Layer - P2P Energy Trading System API Gateway (Actual Implementation)
!define DIRECTION top to bottom direction

'=================================
' External Entry Points
'=================================
Container_Boundary(external_layer, "External Systems & Load Balancing") {
    Container(frontend, "React Frontend", "React/TypeScript/Vite", "Energy trading web interface\n• User authentication\n• Trading dashboard\n• Energy monitoring\n• Analytics views")
    Container(nginx, "Nginx Load Balancer", "Nginx", "Reverse proxy and load balancing\n• SSL termination (443→80)\n• Request routing\n• Health checks")
}

'=================================
' API Gateway Controllers - Actual Implementation (main.rs)
'=================================
Container_Boundary(controllers_layer, "API Gateway Controllers - Rust/Axum (23 Endpoints)") {
    
    Component(health_controller, "Health Controller", "Axum Handler", "System health monitoring endpoints\n• /health - Basic health check\n• /health/ready - Readiness probe\n• /health/live - Liveness probe")
    
    Component(auth_controller, "Authentication Controller", "Axum Handler", "JWT authentication and user session management\n• POST /auth/login - User login\n• GET /auth/profile - Get user profile\n• POST /auth/profile - Update profile\n• POST /auth/password - Change password")
    
    Component(user_controller, "User Management Controller", "Axum Handler", "Enhanced user registration and profile management\n• POST /auth/register - Enhanced registration\n• POST /user/wallet - Update wallet address\n• DELETE /user/wallet - Remove wallet\n• GET /user/activity - User activity history")
    
    Component(admin_controller, "Admin Management Controller", "Axum Handler", "Administrative user operations\n• GET /users/:id - Get specific user\n• PUT /users/:id - Admin update user\n• POST /users/:id/deactivate - Deactivate user\n• POST /users/:id/reactivate - Reactivate user\n• GET /users - List all users")
    
    Component(blockchain_controller, "Blockchain Controller", "Axum Handler", "Solana blockchain integration endpoints\n• POST /blockchain/transactions - Submit transaction\n• GET /blockchain/transactions - Transaction history\n• GET /blockchain/transactions/:signature - Transaction status\n• POST /blockchain/programs/:name - Program interaction\n• GET /blockchain/accounts/:address - Account info\n• GET /blockchain/network - Network status")
    
    Component(trading_controller, "Trading Controller", "Axum Handler", "Energy trading order management\n• POST /trading/orders - Create trading order\n• GET /trading/orders - Get user orders\n• GET /trading/market - Market data\n• GET /trading/stats - Trading statistics")
    
    Component(meters_controller, "Energy Meters Controller", "Axum Handler", "AMI smart meter data management\n• POST /meters/readings - Submit energy reading\n• GET /meters/readings - Get readings\n• GET /meters/readings/:id - Specific reading\n• GET /meters/aggregated - Aggregated data")
    
    Component(analytics_controller, "Analytics Controller", "Axum Handler", "Energy and trading analytics\n• GET /analytics/user - User analytics\n• GET /analytics/system - System analytics")
    
    Component(department_controller, "Department Controller", "Axum Handler", "Department information service\n• GET /departments/:department - Department info")
}

'=================================
' Middleware Layer - Actual Implementation
'=================================
Container_Boundary(middleware_layer, "Middleware Layer - Tower/Axum") {
    Component(auth_middleware, "Authentication Middleware", "Tower Middleware", "JWT token validation and authorization\n• JWT verification\n• Role-based access control\n• API key validation")
    
    Component(cors_middleware, "CORS Middleware", "Tower HTTP", "Cross-Origin Resource Sharing\n• Permissive CORS (dev)\n• Request origin validation")
    
    Component(trace_middleware, "Tracing Middleware", "Tower HTTP", "Request/response logging\n• HTTP request tracing\n• Performance monitoring")
    
    Component(timeout_middleware, "Timeout Middleware", "Tower HTTP", "Request timeout management\n• 30-second timeout\n• Connection management")
}

'=================================
' Service Layer - Business Logic Implementation
'=================================
Container_Boundary(service_layer, "Service Layer - Business Logic (Rust)") {
    Component(jwt_service, "JWT Service", "Authentication Service", "JWT token management\n• Token generation\n• Token validation\n• User claims extraction")
    
    Component(api_key_service, "API Key Service", "Authentication Service", "API key management\n• Key generation\n• Key validation\n• Service authentication")
    
    Component(blockchain_service, "Blockchain Service", "Solana Integration", "Solana blockchain interactions\n• RPC calls to 5 programs\n• Transaction submission\n• Account management")
}

'=================================
' Application State - Shared Resources
'=================================
Container_Boundary(app_state, "Application State (AppState)") {
    Component(postgres_pool, "PostgreSQL Pool", "SQLx Pool", "Database connection pool\n• User management\n• Trading orders\n• Activity logs")
    
    Component(redis_client, "Redis Client", "Redis Connection", "Cache and session management\n• JWT session cache\n• Rate limiting\n• Performance optimization")
    
    Component(config, "Configuration", "Environment Config", "Application configuration\n• Database URLs\n• JWT secrets\n• Environment settings")
}

'=================================
' External Request Flow - Actual Endpoints
'=================================
Rel_D(nginx, health_controller, "Health checks", "GET /health, /health/ready, /health/live")
Rel_D(nginx, auth_controller, "Authentication", "POST /auth/login")
Rel_D(nginx, user_controller, "Registration", "POST /auth/register")

Rel_D(frontend, auth_controller, "User authentication", "Login forms, profile management")
Rel_D(frontend, user_controller, "User management", "Registration, wallet updates")
Rel_D(frontend, trading_controller, "Trading interface", "Order placement, market data")
Rel_D(frontend, meters_controller, "Meter management", "Energy readings, AMI data")
Rel_D(frontend, analytics_controller, "Analytics dashboard", "User and system analytics")
Rel_D(frontend, blockchain_controller, "Blockchain operations", "Transaction submission, program interaction")

'=================================
' Controller-Middleware Integration
'=================================
Rel_R(auth_controller, auth_middleware, "Session validation")
Rel_R(user_controller, auth_middleware, "Authorization check")
Rel_R(admin_controller, auth_middleware, "Admin role verification")
Rel_R(trading_controller, auth_middleware, "User authentication")
Rel_R(meters_controller, auth_middleware, "Meter access control")
Rel_R(analytics_controller, auth_middleware, "Analytics authorization")
Rel_R(blockchain_controller, auth_middleware, "Blockchain access control")

'=================================
' Controller-Service Integration
'=================================
Rel_D(auth_controller, jwt_service, "JWT operations")
Rel_D(auth_controller, api_key_service, "API key validation")
Rel_D(blockchain_controller, blockchain_service, "Solana integration")

'=================================
' Service-State Integration
'=================================
Rel_D(jwt_service, redis_client, "Session caching")
Rel_D(auth_controller, postgres_pool, "User data access")
Rel_D(trading_controller, postgres_pool, "Order management")
Rel_D(meters_controller, postgres_pool, "Meter data storage")
Rel_D(analytics_controller, postgres_pool, "Analytics queries")

'=================================
' Enhanced Styling - Based on Actual Function
'=================================

' External systems
UpdateElementStyle(frontend, $bgColor="#E8F4FD", $fontColor="#1565C0", $borderColor="#1976D2")
UpdateElementStyle(nginx, $bgColor="#F3E5F5", $fontColor="#4A148C", $borderColor="#7B1FA2")

' Controllers - Color-coded by actual functionality
UpdateElementStyle(health_controller, $bgColor="#F1F8E9", $fontColor="#33691E", $borderColor="#689F38")
UpdateElementStyle(auth_controller, $bgColor="#E3F2FD", $fontColor="#0D47A1", $borderColor="#1976D2")
UpdateElementStyle(user_controller, $bgColor="#E8F5E8", $fontColor="#2E7D32", $borderColor="#388E3C")
UpdateElementStyle(admin_controller, $bgColor="#FFEBEE", $fontColor="#C62828", $borderColor="#D32F2F")
UpdateElementStyle(blockchain_controller, $bgColor="#E0F2F1", $fontColor="#00695C", $borderColor="#009688")
UpdateElementStyle(trading_controller, $bgColor="#FFF3E0", $fontColor="#E65100", $borderColor="#F57C00")
UpdateElementStyle(meters_controller, $bgColor="#F3E5F5", $fontColor="#6A1B9A", $borderColor="#7B1FA2")
UpdateElementStyle(analytics_controller, $bgColor="#E1F5FE", $fontColor="#0277BD", $borderColor="#0288D1")
UpdateElementStyle(department_controller, $bgColor="#FCE4EC", $fontColor="#C2185B", $borderColor="#E91E63")

' Middleware - Infrastructure styling
UpdateElementStyle(auth_middleware, $bgColor="#E8EAF6", $fontColor="#3F51B5", $borderColor="#5C6BC0")
UpdateElementStyle(cors_middleware, $bgColor="#FFF8E1", $fontColor="#F57F17", $borderColor="#FBC02D")
UpdateElementStyle(trace_middleware, $bgColor="#ECEFF1", $fontColor="#455A64", $borderColor="#607D8B")
UpdateElementStyle(timeout_middleware, $bgColor="#FFEBEE", $fontColor="#C62828", $borderColor="#F44336")

' Services - Business logic styling
UpdateElementStyle(jwt_service, $bgColor="#E8F5E8", $fontColor="#1B5E20", $borderColor="#2E7D32")
UpdateElementStyle(api_key_service, $bgColor="#E1F5FE", $fontColor="#0277BD", $borderColor="#0288D1")
UpdateElementStyle(blockchain_service, $bgColor="#E0F2F1", $fontColor="#00695C", $borderColor="#009688")

' Application state - Data layer styling
UpdateElementStyle(postgres_pool, $bgColor="#ECEFF1", $fontColor="#37474F", $borderColor="#607D8B")
UpdateElementStyle(redis_client, $bgColor="#FFEBEE", $fontColor="#C62828", $borderColor="#F44336")
UpdateElementStyle(config, $bgColor="#F3E5F5", $fontColor="#7B1FA2", $borderColor="#9C27B0")

'=================================
' Layout Configuration
'=================================
LAYOUT_WITH_LEGEND()
LAYOUT_TOP_DOWN()

@enduml
```

```plantuml
@startuml
!include https://raw.githubusercontent.com/plantuml-stdlib/C4-PlantUML/master/C4_Component.puml

title Controllers Layer - P2P Energy Trading System API Gateway
!define DIRECTION top to bottom direction

'=================================
' External Entry Points
'=================================
Container_Boundary(external_layer, "External Systems & Load Balancing") {
    Container(frontend, "Web Application", "React/TypeScript", "User interface for energy trading\n• Authentication forms\n• Trading dashboard\n• Energy monitoring")
    Container(nginx, "Load Balancer", "Nginx", "Reverse proxy and load balancing\n• SSL termination\n• Request routing\n• Health checks")
}

'=================================
' API Gateway Controllers - Main Focus
'=================================
Container_Boundary(controllers_layer, "API Gateway Controllers - Rust/Axum") {
    
    Component(auth_controller, "Authentication Controller", "Axum Handler", "JWT token validation and user session management\n• Login/logout endpoints\n• Token refresh\n• Session validation\n• Password reset")
    
    Component(user_controller, "User Management Controller", "Axum Handler", "User registration, profile management, and KYC\n• User registration\n• Profile updates\n• KYC verification\n• Account settings")
    
    Component(trading_controller, "Trading Controller", "Axum Handler", "Order creation, matching, and trade execution\n• Create buy/sell orders\n• Order history\n• Trade matching\n• Price discovery")
    
    Component(energy_controller, "Energy Data Controller", "Axum Handler", "Meter readings, consumption tracking, and analytics\n• Real-time meter data\n• Consumption analytics\n• Energy forecasting\n• REC validation")
    
    Component(admin_controller, "Admin Controller", "Axum Handler", "System administration and REC validator operations\n• System monitoring\n• User management\n• Validator operations\n• Compliance reporting")
}

'=================================
' Middleware Layer - Security & Cross-cutting Concerns
'=================================
Container_Boundary(middleware_layer, "Middleware Layer - Tower/Axum") {
    Component(auth_middleware, "Authentication Middleware", "Tower Middleware", "Request authentication and authorization\n• JWT validation\n• Role-based access control\n• API key verification")
    
    Component(rate_limiter, "Rate Limiting Middleware", "Tower Middleware", "API rate limiting and DDoS protection\n• Request throttling\n• IP-based limiting\n• Burst protection")
    
    Component(audit_middleware, "Audit Middleware", "Tower Middleware", "Security logging and compliance tracking\n• Request logging\n• Security events\n• Compliance audit trails")
}

'=================================
' Service Layer - Business Logic
'=================================
Container_Boundary(service_layer, "Service Layer - Business Logic") {
    Component(user_service, "User Service", "Business Logic", "User account management and validation\n• Account creation\n• Profile validation\n• Identity verification")
    
    Component(trading_service, "Trading Service", "Business Logic", "Order management and trading operations\n• Order validation\n• Trade execution\n• Market making")
    
    Component(energy_service, "Energy Service", "Business Logic", "Energy data processing and REC validation\n• Meter data processing\n• REC token minting\n• Energy analytics")
    
    Component(oracle_service, "Oracle Service", "Business Logic", "External data integration and market clearing\n• Price feeds\n• Market data\n• External API integration")
}

'=================================
' External Request Flow - Enhanced Detail
'=================================
Rel_D(nginx, auth_controller, "Authentication requests", "HTTPS\n/auth/login, /auth/logout\n/auth/refresh")
Rel_D(nginx, user_controller, "User management", "HTTPS\n/users/register, /users/profile\n/users/kyc")
Rel_D(nginx, trading_controller, "Trading operations", "HTTPS\n/orders/create, /orders/cancel\n/trades/history")
Rel_D(nginx, energy_controller, "Energy data", "HTTPS\n/energy/meters, /energy/consumption\n/energy/analytics")
Rel_D(nginx, admin_controller, "Admin operations", "HTTPS\n/admin/users, /admin/system\n/admin/validator")

Rel_D(frontend, auth_controller, "Authentication flows", "JSON/HTTPS\nLogin forms, token management")
Rel_D(frontend, user_controller, "User interactions", "JSON/HTTPS\nProfile management, settings")
Rel_D(frontend, trading_controller, "Trading interface", "JSON/HTTPS\nOrder placement, trade history")
Rel_D(frontend, energy_controller, "Energy monitoring", "JSON/HTTPS\nMeter dashboards, analytics")

'=================================
' Controller-Middleware Integration
'=================================
Rel_R(auth_controller, auth_middleware, "Session validation", "Validates JWT tokens\nManages user sessions")
Rel_R(user_controller, auth_middleware, "Authorization check", "Verifies user permissions\nRole-based access")
Rel_R(trading_controller, rate_limiter, "Request throttling", "Prevents trading spam\nAPI abuse protection")
Rel_R(energy_controller, audit_middleware, "Compliance logging", "Tracks energy data access\nRegulatory compliance")
Rel_R(admin_controller, audit_middleware, "Security monitoring", "Admin action logging\nSecurity event tracking")

'=================================
' Controller-Service Integration
'=================================
Rel_D(auth_controller, user_service, "User authentication", "Validates credentials\nManages user sessions")
Rel_D(user_controller, user_service, "User operations", "Profile management\nKYC processing")
Rel_D(trading_controller, trading_service, "Trading logic", "Order processing\nTrade execution")
Rel_D(energy_controller, energy_service, "Energy processing", "Meter data handling\nREC validation")
Rel_D(admin_controller, oracle_service, "System administration", "External data management\nSystem monitoring")

'=================================
' Enhanced Styling - Professional Theme with Better Visual Hierarchy
'=================================

' External systems - Entry point styling
UpdateElementStyle(frontend, $bgColor="#E8F4FD", $fontColor="#1565C0", $borderColor="#1976D2")
UpdateElementStyle(nginx, $bgColor="#F3E5F5", $fontColor="#4A148C", $borderColor="#7B1FA2")

' Controllers - Color-coded by functionality
UpdateElementStyle(auth_controller, $bgColor="#E3F2FD", $fontColor="#0D47A1", $borderColor="#1976D2")
UpdateElementStyle(user_controller, $bgColor="#E8F5E8", $fontColor="#2E7D32", $borderColor="#388E3C")
UpdateElementStyle(trading_controller, $bgColor="#FFF3E0", $fontColor="#E65100", $borderColor="#F57C00")
UpdateElementStyle(energy_controller, $bgColor="#F3E5F5", $fontColor="#6A1B9A", $borderColor="#7B1FA2")
UpdateElementStyle(admin_controller, $bgColor="#FFEBEE", $fontColor="#C62828", $borderColor="#D32F2F")

' Middleware - Security-focused styling
UpdateElementStyle(auth_middleware, $bgColor="#E8EAF6", $fontColor="#3F51B5", $borderColor="#5C6BC0")
UpdateElementStyle(rate_limiter, $bgColor="#FFF8E1", $fontColor="#F57F17", $borderColor="#FBC02D")
UpdateElementStyle(audit_middleware, $bgColor="#ECEFF1", $fontColor="#455A64", $borderColor="#607D8B")

' Services - Business logic styling
UpdateElementStyle(user_service, $bgColor="#E8F5E8", $fontColor="#1B5E20", $borderColor="#2E7D32")
UpdateElementStyle(trading_service, $bgColor="#E1F5FE", $fontColor="#0277BD", $borderColor="#0288D1")
UpdateElementStyle(energy_service, $bgColor="#F1F8E9", $fontColor="#33691E", $borderColor="#558B2F")
UpdateElementStyle(oracle_service, $bgColor="#FFF3E0", $fontColor="#BF360C", $borderColor="#E65100")

'=================================
' Layout Configuration - Optimized for Clarity
'=================================
LAYOUT_WITH_LEGEND()
LAYOUT_TOP_DOWN()

@enduml
```