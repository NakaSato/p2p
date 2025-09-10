# Migration Plan: From ink! to Solana Anchor Permissioned Environment

## Executive Summary

This document outlines the strategic migration plan from ink! smart contracts on Substrate framework to Solana's Anchor framework within a permissioned blockchain environment. This migration will transform the P2P Energy Trading System to leverage Solana's high-performance blockchain infrastructure while maintaining centralized control suitable for university campus deployment.

## Current State Analysis

### **Current Technology Stack**
- **Blockchain**: Substrate Framework
- **Smart Contracts**: ink! (Rust-based)
- **Token Standard**: PSP22 (Polkadot Standard)
- **Consensus**: GRANDPA + BABE
- **Network Type**: Private/Consortium blockchain

### **Current Smart Contracts**
1. **Registry Contract** (`contracts/registry/lib.rs`)
2. **GridToken Contract** (`contracts/grid-token/lib.rs`) 
3. **Trading Contract** (`contracts/trading/lib.rs`)
4. **Oracle Client Contract** (`contracts/oracle-client/lib.rs`)

## Target State: Solana Anchor Permissioned Environment

### **New Technology Stack**
- **Blockchain**: Solana (Permissioned Cluster)
- **Smart Contracts**: Anchor Framework (Rust-based)
- **Token Standard**: SPL Token (Solana Program Library)
- **Consensus**: Proof of Authority (PoA) with designated REC validators
- **Network Type**: Private Solana Cluster with university-controlled REC validators

### **Solana PoA Environment Benefits**
1. **High Performance**: 5,000+ TPS with PoA consensus
2. **Ultra-Low Latency**: <400ms block finality with known validators
3. **Zero Staking Costs**: No economic barriers for validators
4. **Rust Ecosystem**: Leverages existing Rust knowledge
5. **University Control**: Complete authority over REC validator network
6. **Instant Governance**: Immediate REC validator addition/removal
7. **Predictable Performance**: Known validator set ensures consistent throughput
8. **Academic Suitability**: Perfect for controlled university environments
9. **Energy Efficient**: No energy-intensive mining or staking requirements
10. **REC Compliance**: Built-in Renewable Energy Certificate validation and tracking
11. **Regulatory Compliance**: Meets institutional governance and environmental standards

## Proof of Authority (PoA) Consensus Advantages

### **Why PoA for University Environment**

**Proof of Authority** is the optimal consensus mechanism for the university's P2P energy trading system for the following reasons:

#### **1. Complete Institutional Control**
- University IT department controls all validators
- Immediate response to network issues or policy changes
- No external dependencies on public validators or staking

#### **2. Predictable Performance**
- Known validator set ensures consistent block times
- No network congestion from external users
- Guaranteed transaction throughput for campus operations

#### **3. Cost Efficiency**
- No staking requirements or economic incentives needed
- Zero validator rewards or penalties
- Minimal infrastructure costs (3-5 university servers)

#### **4. Academic Compliance**
- Meets institutional governance and audit requirements
- Full transaction traceability and control
- Compliance with university data policies

#### **5. Operational Simplicity**
- Straightforward validator management
- No complex tokenomics or economic models
- Easy maintenance and upgrades

#### **6. Network Security & REC Validation**
- University acts as the sole REC certification authority
- University infrastructure contains smart meters (AMI) for energy monitoring and REC validation nodes
- University validates and issues Renewable Energy Certificates for all energy transactions
- Complete university control over REC certification process

### **PoA Network Architecture**
  ```mermaid
  graph TB
    subgraph Campus ["University Campus Network"]
      subgraph Infrastructure ["Campus AMI Infrastructure"]
        DA["Dorm A<br/>AMI Smart Meters<br/>Solar Panels"]
        DB["Dorm B<br/>AMI Smart Meters<br/>Wind Turbines"]
        FH["Faculty Housing<br/>AMI Smart Meters<br/>Battery Storage"]
        Lab["Research Labs<br/>AMI Smart Meters<br/>High Consumption"]
      end
      
      subgraph University ["University IT Infrastructure & REC Authority"]
        UI["University IT<br/>Campus Network<br/>AMI Data Collection<br/>Blockchain Node"]
        RV1["University REC Validator 1<br/>Sustainability Office<br/>PoA Consensus<br/>REC Certification Authority"]
        RV2["University REC Validator 2<br/>Engineering Department<br/>PoA Consensus<br/>REC Certification Authority"]  
        RV3["University REC Validator 3<br/>Facilities Management<br/>PoA Consensus<br/>REC Certification Authority"]
      end
    end
    
    %% REC validator consensus connections (within university)
    RV1 -.->|"University REC Consensus"| RV2
    RV2 -.->|"University REC Consensus"| RV3
    RV3 -.->|"University REC Consensus"| RV1
    
    %% AMI data flows to university IT
    DA ==>|"AMI Data Feed"| UI
    DB ==>|"AMI Data Feed"| UI
    FH ==>|"AMI Data Feed"| UI
    Lab ==>|"AMI Data Feed"| UI
    
    %% University IT coordinates with REC validators
    UI ==>|"REC Validation Request"| RV1
    UI ==>|"REC Validation Request"| RV2
    UI ==>|"REC Validation Request"| RV3
    
    %% REC certificates back to blockchain
    RV1 ==>|"REC Certificate"| UI
    RV2 ==>|"REC Certificate"| UI
    RV3 ==>|"REC Certificate"| UI
    
    %% Enhanced styling
    classDef validator fill:#1e88e5,stroke:#0d47a1,stroke-width:3px,color:#ffffff,font-weight:bold
    classDef infrastructure fill:#43a047,stroke:#1b5e20,stroke-width:3px,color:#ffffff,font-weight:bold
    classDef university fill:#2196F3,stroke:#1565C0,stroke-width:3px,color:#ffffff,font-weight:bold
    classDef campus fill:#f5f5f5,stroke:#757575,stroke-width:2px,stroke-dasharray: 5 5
    classDef validatorGroup fill:#e3f2fd,stroke:#1976d2,stroke-width:2px,stroke-dasharray: 3 3
    classDef infraGroup fill:#e8f5e8,stroke:#388e3c,stroke-width:2px,stroke-dasharray: 3 3
    
    class RV1,RV2,RV3 validator
    class DA,DB,FH,Lab infrastructure
    class UI university
    class Campus campus
    class University validatorGroup
    class Infrastructure infraGroup
  ```

## Migration Strategy

### **Phase 1: Environment Setup and Planning (2 weeks)**

#### **1.1 Solana Permissioned Cluster Setup**
```bash
# Install Solana CLI and tools
curl -sSf https://release.solana.com/v1.17.0/install | sh
source ~/.profile

# Install Anchor Framework
npm install -g @coral-xyz/anchor-cli
cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
avm install latest
avm use latest

# Setup permissioned cluster configuration
solana config set --url http://localhost:8899
solana-test-validator --reset
```

#### **1.2 Project Structure Redesign**
```
programs/                    # Anchor programs (smart contracts)
├── registry/               # User and meter registration
├── energy-token/          # SPL Token for energy trading  
├── trading/               # Energy trading marketplace
└── oracle/                # Oracle data integration

app/                        # Frontend application
├── src/
└── anchor/                # Generated anchor client

tests/                      # Integration tests
migrations/                 # Deployment scripts
```

#### **1.3 Permissioned Network Configuration**
```toml
# Anchor.toml
[provider]
cluster = "localnet"
wallet = "~/.config/solana/id.json"

[programs.localnet]
registry = "TokenProgramAddress1234567890123456789"
energy_token = "TokenProgramAddress1234567890123456790"
trading = "TokenProgramAddress1234567890123456791"
oracle = "TokenProgramAddress1234567890123456792"

[scripts]
test = "yarn run ts-mocha -p ./tsconfig.json -t 1000000 tests/**/*.ts"
```

### **Phase 2: Smart Contract Migration (6 weeks)**

#### **2.1 Registry Program Migration**
```rust
// programs/registry/src/lib.rs
use anchor_lang::prelude::*;

declare_id!("RegistryProgramId1234567890123456789");

#[program]
pub mod registry {
    use super::*;
    
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let registry = &mut ctx.accounts.registry;
        registry.authority = ctx.accounts.authority.key();
        registry.user_count = 0;
        registry.meter_count = 0;
        Ok(())
    }
    
    pub fn register_user(
        ctx: Context<RegisterUser>,
        user_type: UserType,
        location: String,
    ) -> Result<()> {
        let user_account = &mut ctx.accounts.user_account;
        let registry = &mut ctx.accounts.registry;
        
        user_account.authority = ctx.accounts.user_authority.key();
        user_account.user_type = user_type;
        user_account.location = location;
        user_account.status = UserStatus::Active;
        user_account.registered_at = Clock::get()?.unix_timestamp;
        
        registry.user_count += 1;
        
        emit!(UserRegistered {
            user: ctx.accounts.user_authority.key(),
            user_type,
            timestamp: Clock::get()?.unix_timestamp,
        });
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + Registry::LEN,
        seeds = [b"registry"],
        bump
    )]
    pub registry: Account<'info, Registry>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Registry {
    pub authority: Pubkey,
    pub user_count: u64,
    pub meter_count: u64,
}

impl Registry {
    pub const LEN: usize = 32 + 8 + 8;
}
```

#### **2.2 Energy Token Program with REC Validation**
```rust
// programs/energy-token/src/lib.rs
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

declare_id!("EnergyTokenProgramId1234567890123456789");

#[program]
pub mod energy_token {
    use super::*;
    
    pub fn initialize_token(ctx: Context<InitializeToken>) -> Result<()> {
        let token_info = &mut ctx.accounts.token_info;
        token_info.authority = ctx.accounts.authority.key();
        token_info.mint = ctx.accounts.mint.key();
        token_info.total_supply = 0;
        token_info.rec_validators = vec![];
        Ok(())
    }
    
    pub fn add_rec_validator(
        ctx: Context<AddRecValidator>,
        validator_pubkey: Pubkey,
    ) -> Result<()> {
        let token_info = &mut ctx.accounts.token_info;
        
        require!(
            ctx.accounts.authority.key() == token_info.authority,
            ErrorCode::UnauthorizedAuthority
        );
        
        token_info.rec_validators.push(validator_pubkey);
        
        emit!(RecValidatorAdded {
            validator: validator_pubkey,
            authority: ctx.accounts.authority.key(),
            timestamp: Clock::get()?.unix_timestamp,
        });
        
        Ok(())
    }
    
    pub fn mint_energy_tokens_with_rec(
        ctx: Context<MintEnergyTokensWithRec>,
        amount: u64,
        meter_id: String,
        rec_certificate_id: String,
        renewable_source: RenewableSource,
    ) -> Result<()> {
        // Verify meter ownership and energy generation
        let registry_account = &ctx.accounts.registry_account;
        require!(
            registry_account.is_valid_meter(&meter_id),
            ErrorCode::InvalidMeter
        );
        
        // Verify REC validator signature
        let token_info = &ctx.accounts.token_info;
        require!(
            token_info.rec_validators.contains(&ctx.accounts.rec_validator.key()),
            ErrorCode::UnauthorizedRecValidator
        );
        
        // Create REC certificate on-chain
        let rec_certificate = &mut ctx.accounts.rec_certificate;
        rec_certificate.certificate_id = rec_certificate_id.clone();
        rec_certificate.meter_id = meter_id.clone();
        rec_certificate.energy_amount = amount;
        rec_certificate.renewable_source = renewable_source;
        rec_certificate.validator = ctx.accounts.rec_validator.key();
        rec_certificate.issued_at = Clock::get()?.unix_timestamp;
        rec_certificate.status = RecStatus::Active;
        
        // Mint tokens representing certified renewable energy
        let seeds = &[b"token_authority", &[ctx.bumps.token_authority]];
        let signer = &[&seeds[..]];
        
        let cpi_accounts = token::MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.user_token_account.to_account_info(),
            authority: ctx.accounts.token_authority.to_account_info(),
        };
        
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        
        token::mint_to(cpi_ctx, amount)?;
        
        let token_info = &mut ctx.accounts.token_info;
        token_info.total_supply += amount;
        
        emit!(EnergyTokensWithRecMinted {
            user: ctx.accounts.user_authority.key(),
            amount,
            meter_id,
            rec_certificate_id,
            renewable_source,
            validator: ctx.accounts.rec_validator.key(),
            timestamp: Clock::get()?.unix_timestamp,
        });
        
        Ok(())
    }
    
    pub fn retire_rec_certificate(
        ctx: Context<RetireRecCertificate>,
        certificate_id: String,
    ) -> Result<()> {
        let rec_certificate = &mut ctx.accounts.rec_certificate;
        
        require!(
            rec_certificate.certificate_id == certificate_id,
            ErrorCode::InvalidCertificateId
        );
        
        require!(
            rec_certificate.status == RecStatus::Active,
            ErrorCode::CertificateAlreadyRetired
        );
        
        rec_certificate.status = RecStatus::Retired;
        rec_certificate.retired_at = Some(Clock::get()?.unix_timestamp);
        rec_certificate.retired_by = ctx.accounts.authority.key();
        
        emit!(RecCertificateRetired {
            certificate_id,
            retired_by: ctx.accounts.authority.key(),
            timestamp: Clock::get()?.unix_timestamp,
        });
        
        Ok(())
    }
}

#[account]
pub struct TokenInfo {
    pub authority: Pubkey,
    pub mint: Pubkey,
    pub total_supply: u64,
    pub rec_validators: Vec<Pubkey>,
}

#[account]
pub struct RecCertificate {
    pub certificate_id: String,
    pub meter_id: String,
    pub energy_amount: u64,
    pub renewable_source: RenewableSource,
    pub validator: Pubkey,
    pub issued_at: i64,
    pub status: RecStatus,
    pub retired_at: Option<i64>,
    pub retired_by: Pubkey,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum RenewableSource {
    Solar,
    Wind,
    Hydro,
    Geothermal,
    Biomass,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum RecStatus {
    Active,
    Retired,
    Cancelled,
}

#[derive(Accounts)]
pub struct MintEnergyTokensWithRec<'info> {
    #[account(mut)]
    pub token_info: Account<'info, TokenInfo>,
    
    #[account(
        init,
        payer = user_authority,
        space = 8 + RecCertificate::LEN,
        seeds = [b"rec_certificate", rec_certificate_id.as_bytes()],
        bump
    )]
    pub rec_certificate: Account<'info, RecCertificate>,
    
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    
    /// CHECK: Token mint authority PDA
    #[account(
        seeds = [b"token_authority"],
        bump
    )]
    pub token_authority: AccountInfo<'info>,
    
    #[account(mut)]
    pub user_authority: Signer<'info>,
    
    #[account(signer)]
    pub rec_validator: Signer<'info>,
    
    pub registry_account: Account<'info, Registry>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl RecCertificate {
    pub const LEN: usize = 64 + 64 + 8 + 1 + 32 + 8 + 1 + 9 + 32;
}
```

#### **2.3 Trading Program Migration**
```rust
// programs/trading/src/lib.rs
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("TradingProgramId1234567890123456789");

#[program]
pub mod trading {
    use super::*;
    
    pub fn create_sell_order(
        ctx: Context<CreateSellOrder>,
        amount: u64,
        price_per_kwh: u64,
    ) -> Result<()> {
        let order = &mut ctx.accounts.order;
        let market = &mut ctx.accounts.market;
        
        order.seller = ctx.accounts.seller.key();
        order.amount = amount;
        order.price_per_kwh = price_per_kwh;
        order.order_type = OrderType::Sell;
        order.status = OrderStatus::Active;
        order.created_at = Clock::get()?.unix_timestamp;
        
        // Escrow seller's tokens
        let cpi_accounts = Transfer {
            from: ctx.accounts.seller_token_account.to_account_info(),
            to: ctx.accounts.escrow_token_account.to_account_info(),
            authority: ctx.accounts.seller.to_account_info(),
        };
        
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        
        token::transfer(cpi_ctx, amount)?;
        
        market.active_orders += 1;
        
        emit!(SellOrderCreated {
            seller: ctx.accounts.seller.key(),
            order_id: order.key(),
            amount,
            price_per_kwh,
            timestamp: Clock::get()?.unix_timestamp,
        });
        
        Ok(())
    }
    
    pub fn match_orders(ctx: Context<MatchOrders>) -> Result<()> {
        let sell_order = &mut ctx.accounts.sell_order;
        let buy_order = &mut ctx.accounts.buy_order;
        
        require!(
            sell_order.status == OrderStatus::Active,
            ErrorCode::InactiveSellOrder
        );
        require!(
            buy_order.status == OrderStatus::Active,
            ErrorCode::InactiveBuyOrder
        );
        require!(
            buy_order.price_per_kwh >= sell_order.price_per_kwh,
            ErrorCode::PriceMismatch
        );
        
        let trade_amount = std::cmp::min(sell_order.amount, buy_order.amount);
        let trade_price = sell_order.price_per_kwh;
        
        // Execute token transfer from escrow to buyer
        // Execute payment from buyer to seller
        // Update order status
        
        sell_order.amount -= trade_amount;
        buy_order.amount -= trade_amount;
        
        if sell_order.amount == 0 {
            sell_order.status = OrderStatus::Completed;
        }
        if buy_order.amount == 0 {
            buy_order.status = OrderStatus::Completed;
        }
        
        emit!(OrderMatched {
            sell_order: sell_order.key(),
            buy_order: buy_order.key(),
            amount: trade_amount,
            price: trade_price,
            timestamp: Clock::get()?.unix_timestamp,
        });
        
        Ok(())
    }
}
```

### **Phase 3: Permissioned Network Configuration (2 weeks)**

#### **3.1 Proof of Authority (PoA) Network Setup**
```bash
# Create PoA genesis configuration for university-controlled network
solana-genesis \
    --cluster-type development \
    --ledger ./ledger \
    --hashes-per-tick sleep \
    --lamports-per-byte-year 1 \
    --rent-exemption-threshold 2.0 \
    --target-lamports-per-signature 10000 \
    --bootstrap-validator university-validator-1.json vote-1.json stake-1.json \
    --bootstrap-validator-lamports 500000000000000000 \
    --bootstrap-validator-stake-lamports 1000000000000000 \
    --bootstrap-validator university-validator-2.json vote-2.json stake-2.json \
    --bootstrap-validator-lamports 500000000000000000 \
    --bootstrap-validator-stake-lamports 1000000000000000 \
    --bootstrap-validator university-validator-3.json vote-3.json stake-3.json \
    --bootstrap-validator-lamports 500000000000000000 \
    --bootstrap-validator-stake-lamports 1000000000000000 \
    --slots-per-epoch 432000 \
    --warmup-epochs 0 \
    --enable-warmup-epochs \
    --poa-mode
```

#### **3.1.1 REC Validator Configuration**
```bash
# University departments control REC validators as certification authority
# REC Validator 1: Sustainability Office
solana-keygen new --outfile sustainability-validator.json --no-bip39-passphrase
solana-keygen new --outfile sustainability-vote.json --no-bip39-passphrase
solana-keygen new --outfile sustainability-stake.json --no-bip39-passphrase

# REC Validator 2: Engineering Department
solana-keygen new --outfile engineering-validator.json --no-bip39-passphrase
solana-keygen new --outfile engineering-vote.json --no-bip39-passphrase
solana-keygen new --outfile engineering-stake.json --no-bip39-passphrase

# REC Validator 3: Facilities Management
solana-keygen new --outfile facilities-validator.json --no-bip39-passphrase
solana-keygen new --outfile facilities-vote.json --no-bip39-passphrase
solana-keygen new --outfile facilities-stake.json --no-bip39-passphrase

# Start REC validators with PoA configuration and university authority
solana-validator \
    --identity sustainability-validator.json \
    --vote-account sustainability-vote.json \
    --ledger ./ledger \
    --rpc-port 8899 \
    --entrypoint 127.0.0.1:8001 \
    --expected-genesis-hash $(solana-keygen pubkey genesis.json) \
    --authorized-voters sustainability-vote.json,engineering-vote.json,facilities-vote.json \
    --no-voting \
    --enable-rpc-transaction-history \
    --log ./sustainability-validator.log \
    --enable-rec-validation \
    --rec-authority-keypair sustainability-validator.json

# University REC Authority roles and responsibilities:
# - University departments act as REC certification authority
# - Validate renewable energy certificates for all campus energy generation
# - Verify AMI smart meter readings against renewable source documentation
# - Issue on-chain REC certificates for verified renewable energy
# - Maintain university compliance and audit trail
# - Coordinate with university sustainability and engineering policies
```

#### **3.2 Access Control Implementation**
```rust
// Access control macros for permissioned operations
#[derive(Accounts)]
pub struct PermissionedOperation<'info> {
    #[account(
        constraint = authority.key() == UNIVERSITY_AUTHORITY_PUBKEY,
        signer
    )]
    pub authority: Signer<'info>,
}

// University authority public key (compile-time constant)
pub const UNIVERSITY_AUTHORITY_PUBKEY: Pubkey = solana_program::pubkey!("UniversityAuthorityKey1234567890123456789");
```

#### **3.3 PoA Governance Program with REC Authority**
```rust
#[program]
pub mod poa_governance {
    use super::*;
    
    pub fn initialize_poa_with_rec(ctx: Context<InitializePoAWithRec>) -> Result<()> {
        let poa_config = &mut ctx.accounts.poa_config;
        poa_config.university_authority = ctx.accounts.university_authority.key();
        poa_config.authorized_rec_validators = vec![
            // University departments as REC certification authority
            RecValidatorInfo {
                pubkey: ctx.accounts.sustainability_validator.key(),
                authority_name: "University Sustainability Office".to_string(),
                certification_authority: true,
                active: true,
            },
            RecValidatorInfo {
                pubkey: ctx.accounts.engineering_validator.key(),
                authority_name: "University Engineering Department".to_string(),
                certification_authority: true,
                active: true,
            },
            RecValidatorInfo {
                pubkey: ctx.accounts.facilities_validator.key(),
                authority_name: "University Facilities Management".to_string(),
                certification_authority: true,
                active: true,
            },
        ];
        poa_config.min_rec_validators = 2; // Minimum for REC consensus
        poa_config.created_at = Clock::get()?.unix_timestamp;
        Ok(())
    }
    
    pub fn add_authorized_validator(
        ctx: Context<AddValidator>,
        validator_pubkey: Pubkey,
    ) -> Result<()> {
        let poa_config = &mut ctx.accounts.poa_config;
        
        // Only university authority can add validators
        require!(
            ctx.accounts.university_authority.key() == poa_config.university_authority,
            ErrorCode::UnauthorizedAuthority
        );
        
        require!(
            !poa_config.authorized_validators.contains(&validator_pubkey),
            ErrorCode::ValidatorAlreadyAuthorized
        );
        
        poa_config.authorized_validators.push(validator_pubkey);
        
        emit!(ValidatorAdded {
            validator: validator_pubkey,
            authority: ctx.accounts.university_authority.key(),
            timestamp: Clock::get()?.unix_timestamp,
        });
        
        Ok(())
    }
    
    pub fn remove_authorized_validator(
        ctx: Context<RemoveValidator>,
        validator_pubkey: Pubkey,
    ) -> Result<()> {
        let poa_config = &mut ctx.accounts.poa_config;
        
        require!(
            ctx.accounts.university_authority.key() == poa_config.university_authority,
            ErrorCode::UnauthorizedAuthority
        );
        
        require!(
            poa_config.authorized_validators.len() > poa_config.min_validators as usize,
            ErrorCode::InsufficientValidators
        );
        
        poa_config.authorized_validators.retain(|&x| x != validator_pubkey);
        
        emit!(ValidatorRemoved {
            validator: validator_pubkey,
            authority: ctx.accounts.university_authority.key(),
            timestamp: Clock::get()?.unix_timestamp,
        });
        
        Ok(())
    }
    
    pub fn update_min_validators(
        ctx: Context<UpdateMinValidators>,
        new_min: u8,
    ) -> Result<()> {
        let poa_config = &mut ctx.accounts.poa_config;
        
        require!(
            ctx.accounts.university_authority.key() == poa_config.university_authority,
            ErrorCode::UnauthorizedAuthority
        );
        
        require!(
            new_min > 0 && new_min <= poa_config.authorized_validators.len() as u8,
            ErrorCode::InvalidMinValidators
        );
        
        poa_config.min_validators = new_min;
        
        Ok(())
    }
}

#[account]
pub struct PoAConfig {
    pub university_authority: Pubkey,
    pub authorized_rec_validators: Vec<RecValidatorInfo>,
    pub min_rec_validators: u8,
    pub created_at: i64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct RecValidatorInfo {
    pub pubkey: Pubkey,
    pub authority_name: String, // e.g., "University Sustainability Office"
    pub certification_authority: bool,
    pub active: bool,
}

impl PoAConfig {
    pub const MAX_REC_VALIDATORS: usize = 10;
    pub const LEN: usize = 32 + 4 + (RecValidatorInfo::LEN * Self::MAX_REC_VALIDATORS) + 1 + 8;
}

impl RecValidatorInfo {
    pub const LEN: usize = 32 + 64 + 1 + 1; // pubkey + authority_name + certification_authority + active
}

#[derive(Accounts)]
pub struct InitializePoA<'info> {
    #[account(
        init,
        payer = university_authority,
        space = 8 + PoAConfig::LEN,
        seeds = [b"poa_config"],
        bump
    )]
    pub poa_config: Account<'info, PoAConfig>,
    
    #[account(mut)]
    pub university_authority: Signer<'info>,
    
    /// CHECK: University Sustainability Office validator
    pub sustainability_validator: AccountInfo<'info>,
    /// CHECK: University Engineering Department validator  
    pub engineering_validator: AccountInfo<'info>,
    /// CHECK: University Facilities Management validator
    pub facilities_validator: AccountInfo<'info>,
    
    pub system_program: Program<'info, System>,
}
```

### **Phase 4: API Gateway Migration (3 weeks)**

#### **4.1 Solana RPC Integration**
```rust
// src/solana/client.rs
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use anchor_client::{Client, Cluster, Program};

pub struct SolanaClient {
    rpc_client: RpcClient,
    anchor_client: Client,
    payer: Keypair,
}

impl SolanaClient {
    pub fn new(cluster_url: &str, payer: Keypair) -> Self {
        let rpc_client = RpcClient::new_with_commitment(
            cluster_url.to_string(),
            CommitmentConfig::confirmed(),
        );
        
        let anchor_client = Client::new_with_options(
            Cluster::Custom(cluster_url.to_string(), cluster_url.to_string()),
            payer.clone(),
            CommitmentConfig::confirmed(),
        );
        
        Self {
            rpc_client,
            anchor_client,
            payer,
        }
    }
    
    pub async fn register_user(
        &self,
        user_type: UserType,
        location: String,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let program = self.anchor_client.program(REGISTRY_PROGRAM_ID);
        
        let tx = program
            .request()
            .accounts(registry_accounts::RegisterUser {
                // Account setup
            })
            .args(registry_instruction::RegisterUser {
                user_type,
                location,
            })
            .send()
            .await?;
            
        Ok(tx.to_string())
    }
}
```

#### **4.2 Updated API Endpoints**
```rust
// src/handlers/energy.rs
use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct MeterReadingRequest {
    pub meter_id: String,
    pub energy_generated: u64,
    pub energy_consumed: u64,
    pub timestamp: i64,
    pub signature: String,
}

#[derive(Serialize)]
pub struct MeterReadingResponse {
    pub transaction_id: String,
    pub tokens_minted: u64,
    pub status: String,
}

pub async fn submit_meter_reading(
    State(solana_client): State<SolanaClient>,
    Json(request): Json<MeterReadingRequest>,
) -> Result<Json<MeterReadingResponse>, AppError> {
    // Validate meter reading signature
    validate_meter_signature(&request)?;
    
    // Calculate net energy
    let net_energy = request.energy_generated.saturating_sub(request.energy_consumed);
    
    if net_energy > 0 {
        // Mint energy tokens via Solana program
        let tx_id = solana_client
            .mint_energy_tokens(net_energy, request.meter_id.clone())
            .await?;
            
        Ok(Json(MeterReadingResponse {
            transaction_id: tx_id,
            tokens_minted: net_energy,
            status: "success".to_string(),
        }))
    } else {
        Ok(Json(MeterReadingResponse {
            transaction_id: "".to_string(),
            tokens_minted: 0,
            status: "no_surplus".to_string(),
        }))
    }
}
```

### **Phase 5: Frontend Migration (3 weeks)**

#### **5.1 Solana Wallet Integration**
```typescript
// src/contexts/SolanaWalletContext.tsx
import { createContext, useContext } from 'react';
import { Connection, PublicKey } from '@solana/web3.js';
import { Program, AnchorProvider, web3 } from '@coral-xyz/anchor';
import { useWallet } from '@solana/wallet-adapter-react';

interface SolanaWalletContextType {
  connection: Connection;
  program: Program;
  userAccount: PublicKey | null;
}

const SolanaWalletContext = createContext<SolanaWalletContextType | null>(null);

export const SolanaWalletProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const wallet = useWallet();
  const connection = new Connection('http://localhost:8899', 'confirmed');
  
  const provider = new AnchorProvider(connection, wallet, {
    commitment: 'confirmed',
  });
  
  const program = new Program(idl, PROGRAM_ID, provider);
  
  return (
    <SolanaWalletContext.Provider value={{ connection, program, userAccount: wallet.publicKey }}>
      {children}
    </SolanaWalletContext.Provider>
  );
};
```

#### **5.2 Trading Interface Updates**
```typescript
// src/components/TradingInterface.tsx
import { useProgram } from '../hooks/useProgram';
import { BN } from '@coral-xyz/anchor';

export const TradingInterface: React.FC = () => {
  const { program, userAccount } = useProgram();
  
  const createSellOrder = async (amount: number, pricePerKwh: number) => {
    if (!program || !userAccount) return;
    
    try {
      const tx = await program.methods
        .createSellOrder(new BN(amount), new BN(pricePerKwh))
        .accounts({
          seller: userAccount,
          // Other required accounts
        })
        .rpc();
        
      console.log('Sell order created:', tx);
    } catch (error) {
      console.error('Error creating sell order:', error);
    }
  };
  
  return (
    <div className="trading-interface">
      {/* Trading UI components */}
    </div>
  );
};
```

### **Phase 6: Testing and Deployment (2 weeks)**

#### **6.1 Integration Testing**
```typescript
// tests/integration.test.ts
import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { expect } from 'chai';

describe('P2P Energy Trading', () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  
  const program = anchor.workspace.Registry as Program<Registry>;
  
  it('Should register a new user', async () => {
    const user = anchor.web3.Keypair.generate();
    
    await program.methods
      .registerUser({ prosumer: {} }, 'Engineering Building')
      .accounts({
        userAccount: user.publicKey,
        userAuthority: provider.wallet.publicKey,
        registry: registryPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([user])
      .rpc();
      
    const userAccount = await program.account.userAccount.fetch(user.publicKey);
    expect(userAccount.location).to.equal('Engineering Building');
  });
});
```

#### **6.2 Deployment Scripts**
```bash
#!/bin/bash
# deploy.sh

echo "Deploying P2P Energy Trading to Solana Permissioned Network..."

# Build all programs
anchor build

# Deploy programs
anchor deploy --provider.cluster localnet

# Initialize programs
anchor run initialize

echo "Deployment completed!"
```

## Risk Mitigation

### **Technical Risks**
1. **Learning Curve**: Solana/Anchor differs from ink!/Substrate
   - **Mitigation**: Intensive training and documentation study
   
2. **Program Size Limits**: Solana has 10MB program size limit
   - **Mitigation**: Modular program design, code optimization
   
3. **Account Model Complexity**: Different from Substrate's storage model
   - **Mitigation**: Careful account structure planning

### **Operational Risks**
1. **Network Stability**: Permissioned network configuration
   - **Mitigation**: Thorough testing, validator redundancy
   
2. **Migration Downtime**: Service interruption during migration
   - **Mitigation**: Parallel deployment, phased migration

## Timeline Summary

| Phase | Duration | Description |
|-------|----------|-------------|
| 1 | 2 weeks | Environment Setup and Planning |
| 2 | 6 weeks | Smart Contract Migration |
| 3 | 2 weeks | Permissioned Network Configuration |
| 4 | 3 weeks | API Gateway Migration |
| 5 | 3 weeks | Frontend Migration |
| 6 | 2 weeks | Testing and Deployment |
| **Total** | **18 weeks** | **Complete Migration** |

## Success Metrics

### **Performance Improvements**
- Transaction throughput: >5000 TPS with PoA (vs ~100 TPS with Substrate)
- Transaction finality: <400ms with PoA consensus (vs ~6 seconds with Substrate)
- Transaction costs: <$0.0001 per transaction (no staking costs)
- Network governance: Instant validator updates by university authority

### **Functional Requirements**
- All 4 smart contracts successfully migrated and functional
- User registration and authentication working
- Energy token minting and trading operational
- Oracle data integration functional
- Frontend fully integrated with Solana programs

## Post-Migration Benefits

### **Technical Advantages**
1. **Superior Performance**: PoA consensus provides 5000+ TPS with <400ms finality
2. **Minimal Costs**: No staking requirements, ultra-low transaction fees
3. **Optimal UX**: Near-instant transaction confirmation for users
4. **Ecosystem Access**: Leverage Solana's mature tooling and libraries
5. **University Scalability**: Easily scale from dorms to entire campus
6. **Deterministic Operation**: Predictable performance with known validators
7. **Simple Governance**: Direct university control over network parameters

### **Business Advantages**
1. **Future-Proof**: Solana's growing ecosystem and adoption
2. **Cost Efficiency**: Lower operational costs
3. **Performance**: Better user experience with faster transactions
4. **Innovation**: Access to cutting-edge blockchain technology

This migration plan provides a comprehensive roadmap for transitioning from ink! to Solana Anchor while maintaining the permissioned, centralized control appropriate for a university environment.
