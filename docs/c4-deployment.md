# P2P Energy Trading System - C4 Deployment Diagram (Actual Docker Implementation)

This diagram shows the actual deployment architecture based on the docker-compose.yml file, reflecting the real production-ready containerized environment.

```plantuml
@startuml
!include https://raw.githubusercontent.com/plantuml-stdlib/C4-PlantUML/master/C4_Deployment.puml

!define DEVICONS https://raw.githubusercontent.com/tupadr3/plantuml-icon-font-sprites/master/devicons
!define FONTAWESOME https://raw.githubusercontent.com/tupadr3/plantuml-icon-font-sprites/master/font-awesome-5
!include DEVICONS/react.puml
!include DEVICONS/rust.puml
!include DEVICONS/postgresql.puml
!include DEVICONS/redis.puml
!include DEVICONS/docker.puml
!include DEVICONS/nginx.puml
!include FONTAWESOME/laptop.puml
!include FONTAWESOME/server.puml
!include FONTAWESOME/database.puml
!include FONTAWESOME/chart_line.puml
!include FONTAWESOME/bolt.puml

title Deployment Diagram - P2P Energy Trading System (Docker Compose Architecture)

'=================================
' User Device Layer
'=================================
Deployment_Node(user_devices, "Engineering Complex Users", "Campus Network") {
    Deployment_Node(student_device, "Engineering Student Device", "macOS/Windows/Linux", $sprite="laptop") {
        Container(web_browser, "Web Browser", "Chrome/Firefox", "React web app for energy trading")
    }
    Deployment_Node(admin_device, "Engineering Admin Device", "Secure Workstation", $sprite="laptop") {
        Container(admin_dashboard, "Admin Interface", "Web Browser", "System administration and PoA validator operations")
    }
}

'=================================
' Docker Infrastructure (Based on actual docker-compose.yml)
'=================================
Deployment_Node(docker_host, "Docker Host Environment", "Production Server") {
    
    '--- Load Balancer Layer ---
    Deployment_Node(nginx_service, "p2p-nginx", "Docker Container", $sprite="docker") {
        Container(nginx_lb, "Nginx Load Balancer", "Nginx", "Reverse proxy (80/443) → Frontend & API", $sprite="nginx")
    }
    
    '--- Frontend Layer ---
    Deployment_Node(frontend_service, "p2p-frontend", "Docker Container", $sprite="docker") {
        Container(react_app, "React Application", "React/TypeScript/Vite", "Static web app served on port 80", $sprite="react")
    }
    
    '--- API Gateway Layer ---
    Deployment_Node(api_service, "p2p-api-gateway", "Docker Container", $sprite="docker") {
        Container(rust_api, "API Gateway", "Rust/Axum", "23 REST endpoints on port 8080", $sprite="rust")
    }
    
    '--- Blockchain Layer ---
    Deployment_Node(solana_service, "p2p-anchor-dev", "Docker Container", $sprite="docker") {
        Container(solana_validator, "Solana Validator", "Anchor Development", "Local blockchain (8898:8899, 8901:8900)")
        Container(anchor_programs, "5 Anchor Programs", "Rust/Anchor", "Registry, Token, Trading, Oracle, Governance")
    }
    
    '--- Smart Contract Deployment ---
    Deployment_Node(contract_service, "p2p-contact", "Docker Container", $sprite="docker") {
        Container(contract_deployer, "Contract Deployer", "Anchor CLI", "Automated program deployment")
    }
    
    '--- Database Layer ---
    Deployment_Node(postgres_service, "p2p-postgres", "Docker Container", $sprite="docker") {
        Container(postgres_db, "PostgreSQL 18", "PostgreSQL", "Main database on port 5432", $sprite="postgresql")
    }
    
    Deployment_Node(redis_service, "p2p-redis", "Docker Container", $sprite="docker") {
        Container(redis_cache, "Redis 7", "Redis", "Session cache on port 6379", $sprite="redis")
    }
    
    '--- Message Queue Layer ---
    Deployment_Node(kafka_service, "p2p-kafka", "Docker Container", $sprite="docker") {
        Container(kafka_broker, "Apache Kafka", "Confluent Kafka", "Message broker on port 9092")
    }
    
    Deployment_Node(zookeeper_service, "p2p-zookeeper", "Docker Container", $sprite="docker") {
        Container(zookeeper, "Zookeeper", "Confluent Zookeeper", "Kafka coordination (port 2181)")
    }
    
    '--- Simulation Layer ---
    Deployment_Node(oracle_sim_service, "p2p-oracle-simulator", "Docker Container", $sprite="docker") {
        Container(oracle_sim, "Oracle Simulator", "Rust", "Market data simulation", $sprite="rust")
    }
    
    Deployment_Node(meter_sim_service, "p2p-smart-meter-simulator", "Docker Container", $sprite="docker") {
        Container(meter_sim, "Smart Meter Simulator", "Rust", "AMI data generation (10 meters)", $sprite="rust")
    }
    
    '--- Monitoring Layer ---
    Deployment_Node(prometheus_service, "p2p-prometheus", "Docker Container", $sprite="docker") {
        Container(prometheus, "Prometheus", "Prometheus", "Metrics collection (port 9090)", $sprite="chart_line")
    }
    
    Deployment_Node(grafana_service, "p2p-grafana", "Docker Container", $sprite="docker") {
        Container(grafana, "Grafana", "Grafana", "Monitoring dashboards (port 3001)", $sprite="chart_line")
    }
}

'=================================
' External Systems (Real Implementation)
'=================================
Deployment_Node(external_systems, "External Systems", "Internet/Campus Network") {
    Deployment_Node(ami_infrastructure, "AMI Infrastructure", "Engineering Complex") {
        Container(smart_meters, "Smart Meters (15 units)", "IoT Devices", "METER_001-015 energy data collection", $sprite="bolt")
        Container(meter_gateway, "AMI Gateway", "Industrial Gateway", "Data aggregation and protocol translation", $sprite="server")
    }
    
    Deployment_Node(rec_services, "REC Services", "External APIs") {
        Container(rec_authority, "REC Authority API", "External Service", "Renewable energy certificate validation")
    }
    
    Deployment_Node(blockchain_network, "Solana Network", "Blockchain Infrastructure") {
        Container(devnet, "Solana Devnet", "Blockchain Network", "Alternative deployment target")
    }
}

'=================================
' Port Mappings & Relationships (From docker-compose.yml)
'=================================

' User Access
Rel_R(web_browser, nginx_lb, "HTTPS/HTTP", "Port 80/443")
Rel_R(admin_dashboard, nginx_lb, "Admin access", "Port 80/443")

' Load Balancer Routing
Rel_D(nginx_lb, react_app, "Static content", "Port 80")
Rel_D(nginx_lb, rust_api, "API requests", "Port 8080")

' Frontend to API
Rel_D(react_app, rust_api, "REST API calls", "JSON/HTTP")

' API to Data Layer
Rel_D(rust_api, postgres_db, "Database operations", "Port 5432")
Rel_D(rust_api, redis_cache, "Cache operations", "Port 6379")
Rel_D(rust_api, kafka_broker, "Event streaming", "Port 9092")

' API to Blockchain
Rel_D(rust_api, solana_validator, "Blockchain RPC", "Port 8898")
Rel_D(rust_api, anchor_programs, "Program interactions", "CPI calls")

' Contract Deployment
Rel_D(contract_deployer, solana_validator, "Program deployment", "Anchor deploy")
Rel_D(contract_deployer, anchor_programs, "Program management", "Anchor CLI")

' External Integrations
Rel_U(rust_api, smart_meters, "AMI data", "HTTPS/MQTT")
Rel_U(rust_api, rec_authority, "REC validation", "HTTPS/API")
Rel_U(solana_validator, devnet, "Alternative deployment", "Blockchain sync")

' Simulation Infrastructure
Rel_D(oracle_sim, kafka_broker, "Market events", "Kafka events")
Rel_D(meter_sim, kafka_broker, "Meter readings", "Event streaming")
Rel_D(oracle_sim, solana_validator, "Market data injection", "RPC calls")
Rel_U(smart_meters, meter_gateway, "Meter protocols", "Modbus/DNP3")

' Message Queue Dependencies
Rel_D(kafka_broker, zookeeper, "Coordination", "Port 2181")

' Monitoring
Rel_U(rust_api, prometheus, "Application metrics", "Port 9090")
Rel_U(postgres_db, prometheus, "Database metrics", "Postgres exporter")
Rel_U(solana_validator, prometheus, "Blockchain metrics", "RPC metrics")
Rel_D(prometheus, grafana, "Metrics data source", "Port 3001")

'=================================
' Docker Volume Mappings (From compose)
'=================================
note right of postgres_db : Persistent Volume:\npostgres_data
note right of redis_cache : Persistent Volume:\nredis_data  
note right of kafka_broker : Persistent Volume:\nkafka_data
note right of solana_validator : Persistent Volume:\nsolana_ledger
note right of contract_deployer : Artifact Volume:\ncontact_artifacts

'=================================
' Styling - Infrastructure Theme
'=================================

' User devices
UpdateElementStyle(web_browser, $bgColor="#E8F8F5", $fontColor="#1B5E20", $borderColor="#4CAF50")
UpdateElementStyle(admin_dashboard, $bgColor="#FFE6CC", $fontColor="#BF360C", $borderColor="#FF6F00")

' Infrastructure services
UpdateElementStyle(nginx_lb, $bgColor="#F3E5F5", $fontColor="#4A148C", $borderColor="#7B1FA2")
UpdateElementStyle(react_app, $bgColor="#E0F2F1", $fontColor="#00695C", $borderColor="#009688")
UpdateElementStyle(rust_api, $bgColor="#E3F2FD", $fontColor="#0D47A1", $borderColor="#1976D2")

' Blockchain layer
UpdateElementStyle(solana_validator, $bgColor="#E8F5E8", $fontColor="#1B5E20", $borderColor="#388E3C")
UpdateElementStyle(anchor_programs, $bgColor="#F1F8E9", $fontColor="#33691E", $borderColor="#689F38")
UpdateElementStyle(contract_deployer, $bgColor="#E0F7FA", $fontColor="#00695C", $borderColor="#00ACC1")

' Data layer
UpdateElementStyle(postgres_db, $bgColor="#ECEFF1", $fontColor="#263238", $borderColor="#607D8B")
UpdateElementStyle(redis_cache, $bgColor="#FFEBEE", $fontColor="#C62828", $borderColor="#F44336")
UpdateElementStyle(kafka_broker, $bgColor="#FFF8E1", $fontColor="#E65100", $borderColor="#FBC02D")
UpdateElementStyle(zookeeper, $bgColor="#FFF3E0", $fontColor="#F57F17", $borderColor="#FBC02D")

' Simulation & Testing
UpdateElementStyle(oracle_sim, $bgColor="#FFF3E0", $fontColor="#E65100", $borderColor="#FF9800")
UpdateElementStyle(meter_sim, $bgColor="#F1F8E9", $fontColor="#33691E", $borderColor="#689F38")

' Monitoring
UpdateElementStyle(prometheus, $bgColor="#FFF3E0", $fontColor="#E65100", $borderColor="#FF9800")
UpdateElementStyle(grafana, $bgColor="#E1F5FE", $fontColor="#0277BD", $borderColor="#03A9F4")

' External systems
UpdateElementStyle(smart_meters, $bgColor="#FFF3E0", $fontColor="#E65100", $borderColor="#FF9800")
UpdateElementStyle(meter_gateway, $bgColor="#F3E5F5", $fontColor="#7B1FA2", $borderColor="#9C27B0")
UpdateElementStyle(rec_authority, $bgColor="#E0F2F1", $fontColor="#00796B", $borderColor="#26A69A")
UpdateElementStyle(devnet, $bgColor="#E8F5E8", $fontColor="#1B5E20", $borderColor="#388E3C")

'=================================
' Layout Configuration
'=================================
LAYOUT_WITH_LEGEND()
LAYOUT_TOP_DOWN()

@enduml
```

```plantuml
@startuml
!include https://raw.githubusercontent.com/plantuml-stdlib/C4-PlantUML/master/C4_Deployment.puml

!define DEVICONS https://raw.githubusercontent.com/tupadr3/plantuml-icon-font-sprites/master/devicons
!define FONTAWESOME https://raw.githubusercontent.com/tupadr3/plantuml-icon-font-sprites/master/font-awesome-5
!include DEVICONS/react.puml
!include DEVICONS/rust.puml
!include DEVICONS/postgresql.puml
!include DEVICONS/redis.puml
!include DEVICONS/docker.puml
!include DEVICONS/nginx.puml
!include FONTAWESOME/laptop.puml
!include FONTAWESOME/server.puml
!include FONTAWESOME/database.puml
!include FONTAWESOME/chart_line.puml
!include FONTAWESOME/bolt.puml

title Deployment Diagram for P2P Energy Trading System - Production Environment

'=================================
' User Device Layer
'=================================
Deployment_Node(user_devices, "User Devices", "Campus Network") {
    Deployment_Node(student_laptop, "Student Laptop", "macOS/Windows/Linux", $sprite="laptop") {
        Container(web_browser, "Web Browser", "Chrome/Firefox/Safari", "Web3-enabled browser for energy trading")
    }
    Deployment_Node(admin_workstation, "Admin Workstation", "Secure Workstation", $sprite="laptop") {
        Container(admin_interface, "Admin Interface", "Web Browser + CLI", "System administration and REC validator operations")
    }
}

'=================================
' Campus Infrastructure
'=================================
Deployment_Node(campus_infrastructure, "Campus Infrastructure", "University Data Center") {
    
    '--- Load Balancer Layer ---
    Deployment_Node(load_balancer_node, "Load Balancer", "Ubuntu 22.04 LTS", $sprite="server") {
        Deployment_Node(nginx_container, "Nginx Container", "Docker", $sprite="docker") {
            Container(nginx, "Nginx", "Nginx 1.24", "Reverse proxy, SSL termination, and load balancing", $sprite="nginx")
        }
    }
    
    '--- Application Server Layer ---
    Deployment_Node(app_servers, "Application Servers x3", "Ubuntu 22.04 LTS", $sprite="server") {
        Deployment_Node(frontend_container, "Frontend Container", "Docker", $sprite="docker") {
            Container(frontend_app, "React Application", "Node.js/Nginx", "Web interface for energy trading", $sprite="react")
        }
        Deployment_Node(api_container, "API Gateway Container", "Docker", $sprite="docker") {
            Container(api_gateway, "API Gateway", "Rust/Axum", "REST API and blockchain integration", $sprite="rust")
        }
    }
    
    '--- Blockchain Layer ---
    Deployment_Node(blockchain_node, "Blockchain Node", "Ubuntu 22.04 LTS", $sprite="server") {
        Deployment_Node(solana_container, "Solana Validator Container", "Docker", $sprite="docker") {
            Container(solana_validator, "Solana Validator", "Solana 1.17+", "Local blockchain validator for PoA network", $sprite="bolt")
            ContainerDb(solana_ledger, "Blockchain Ledger", "RocksDB", "Stores blockchain state and transactions", $sprite="database")
        }
    }
    
    '--- Message Queue Layer ---
    Deployment_Node(message_queue_node, "Message Queue Node", "Ubuntu 22.04 LTS", $sprite="server") {
        Deployment_Node(kafka_container, "Kafka Container", "Docker", $sprite="docker") {
            ContainerQueue(kafka_broker, "Apache Kafka", "Kafka 3.5", "Event streaming and real-time data processing")
        }
    }
    
    '--- Database Layer ---
    Deployment_Node(database_cluster, "Database Cluster", "Ubuntu 22.04 LTS", $sprite="server") {
        Deployment_Node(postgres_primary, "PostgreSQL Primary", "Docker", $sprite="docker") {
            ContainerDb(postgres_db, "PostgreSQL Database", "PostgreSQL 18", "Primary data storage with TimescaleDB", $sprite="postgresql")
        }
        Deployment_Node(postgres_replica, "PostgreSQL Replica", "Docker", $sprite="docker") {
            ContainerDb(postgres_replica_db, "PostgreSQL Replica", "PostgreSQL 18", "Read replica for high availability", $sprite="postgresql")
        }
        Deployment_Node(redis_cluster, "Redis Cluster", "Docker", $sprite="docker") {
            Container(redis_cache, "Redis Cache", "Redis 7", "Session management and caching", $sprite="redis")
        }
    }
    
    '--- Monitoring Layer ---
    Deployment_Node(monitoring_node, "Monitoring Node", "Ubuntu 22.04 LTS", $sprite="server") {
        Deployment_Node(prometheus_container, "Prometheus Container", "Docker", $sprite="docker") {
            Container(prometheus, "Prometheus", "Prometheus", "Metrics collection and monitoring", $sprite="chart_line")
        }
        Deployment_Node(grafana_container, "Grafana Container", "Docker", $sprite="docker") {
            Container(grafana, "Grafana", "Grafana", "Dashboards and visualization", $sprite="chart_line")
        }
    }
}

'=================================
' External Systems
'=================================
Deployment_Node(external_systems, "External Systems", "Campus Network") {
    Deployment_Node(ami_infrastructure, "AMI Infrastructure", "Secure Network") {
        Container(smart_meters, "Smart Meters", "IoT Devices", "Advanced Metering Infrastructure with UUID management", $sprite="bolt")
        Container(meter_gateway, "Meter Gateway", "Industrial Gateway", "Data collection and protocol translation", $sprite="server")
    }
    Deployment_Node(rec_services, "REC Services", "Internet") {
        Container(rec_authority, "REC Authority", "External API", "Renewable Energy Certificate validation")
    }
}

'=================================
' Relationships - User Connections
'=================================
Rel_R(web_browser, nginx, "HTTPS requests", "443/TCP")
Rel_R(admin_interface, nginx, "Admin access", "443/TCP")

'=================================
' Load Balancer Routing
'=================================
Rel_R(nginx, frontend_app, "Static content", "80/TCP")
Rel_R(nginx, api_gateway, "API requests", "8080/TCP")

'=================================
' Application Layer Interactions
'=================================
Rel_D(frontend_app, api_gateway, "REST API", "HTTP/JSON")
Rel_D(api_gateway, postgres_db, "Database queries", "5432/TCP")
Rel_D(api_gateway, redis_cache, "Cache operations", "6379/TCP")
Rel_R(api_gateway, kafka_broker, "Event streaming", "9092/TCP")
Rel_R(api_gateway, solana_validator, "Blockchain RPC", "8899/TCP")

'=================================
' Database Layer
'=================================
Rel_R(postgres_db, postgres_replica_db, "Streaming replication", "5432/TCP")

'=================================
' External Integrations
'=================================
Rel_R(api_gateway, smart_meters, "Meter data", "HTTPS/MQTT")
Rel_R(api_gateway, rec_authority, "Certificate validation", "HTTPS")
Rel_U(smart_meters, meter_gateway, "Modbus/IoT", "Various protocols")

'=================================
' Monitoring Layer
'=================================
Rel_D(api_gateway, prometheus, "Metrics export", "9090/TCP")
Rel_D(postgres_db, prometheus, "DB metrics", "9187/TCP")
Rel_D(solana_validator, prometheus, "Blockchain metrics", "8899/TCP")
Rel_R(prometheus, grafana, "Data source", "3000/TCP")

'=================================
' Styling - Infrastructure Theme
'=================================

' User devices
UpdateElementStyle(web_browser, $bgColor="#E8F8F5", $fontColor="#1B5E20", $borderColor="#4CAF50")
UpdateElementStyle(admin_interface, $bgColor="#FFE6CC", $fontColor="#BF360C", $borderColor="#FF6F00")

' Infrastructure components
UpdateElementStyle(nginx, $bgColor="#F3E5F5", $fontColor="#4A148C", $borderColor="#7B1FA2")
UpdateElementStyle(frontend_app, $bgColor="#E0F2F1", $fontColor="#00695C", $borderColor="#009688")
UpdateElementStyle(api_gateway, $bgColor="#E3F2FD", $fontColor="#0D47A1", $borderColor="#1976D2")

' Blockchain layer
UpdateElementStyle(solana_validator, $bgColor="#E8F5E8", $fontColor="#1B5E20", $borderColor="#388E3C")
UpdateElementStyle(solana_ledger, $bgColor="#F1F8E9", $fontColor="#33691E", $borderColor="#689F38")

' Data layer
UpdateElementStyle(postgres_db, $bgColor="#ECEFF1", $fontColor="#263238", $borderColor="#607D8B")
UpdateElementStyle(postgres_replica_db, $bgColor="#E8EAF6", $fontColor="#3F51B5", $borderColor="#5C6BC0")
UpdateElementStyle(redis_cache, $bgColor="#FFEBEE", $fontColor="#C62828", $borderColor="#F44336")
UpdateElementStyle(kafka_broker, $bgColor="#FFF8E1", $fontColor="#E65100", $borderColor="#FBC02D")

' Monitoring
UpdateElementStyle(prometheus, $bgColor="#FFF3E0", $fontColor="#E65100", $borderColor="#FF9800")
UpdateElementStyle(grafana, $bgColor="#E1F5FE", $fontColor="#0277BD", $borderColor="#03A9F4")

' External systems
UpdateElementStyle(smart_meters, $bgColor="#FFF3E0", $fontColor="#E65100", $borderColor="#FF9800")
UpdateElementStyle(meter_gateway, $bgColor="#F3E5F5", $fontColor="#7B1FA2", $borderColor="#9C27B0")
UpdateElementStyle(rec_authority, $bgColor="#E0F2F1", $fontColor="#00796B", $borderColor="#26A69A")

'=================================
' Layout Configuration
'=================================
LAYOUT_WITH_LEGEND()
LAYOUT_LEFT_RIGHT()

@enduml
```