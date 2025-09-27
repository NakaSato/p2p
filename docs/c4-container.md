# P2P Energy Trading System - C4 Container Diagram

This diagram shows the main applications and data stores within the P2P Energy Trading System with improved logical flow and comprehensive system interactions using C4-PlantUML.

```plantuml
@startuml
!include https://raw.githubusercontent.com/plantuml-stdlib/C4-PlantUML/master/C4_Container.puml

!define DEVICONS https://raw.githubusercontent.com/tupadr3/plantuml-icon-font-sprites/master/devicons
!define FONTAWESOME https://raw.githubusercontent.com/tupadr3/plantuml-icon-font-sprites/master/font-awesome-5
!include DEVICONS/react.puml
!include DEVICONS/rust.puml
!include DEVICONS/postgresql.puml
!include DEVICONS/redis.puml
!include FONTAWESOME/users.puml

title Container Diagram for P2P Energy Trading System - Actual Implementation

'=================================
' Actors & External Systems (From Codebase)
'=================================
Person(engineering_users, "Engineering Users", "Students, Faculty trading energy in Engineering Complex", $sprite="users")
Person(engineering_admin, "Engineering Admin", "Department authority managing PoA consensus and system operations")

System_Ext(solana_network, "Solana Blockchain", "5 deployed Anchor programs: Registry, Trading, Oracle, Energy Token, Governance")
System_Ext(smart_meters, "Smart Meters (15 units)", "METER_001-015 providing AMI data via HTTPS/MQTT")
System_Ext(rec_authority, "REC Authority", "External API for renewable energy certificate validation")

'=================================
' System Boundaries
'=================================

' Monitoring positioned at the top as it observes the entire system
Container_Boundary(observability, "Monitoring & Observability Stack") {
    Container(prometheus, "Prometheus", "Prometheus", "Metrics collection and monitoring")
    Container(grafana, "Grafana", "Grafana", "Dashboards and data visualization")
}

' Simulation layer below monitoring as supporting system for test data injection
Container_Boundary(simulation_layer, "Development & Testing Layer") {
    Container(oracle_simulator, "Oracle Simulator", "Rust", "Market data and clearing simulation", $sprite="rust")
    Container(meter_simulator, "Smart Meter Simulator", "Rust", "Energy consumption/production data generation", $sprite="rust")
}

' Main platform as central focus with horizontal layer arrangement
Container_Boundary(p2p_platform, "P2P Energy Trading Platform - Actual Docker Architecture") {
    ' Layers arranged horizontally from left to right for clear data flow
    Container_Boundary(presentation_layer, "Presentation Layer") {
        Container(nginx, "Nginx Load Balancer", "Nginx", "Reverse proxy, SSL termination (ports 80/443)")
        Container(frontend, "React Frontend", "React/TypeScript/Vite", "Energy trading web interface with Solana Web3 integration", $sprite="react")
    }
    
    Container_Boundary(api_layer, "API Gateway Layer") {
        Container(api_gateway, "API Gateway", "Rust/Axum", "23 REST endpoints: auth, trading, blockchain, meters, analytics", $sprite="rust")
    }
    
    Container_Boundary(data_layer, "Data Storage Layer") {
        ContainerDb(postgres, "PostgreSQL 18", "PostgreSQL", "Users, orders, meter assignments, activities", $sprite="postgresql")
        ContainerDb(redis, "Redis 7", "Redis", "Session cache, rate limiting, JWT tokens", $sprite="redis")
        ContainerQueue(kafka, "Apache Kafka", "Message Queue", "Real-time event streaming and notifications")
    }
    
    ' Blockchain layer - 5 actual Anchor programs
    Container_Boundary(blockchain_programs, "Solana Anchor Programs (5 Programs)") {
        Container(registry_program, "Registry Program", "Anchor/Rust", "User/meter registration with Engineering authority", $sprite="rust")
        Container(energy_token_program, "Energy Token (SPL)", "SPL Token/Anchor", "EnergyToken minting/burning/transfers", $sprite="rust")
        Container(trading_program, "Trading Program", "Anchor/Rust", "Order book, market clearing, trade matching", $sprite="rust")
        Container(oracle_program, "Oracle Program", "Anchor/Rust", "AMI integration, automated market operations", $sprite="rust")
        Container(governance_program, "Governance Program", "Anchor/Rust", "Engineering Department PoA consensus", $sprite="rust")
    }
    
    ' Development & Testing Infrastructure (Docker containers)
    Container_Boundary(development_layer, "Development & Testing Infrastructure") {
        Container(oracle_simulator, "Oracle Simulator", "Rust", "Market data simulation and testing", $sprite="rust")
        Container(meter_simulator, "Smart Meter Simulator", "Rust", "AMI data generation for 10+ meters", $sprite="rust")
        Container(solana_validator, "Solana Validator", "Docker", "Local blockchain node for development")
    }
}

'=================================
' Relationships - Actual Implementation
'=================================

' Core User Flow (Actual REST API endpoints)
Rel_R(engineering_users, nginx, "Web app access", "HTTPS")
Rel_R(engineering_admin, nginx, "Admin dashboard", "HTTPS")
Rel_R(nginx, frontend, "Static React app", "HTTP")
Rel_R(nginx, api_gateway, "API routing", "HTTP/8080")
Rel_R(frontend, api_gateway, "23 REST endpoints", "JSON/HTTPS")

' Data Layer Interactions (Actual database connections)
Rel_R(api_gateway, postgres, "User data, orders, activities", "PostgreSQL/5432")
Rel_R(api_gateway, redis, "Session cache, rate limits", "Redis/6379")
Rel_R(api_gateway, kafka, "Event streaming", "Kafka/9092")

' External System Integrations (Real APIs)
Rel_R(api_gateway, smart_meters, "AMI data ingestion", "HTTPS/MQTT")
Rel_R(api_gateway, rec_authority, "REC validation API", "HTTPS/REST")

' Blockchain Program Interactions (Anchor CPI calls)
Rel_D(api_gateway, registry_program, "User/meter registration", "Solana RPC")
Rel_D(api_gateway, energy_token_program, "Token mint/burn/transfer", "Solana RPC")
Rel_D(api_gateway, trading_program, "Order management", "Solana RPC")
Rel_D(api_gateway, oracle_program, "AMI data submission", "Solana RPC")
Rel_D(api_gateway, governance_program, "PoA validation", "Solana RPC")

' All Anchor programs interact with Solana
Rel_D(registry_program, solana_network, "Account storage", "Blockchain")
Rel_D(energy_token_program, solana_network, "SPL token operations", "Blockchain")
Rel_D(trading_program, solana_network, "Order book state", "Blockchain")
Rel_D(oracle_program, solana_network, "Oracle data feeds", "Blockchain")
Rel_D(governance_program, solana_network, "PoA consensus", "Blockchain")

' Development & Testing Infrastructure
Rel_D(meter_simulator, kafka, "Test meter data", "Events")
Rel_D(oracle_simulator, oracle_program, "Market simulation", "RPC")
Rel_D(meter_simulator, postgres, "Simulated readings", "SQL")

' Monitoring Interactions (Upward Flow)
Rel_U(api_gateway, prometheus, "Application metrics", "HTTP")
Rel_U(postgres, prometheus, "Database metrics", "HTTP")
Rel_U(kafka, prometheus, "Streaming metrics", "HTTP")
Rel_U(solana_network, prometheus, "On-chain metrics", "HTTP")
Rel_R(prometheus, grafana, "Metrics data source", "HTTP")

'=================================
' Layout and Styling
'=================================

' Enhanced layout directives for better positioning
!define DIRECTION top to bottom direction
LAYOUT_TOP_DOWN()
LAYOUT_WITH_LEGEND()

' Custom styling for different component types
UpdateElementStyle(users, $bgColor="#E8F5E8", $fontColor="#1B5E20", $borderColor="#4CAF50")
UpdateElementStyle(admin, $bgColor="#FFE6CC", $fontColor="#BF360C", $borderColor="#FF6F00")

UpdateElementStyle(nginx, $bgColor="#F3E5F5", $fontColor="#4A148C", $borderColor="#7B1FA2")
UpdateElementStyle(frontend, $bgColor="#E0F2F1", $fontColor="#00695C", $borderColor="#009688")
UpdateElementStyle(api_gateway, $bgColor="#E3F2FD", $fontColor="#0D47A1", $borderColor="#1976D2")

UpdateElementStyle(postgres, $bgColor="#ECEFF1", $fontColor="#263238", $borderColor="#607D8B")
UpdateElementStyle(timescale, $bgColor="#E8EAF6", $fontColor="#3F51B5", $borderColor="#5C6BC0")
UpdateElementStyle(redis, $bgColor="#FFEBEE", $fontColor="#C62828", $borderColor="#F44336")
UpdateElementStyle(kafka, $bgColor="#FFF8E1", $fontColor="#E65100", $borderColor="#FBC02D")

UpdateElementStyle(registry_program, $bgColor="#F1F8E9", $fontColor="#33691E", $borderColor="#689F38")
UpdateElementStyle(energy_token_program, $bgColor="#E8F5E8", $fontColor="#1B5E20", $borderColor="#388E3C")
UpdateElementStyle(trading_program, $bgColor="#E0F7FA", $fontColor="#00695C", $borderColor="#00ACC1")
UpdateElementStyle(oracle_program, $bgColor="#FFF3E0", $fontColor="#E65100", $borderColor="#FF9800")
UpdateElementStyle(governance_program, $bgColor="#FCE4EC", $fontColor="#AD1457", $borderColor="#E91E63")

UpdateElementStyle(prometheus, $bgColor="#FFF8E1", $fontColor="#F57F17", $borderColor="#FBC02D")
UpdateElementStyle(grafana, $bgColor="#E1F5FE", $fontColor="#0277BD", $borderColor="#03A9F4")

UpdateElementStyle(oracle_simulator, $bgColor="#FFF3E0", $fontColor="#E65100", $borderColor="#FF9800")
UpdateElementStyle(meter_simulator, $bgColor="#F1F8E9", $fontColor="#33691E", $borderColor="#689F38")

' External systems styling
UpdateElementStyle(solana_network, $bgColor="#E8F5E8", $fontColor="#1B5E20", $borderColor="#388E3C")
UpdateElementStyle(smart_meters, $bgColor="#FFF3E0", $fontColor="#E65100", $borderColor="#FF9800")
UpdateElementStyle(rec_authority, $bgColor="#F3E5F5", $fontColor="#7B1FA2", $borderColor="#9C27B0")

@enduml
```