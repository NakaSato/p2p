# P2P Energy Trading System - C4 System Context Diagram

This diagram presents a modern architectural overview of the P2P Energy Trading Platform ecosystem, showcasing the interactions between campus stakeholders, core systems, and external infrastructure with contemporary visual design using C4-PlantUML.

```plantuml
@startuml
!include https://raw.githubusercontent.com/plantuml-stdlib/C4-PlantUML/master/C4_Context.puml

!define DEVICONS https://raw.githubusercontent.com/tupadr3/plantuml-icon-font-sprites/master/devicons
!define FONTAWESOME https://raw.githubusercontent.com/tupadr3/plantuml-icon-font-sprites/master/font-awesome-5
!include FONTAWESOME/users.puml
!include FONTAWESOME/user_tie.puml
!include FONTAWESOME/user_cog.puml
!include FONTAWESOME/bolt.puml
!include FONTAWESOME/solar_panel.puml
!include FONTAWESOME/network_wired.puml
!include FONTAWESOME/certificate.puml
!include FONTAWESOME/cloud.puml

title P2P Energy Trading Platform - System Context Architecture (Actual Implementation)

'=================================
' Campus Ecosystem - Engineering Complex
'=================================
Enterprise_Boundary(engineering_campus, "Engineering Complex Campus") {
    Person(engineering_students, "Engineering Students", "Prosumers and consumers in Engineering Complex with smart meter assignments", $sprite="users")
    Person(engineering_faculty, "Engineering Faculty", "Faculty members participating in energy trading with assigned meters", $sprite="user_tie")
    Person(engineering_admin, "Engineering Department", "Single authority managing PoA consensus and system administration", $sprite="user_cog")
    
    System(p2p_platform, "P2P Energy Trading Platform", "Solana-based energy trading system with 5 Anchor programs and REST API Gateway", $sprite="bolt")
    
    System_Boundary(engineering_infrastructure, "Engineering Complex Infrastructure") {
        System_Ext(ami_meters, "Smart Meters (METER_001-015)", "15 smart meters providing AMI data for energy generation and consumption tracking", $sprite="network_wired")
        System_Ext(solar_systems, "Solar Generation Systems", "Distributed solar panels and battery storage in Engineering Complex", $sprite="solar_panel")
        System_Ext(engineering_grid, "Engineering Grid Infrastructure", "Campus electrical distribution with grid connection capabilities", $sprite="bolt")
    }
}

'=================================
' External Systems (Actual Implementation)
'=================================
System_Ext(solana_network, "Solana Blockchain", "Devnet/Localnet deployment with 5 Anchor programs: Registry, Energy Token (SPL), Trading, Oracle, Governance")
System_Ext(rec_authority, "REC Authority", "External API for renewable energy certificate validation and compliance", $sprite="certificate")
System_Ext(utility_grid, "Regional Utility Grid", "External grid connection for backup supply and excess energy export", $sprite="bolt")
System_Ext(weather_data, "Weather API", "External weather data service for solar generation forecasting", $sprite="cloud")

'=================================
' Relationships (Actual Implementation)
'=================================

' Core user interactions with actual system
BiRel(engineering_students, p2p_platform, "Create buy/sell orders & view analytics", "React Web App + Rust API Gateway")
BiRel(engineering_faculty, p2p_platform, "Energy trading & meter management", "Web Interface + Blockchain Integration")
Rel_Down(engineering_admin, p2p_platform, "System administration & PoA validation", "Admin API + Governance Program")

' Blockchain interactions (5 Anchor programs)
BiRel(p2p_platform, solana_network, "Program interactions: Registry, Trading, Oracle, Token, Governance", "Solana RPC/WebSocket")
Rel_Up(p2p_platform, rec_authority, "Certificate validation API", "HTTPS/REST API")
BiRel(p2p_platform, weather_data, "Solar generation forecasting", "HTTP API")

' Engineering infrastructure data flows
BiRel(p2p_platform, ami_meters, "AMI data ingestion & meter readings", "HTTPS/MQTT API")
Rel_Left(ami_meters, solar_systems, "Solar generation monitoring", "Modbus/IoT Protocols")
Rel_Right(ami_meters, engineering_grid, "Energy consumption tracking", "Smart Meter Protocols")
BiRel(engineering_grid, utility_grid, "Grid connection & energy export", "Electrical Grid Infrastructure")

'=================================
' Styling - Modern color palette with enhanced visual hierarchy
'=================================

' Campus users with distinct professional styling
UpdateElementStyle(engineering_students, $bgColor="#E8F8F5", $fontColor="#0D4F3C", $borderColor="#1ABC9C")
UpdateElementStyle(engineering_faculty, $bgColor="#EBF3FD", $fontColor="#1B4F72", $borderColor="#3498DB")
UpdateElementStyle(engineering_admin, $bgColor="#FDF2E9", $fontColor="#B9770E", $borderColor="#F39C12")

' Core system with prominent branding
UpdateElementStyle(p2p_platform, $bgColor="#E8F6F3", $fontColor="#138D75", $borderColor="#16A085")

' Engineering infrastructure with cohesive color scheme
UpdateElementStyle(ami_meters, $bgColor="#F4ECF7", $fontColor="#6C3483", $borderColor="#A569BD")
UpdateElementStyle(solar_systems, $bgColor="#EAFAF1", $fontColor="#1E8449", $borderColor="#58D68D")
UpdateElementStyle(engineering_grid, $bgColor="#FEF9E7", $fontColor="#B7950B", $borderColor="#F1C40F")

' External systems with distinctive styling
UpdateElementStyle(solana_network, $bgColor="#E5F3FF", $fontColor="#1565C0", $borderColor="#2196F3")
UpdateElementStyle(rec_authority, $bgColor="#F3E5F5", $fontColor="#8E24AA", $borderColor="#AB47BC")
UpdateElementStyle(utility_grid, $bgColor="#FFEBEE", $fontColor="#D32F2F", $borderColor="#F44336")
UpdateElementStyle(weather_data, $bgColor="#E3F2FD", $fontColor="#1976D2", $borderColor="#03A9F4")

'=================================
' Layout Configuration
'=================================
LAYOUT_WITH_LEGEND()
LAYOUT_LEFT_RIGHT()

@enduml
```