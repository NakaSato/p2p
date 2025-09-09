# P2P Energy Trading Smart Contracts Architecture

## Contract Relationships Diagram

```mermaid
graph TB
    %% External Systems
    AMI["🏭 AMI Head-End API<br/>📊 Smart Meter Data"]
    Oracle["🔗 Oracle Network<br/>⚡ Chainlink/External"]
    
    %% Smart Contracts
    Registry["📋 Registry Contract<br/>👥 User & Meter Management"]
    GridToken["🪙 GridToken Contract<br/>💰 PSP22 Token (GRID)"]
    OracleClient["🌉 Oracle Client Contract<br/>🔄 Data Bridge & Automation"]
    Trading["📈 Trading Contract<br/>📊 Order Book & Matching"]
    
    %% User Types
    REC["👨‍💼 REC Regulators<br/>🛡️ Admin Users"]
    Prosumer["🏠 Prosumers<br/>⚡ Generate & Consume"]
    Consumer["🏢 Consumers<br/>🔌 Only Consume"]
    
    %% Contract Interactions - Core Flow
    Registry -.->|✅ User Verification| GridToken
    Registry -.->|✅ User Verification| Trading
    Registry -.->|🔍 Meter Ownership| OracleClient
    
    GridToken <-->|💸 Token Transfers| Trading
    OracleClient -->|🪙 Mint Tokens| GridToken
    OracleClient -->|🔔 Market Clearing| Trading
    
    %% External Interactions
    AMI ==>|📡 Energy Data| OracleClient
    Oracle ==>|🤖 Automated Operations| OracleClient
    
    %% User Interactions - Management
    REC ==>|⚙️ Manage Users/Meters| Registry
    REC ==>|🔑 Add Minters/Burners| GridToken
    REC ==>|📊 Add Market Makers| Trading
    
    %% User Interactions - Trading
    Prosumer -->|📤 Create Sell Orders| Trading
    Prosumer <-->|💰 Token Operations| GridToken
    Consumer -->|📥 Create Buy Orders| Trading
    Consumer <-->|💰 Token Operations| GridToken
    
    %% Styling - Enhanced with gradients and shadows
    classDef contract fill:#e3f2fd,stroke:#1565c0,stroke-width:3px,color:#000,font-weight:bold
    classDef external fill:#fff3e0,stroke:#ef6c00,stroke-width:3px,color:#000,font-weight:bold
    classDef user fill:#e8f5e8,stroke:#2e7d32,stroke-width:3px,color:#000,font-weight:bold
    classDef admin fill:#fce4ec,stroke:#c2185b,stroke-width:3px,color:#000,font-weight:bold
    
    class Registry,GridToken,OracleClient,Trading contract
    class AMI,Oracle external
    class Prosumer,Consumer user
    class REC admin
```

## Detailed Contract Architecture

```mermaid
graph TB
    subgraph Registry ["📋 Registry Contract"]
        direction TB
        R1["👤 User Registration<br/><small>register_user()</small>"]
        R2["📡 Meter Assignment<br/><small>assign_meter()</small>"]
        R3["👨‍💼 REC Regulator Management<br/><small>add_rec_regulator()</small>"]
        R4["✅ User Verification<br/><small>is_user_verified()</small>"]
        
        R1 --> R2
        R3 --> R1
        R3 --> R2
        R1 --> R4
    end
    
    subgraph GridToken ["🪙 GridToken Contract (PSP22)"]
        direction TB
        G1["💰 PSP22 Token Standard<br/><small>transfer(), approve()</small>"]
        G2["⚡ Mint Energy Tokens<br/><small>mint()</small>"]
        G3["🔥 Burn Energy Tokens<br/><small>burn()</small>"]
        G4["💸 Transfer Management<br/><small>transfer_from_to()</small>"]
        G5["🔑 Minter/Burner Authorization<br/><small>add_minter()</small>"]
        
        G5 --> G2
        G5 --> G3
        G2 --> G1
        G3 --> G1
        G1 --> G4
    end
    
    subgraph OracleClient ["🌉 Oracle Client Contract"]
        direction TB
        O1["📊 Energy Data Requests<br/><small>request_energy_data()</small>"]
        O2["🤖 Market Clearing Automation<br/><small>check_upkeep()</small>"]
        O3["👨‍🔧 Oracle Operator Management<br/><small>add_oracle_operator()</small>"]
        O4["🔄 Cross-Contract Calls<br/><small>fulfill_energy_data()</small>"]
        
        O3 --> O1
        O3 --> O2
        O1 --> O4
        O2 --> O4
    end
    
    subgraph Trading ["📈 Trading Contract"]
        direction TB
        T1["📚 Order Book Management<br/><small>get_order()</small>"]
        T2["📊 Buy/Sell Orders<br/><small>create_sell_order()</small>"]
        T3["⚙️ Order Matching Engine<br/><small>match_orders()</small>"]
        T4["💰 Trade Settlement<br/><small>execute_trade()</small>"]
        T5["🏪 Market Maker Management<br/><small>add_market_maker()</small>"]
        
        T5 --> T3
        T2 --> T1
        T1 --> T3
        T3 --> T4
    end
    
    %% Cross-contract relationships with enhanced styling
    R4 -.->|"✅ Verify Users"| G2
    R4 -.->|"🏠 Verify Prosumers"| T2
    R2 -.->|"📡 Meter Ownership"| O1
    
    O4 -.->|"⚡ Mint for Generation"| G2
    O2 -.->|"🔔 Trigger Clearing"| T3
    
    G4 -.->|"💸 Token Settlement"| T4
    T4 -.->|"💰 Execute Transfers"| G4
    
    %% Enhanced Styling with gradients and better colors
    classDef registryStyle fill:linear-gradient(135deg, #ffebee 0%, #ffcdd2 100%),stroke:#c62828,stroke-width:3px,color:#000,font-weight:bold
    classDef tokenStyle fill:linear-gradient(135deg, #e8f5e8 0%, #c8e6c9 100%),stroke:#2e7d32,stroke-width:3px,color:#000,font-weight:bold
    classDef oracleStyle fill:linear-gradient(135deg, #fff3e0 0%, #ffe0b2 100%),stroke:#ef6c00,stroke-width:3px,color:#000,font-weight:bold
    classDef tradingStyle fill:linear-gradient(135deg, #e3f2fd 0%, #bbdefb 100%),stroke:#1565c0,stroke-width:3px,color:#000,font-weight:bold
    
    class R1,R2,R3,R4 registryStyle
    class G1,G2,G3,G4,G5 tokenStyle
    class O1,O2,O3,O4 oracleStyle
    class T1,T2,T3,T4,T5 tradingStyle
```

## Data Flow Diagram

```mermaid
sequenceDiagram
    participant AMI as 🏭 AMI System
    participant Oracle as 🌉 Oracle Client
    participant Registry as 📋 Registry
    participant Token as 🪙 GridToken
    participant Trading as 📈 Trading
    participant User as 👤 Prosumer/Consumer
    
    %% User Registration Flow
    Note over Registry: 🚀 Phase 1: User Registration & Setup
    Registry->>+Registry: 👤 register_user(account, type, location)
    Registry->>Registry: 📡 assign_meter(meter_id, owner)
    Registry-->>-User: ✅ Registration Complete
    
    %% Energy Generation Flow
    Note over AMI,Token: ⚡ Phase 2: Energy Generation & Token Minting
    AMI->>+Oracle: 📊 Energy Data (kWh generated)
    Oracle->>+Registry: 🔍 Verify Meter Owner
    Registry-->>-Oracle: ✅ Owner Verified
    Oracle->>+Token: ⚡ mint(owner, amount, meter_id)
    Token-->>-User: 🪙 Energy Tokens Credited
    Oracle-->>-AMI: ✅ Data Processed
    
    %% Trading Flow
    Note over User,Trading: 📊 Phase 3: Energy Trading
    User->>+Trading: 📤 create_sell_order(energy, price)
    Trading->>+Registry: ✅ Verify User Status
    Registry-->>-Trading: 🏠 User is Prosumer
    Trading->>+Token: 💰 Check Token Allowance
    Token-->>-Trading: ✅ Allowance Sufficient
    Trading-->>-User: 📋 Order Created
    
    %% Market Clearing
    Note over Oracle,Trading: 🤖 Phase 4: Automated Market Clearing
    Oracle->>+Trading: 🔔 Trigger Market Clearing
    Trading->>Trading: ⚙️ match_orders()
    Trading->>+Token: 💸 Execute Token Transfers
    Token-->>-Trading: ✅ Transfers Complete
    Trading-->>-Oracle: 📊 Market Cleared
    
    %% Settlement
    Note over Token,User: 💰 Phase 5: Trade Settlement
    Token->>User: 💸 Transfer Tokens (Buyer → Seller)
    Trading->>User: 📋 Trade Confirmation
    
    %% Styling
    Note over AMI,User: 🎉 Complete P2P Energy Trading Cycle
```

## Contract Storage Overview

```mermaid
erDiagram
    Registry {
        AccountId rec_regulators "🛡️ Admin accounts"
        AccountId users "👥 Registered users"
        String meter_owners "📡 Meter → Owner mapping"
        AccountId user_meters "👤 User → Meters mapping"
        u32 max_meters_per_user "📊 Meter limit per user"
    }
    
    GridToken {
        Balance total_supply "🪙 Total GRID tokens"
        AccountId balances "💰 User token balances"
        AccountId allowances "✅ Spending permissions"
        AccountId minters "⚡ Authorized minters"
        AccountId burners "🔥 Authorized burners"
        AccountId registry_contract "📋 Registry reference"
        String name "📛 Token name (GridToken)"
        String symbol "🏷️ Token symbol (GRID)"
        u8 decimals "🔢 Token decimals (18)"
    }
    
    OracleClient {
        AccountId registry_contract "📋 Registry reference"
        AccountId token_contract "🪙 Token reference"
        AccountId trading_contract "📈 Trading reference"
        u64 next_request_id "🆔 Request counter"
        RequestId oracle_requests "📊 Pending requests"
        AccountId oracle_operators "👨‍🔧 Authorized operators"
        Balance oracle_balance "💰 Oracle funding"
        u64 last_market_check "⏰ Last clearing time"
        bool auto_market_clearing "🤖 Auto-clearing enabled"
    }
    
    Trading {
        AccountId registry_contract "📋 Registry reference"
        AccountId token_contract "🪙 Token reference"
        u64 market_epoch_length "⏱️ Trading period (15min)"
        u64 current_epoch_start "🕐 Current period start"
        u64 next_order_id "🆔 Order counter"
        OrderId sell_orders "📤 Active sell orders"
        OrderId buy_orders "📥 Active buy orders"
        AccountId user_orders "👤 User → Orders mapping"
        AccountId market_makers "🏪 Authorized market makers"
        u64 trades "📊 Trade history"
        u32 max_orders_per_user "📈 Order limit per user"
    }
    
    %% Enhanced relationships with descriptive labels
    Registry ||--o{ GridToken : "🔍 user_verification"
    Registry ||--o{ Trading : "✅ user_verification"
    Registry ||--o{ OracleClient : "📡 meter_ownership"
    GridToken ||--o{ Trading : "💸 token_transfers"
    OracleClient ||--o{ GridToken : "⚡ mint_tokens"
    OracleClient ||--o{ Trading : "🔔 market_clearing"
```

## 🎯 Key Features Summary

### 📋 Registry Contract
- **🎯 Purpose**: Identity and meter management for campus energy ecosystem
- **🔧 Key Functions**: 
  - `register_user()` - User registration with type classification
  - `assign_meter()` - Smart meter assignment to users
  - `is_user_verified()` - User verification for other contracts
- **🛡️ Access Control**: REC regulators manage all user and meter operations
- **📊 Capacity**: Up to 10 meters per user (configurable)

### 🪙 GridToken Contract (PSP22)
- **🎯 Purpose**: Energy-backed token standard (1 kWh = 1 GRID token)
- **🔧 Key Functions**: 
  - `mint()` - Create tokens for energy generation
  - `burn()` - Destroy tokens for energy consumption
  - `transfer()` - PSP22-compliant token transfers
- **⚡ Integration**: Minted by Oracle Client, traded in Trading contract
- **🔒 Security**: Role-based minting/burning with registry verification

### 🌉 Oracle Client Contract
- **🎯 Purpose**: Secure bridge between blockchain and external energy systems
- **🔧 Key Functions**: 
  - `request_energy_data()` - Fetch AMI meter readings
  - `check_upkeep()` - Automated market clearing triggers
  - `fulfill_energy_data()` - Process oracle responses
- **🤖 Automation**: Chainlink Keepers compatible for 24/7 operations
- **💰 Economics**: Self-funded oracle operations with balance management

### 📈 Trading Contract
- **🎯 Purpose**: Decentralized P2P energy marketplace
- **🔧 Key Functions**: 
  - `create_sell_order()` - Prosumers sell excess energy
  - `create_buy_order()` - Consumers purchase energy
  - `match_orders()` - Automated order matching engine
- **⏰ Market Structure**: Periodic clearing every 15 minutes (configurable)
- **📊 Scalability**: Up to 100 orders per user per epoch

## 🔗 Cross-Contract Dependencies

```mermaid
graph LR
    A["📋 Registry"] -->|"✅ User Verification"| B["🪙 GridToken"]
    A -->|"🏠 Prosumer Verification"| C["📈 Trading"]
    A -->|"📡 Meter Ownership"| D["🌉 Oracle Client"]
    
    D -->|"⚡ Automated Minting"| B
    D -->|"🔔 Market Clearing"| C
    B <-->|"💸 Token Settlement"| C
    
    style A fill:#ffebee,stroke:#c62828,stroke-width:3px
    style B fill:#e8f5e8,stroke:#2e7d32,stroke-width:3px
    style C fill:#e3f2fd,stroke:#1565c0,stroke-width:3px
    style D fill:#fff3e0,stroke:#ef6c00,stroke-width:3px
```

### 🔄 Integration Flow
1. **📋 Registry → 🪙 GridToken**: User verification prevents unauthorized minting
2. **📋 Registry → 📈 Trading**: Prosumer verification enables sell orders
3. **📋 Registry → 🌉 Oracle Client**: Meter ownership validation for data requests
4. **🌉 Oracle Client → 🪙 GridToken**: Automated token minting for verified energy generation
5. **🌉 Oracle Client → 📈 Trading**: Scheduled market clearing every 15 minutes
6. **📈 Trading ↔ 🪙 GridToken**: Bidirectional token transfers for trade settlement