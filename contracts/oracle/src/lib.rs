use anchor_lang::prelude::*;
use anchor_lang::solana_program::hash::hash;

declare_id!("BrFkiEYxP6saiVHVBoaDYJZ2RRZnoXoo9P47PuwP9tHA");

#[program]
pub mod oracle {
    use super::*;
    
    /// Initialize the oracle system
    pub fn initialize_oracle(ctx: Context<InitializeOracle>) -> Result<()> {
        let oracle_config = &mut ctx.accounts.oracle_config;
        oracle_config.authority = ctx.accounts.authority.key();
        oracle_config.registry_program = ctx.accounts.registry_program.key();
        oracle_config.energy_token_program = ctx.accounts.energy_token_program.key();
        oracle_config.trading_program = ctx.accounts.trading_program.key();
        oracle_config.next_request_id = 1;
        oracle_config.operators = Vec::new();
        oracle_config.auto_market_clearing = true;
        oracle_config.market_clearing_interval = 3600; // 1 hour
        oracle_config.created_at = Clock::get()?.unix_timestamp;
        
        emit!(OracleInitialized {
            authority: ctx.accounts.authority.key(),
            timestamp: Clock::get()?.unix_timestamp,
        });
        
        Ok(())
    }
    
    /// Add an oracle operator
    pub fn add_oracle_operator(
        ctx: Context<AddOracleOperator>,
        operator: Pubkey,
        operator_type: OperatorType,
    ) -> Result<()> {
        let oracle_config = &mut ctx.accounts.oracle_config;
        
        require!(
            ctx.accounts.authority.key() == oracle_config.authority,
            ErrorCode::UnauthorizedAuthority
        );
        
        let operator_info = OracleOperator {
            pubkey: operator,
            operator_type,
            active: true,
            added_at: Clock::get()?.unix_timestamp,
        };
        
        oracle_config.operators.push(operator_info);
        
        emit!(OracleOperatorAdded {
            operator,
            operator_type,
            authority: ctx.accounts.authority.key(),
            timestamp: Clock::get()?.unix_timestamp,
        });
        
        Ok(())
    }
    
    /// Submit meter reading data from AMI system
    pub fn submit_meter_data(
        ctx: Context<SubmitMeterData>,
        meter_id: String,
        energy_generated: u64,
        energy_consumed: u64,
        reading_timestamp: i64,
        signature: Vec<u8>,
    ) -> Result<()> {
        let oracle_config = &ctx.accounts.oracle_config;
        let meter_data = &mut ctx.accounts.meter_data;
        
        // Verify oracle operator
        require!(
            oracle_config.operators.iter().any(|op| 
                op.pubkey == ctx.accounts.oracle_operator.key() && 
                op.active &&
                matches!(op.operator_type, OperatorType::AmiIntegration)
            ),
            ErrorCode::UnauthorizedOracleOperator
        );
        
        // Store meter data
        meter_data.meter_id = meter_id.clone();
        meter_data.energy_generated = energy_generated;
        meter_data.energy_consumed = energy_consumed;
        meter_data.reading_timestamp = reading_timestamp;
        meter_data.signature = signature;
        meter_data.submitted_at = Clock::get()?.unix_timestamp;
        meter_data.oracle_operator = ctx.accounts.oracle_operator.key();
        meter_data.processed = false;
        
        emit!(MeterDataSubmitted {
            meter_id,
            energy_generated,
            energy_consumed,
            reading_timestamp,
            oracle_operator: ctx.accounts.oracle_operator.key(),
            timestamp: Clock::get()?.unix_timestamp,
        });
        
        Ok(())
    }
    
    /// Process meter data and trigger token minting
    pub fn process_meter_data(
        ctx: Context<ProcessMeterData>,
        rec_certificate_id: String,
        renewable_source: RenewableSource,
    ) -> Result<()> {
        let meter_data = &mut ctx.accounts.meter_data;
        let oracle_config = &ctx.accounts.oracle_config;
        
        // Verify oracle operator
        require!(
            oracle_config.operators.iter().any(|op| 
                op.pubkey == ctx.accounts.oracle_operator.key() && 
                op.active
            ),
            ErrorCode::UnauthorizedOracleOperator
        );
        
        require!(!meter_data.processed, ErrorCode::DataAlreadyProcessed);
        
        // Calculate net energy generation
        let net_energy = meter_data.energy_generated.saturating_sub(meter_data.energy_consumed);
        
        if net_energy > 0 {
            // Call energy token program to mint tokens with REC
            let cpi_program = ctx.accounts.energy_token_program.to_account_info();
            let cpi_accounts = energy_token::cpi::accounts::MintEnergyTokensWithRec {
                token_info: ctx.accounts.token_info.to_account_info(),
                rec_certificate: ctx.accounts.rec_certificate.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                user_token_account: ctx.accounts.user_token_account.to_account_info(),
                token_authority: ctx.accounts.token_authority.to_account_info(),
                user_authority: ctx.accounts.user_authority.to_account_info(),
                rec_validator: ctx.accounts.rec_validator.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
            };
            
            let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
            
            energy_token::cpi::mint_energy_tokens_with_rec(
                cpi_ctx,
                net_energy,
                meter_data.meter_id.clone(),
                rec_certificate_id,
                renewable_source,
            )?;
        }
        
        meter_data.processed = true;
        
        emit!(MeterDataProcessed {
            meter_id: meter_data.meter_id.clone(),
            net_energy,
            oracle_operator: ctx.accounts.oracle_operator.key(),
            timestamp: Clock::get()?.unix_timestamp,
        });
        
        Ok(())
    }
    
    /// Request oracle data
    pub fn request_oracle_data(
        ctx: Context<RequestOracleData>,
        request_type: RequestType,
    ) -> Result<()> {
        let oracle_config = &mut ctx.accounts.oracle_config;
        let oracle_request = &mut ctx.accounts.oracle_request;
        
        let request_id = oracle_config.next_request_id;
        oracle_config.next_request_id += 1;
        
        oracle_request.request_id = request_id;
        oracle_request.requester = ctx.accounts.requester.key();
        oracle_request.request_type = request_type;
        oracle_request.status = RequestStatus::Pending;
        oracle_request.requested_at = Clock::get()?.unix_timestamp;
        oracle_request.expires_at = Clock::get()?.unix_timestamp + 3600; // 1 hour timeout
        
        emit!(OracleDataRequested {
            request_id,
            requester: ctx.accounts.requester.key(),
            request_type,
            timestamp: Clock::get()?.unix_timestamp,
        });
        
        Ok(())
    }
    
    /// Fulfill oracle request
    pub fn fulfill_oracle_request(
        ctx: Context<FulfillOracleRequest>,
        request_data: Vec<u8>,
    ) -> Result<()> {
        let oracle_request = &mut ctx.accounts.oracle_request;
        let oracle_config = &ctx.accounts.oracle_config;
        
        // Verify oracle operator
        require!(
            oracle_config.operators.iter().any(|op| 
                op.pubkey == ctx.accounts.oracle_operator.key() && 
                op.active
            ),
            ErrorCode::UnauthorizedOracleOperator
        );
        
        require!(
            oracle_request.status == RequestStatus::Pending,
            ErrorCode::RequestNotPending
        );
        
        require!(
            Clock::get()?.unix_timestamp <= oracle_request.expires_at,
            ErrorCode::RequestExpired
        );
        
        oracle_request.status = RequestStatus::Fulfilled;
        oracle_request.fulfilled_at = Some(Clock::get()?.unix_timestamp);
        oracle_request.response_data = request_data;
        
        emit!(OracleRequestFulfilled {
            request_id: oracle_request.request_id,
            oracle_operator: ctx.accounts.oracle_operator.key(),
            timestamp: Clock::get()?.unix_timestamp,
        });
        
        Ok(())
    }
    
    /// Trigger market clearing
    pub fn trigger_market_clearing(ctx: Context<TriggerMarketClearing>) -> Result<()> {
        let oracle_config = &ctx.accounts.oracle_config;
        
        // Verify oracle operator
        require!(
            oracle_config.operators.iter().any(|op| 
                op.pubkey == ctx.accounts.oracle_operator.key() && 
                op.active &&
                matches!(op.operator_type, OperatorType::MarketClearing)
            ),
            ErrorCode::UnauthorizedOracleOperator
        );
        
        require!(oracle_config.auto_market_clearing, ErrorCode::MarketClearingDisabled);
        
        emit!(MarketClearingTriggered {
            oracle_operator: ctx.accounts.oracle_operator.key(),
            timestamp: Clock::get()?.unix_timestamp,
        });
        
        Ok(())
    }
}

// Account structs
#[derive(Accounts)]
pub struct InitializeOracle<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + OracleConfig::LEN,
        seeds = [b"oracle_config"],
        bump
    )]
    pub oracle_config: Account<'info, OracleConfig>,
    
    /// CHECK: Registry program ID
    pub registry_program: AccountInfo<'info>,
    
    /// CHECK: Energy token program ID
    pub energy_token_program: AccountInfo<'info>,
    
    /// CHECK: Trading program ID
    pub trading_program: AccountInfo<'info>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddOracleOperator<'info> {
    #[account(mut, has_one = authority @ ErrorCode::UnauthorizedAuthority)]
    pub oracle_config: Account<'info, OracleConfig>,
    
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(meter_id: String)]
pub struct SubmitMeterData<'info> {
    pub oracle_config: Account<'info, OracleConfig>,
    
    #[account(
        init,
        payer = oracle_operator,
        space = 8 + MeterData::LEN,
        seeds = [b"meter_data", meter_id.as_bytes(), &Clock::get().unwrap().unix_timestamp.to_le_bytes()],
        bump
    )]
    pub meter_data: Account<'info, MeterData>,
    
    #[account(mut)]
    pub oracle_operator: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ProcessMeterData<'info> {
    pub oracle_config: Account<'info, OracleConfig>,
    
    #[account(mut)]
    pub meter_data: Account<'info, MeterData>,
    
    /// CHECK: Energy token program
    pub energy_token_program: AccountInfo<'info>,
    
    /// CHECK: Token info account
    #[account(mut)]
    pub token_info: AccountInfo<'info>,
    
    /// CHECK: REC certificate account
    #[account(mut)]
    pub rec_certificate: AccountInfo<'info>,
    
    /// CHECK: Token mint
    #[account(mut)]
    pub mint: AccountInfo<'info>,
    
    /// CHECK: User token account
    #[account(mut)]
    pub user_token_account: AccountInfo<'info>,
    
    /// CHECK: Token authority
    pub token_authority: AccountInfo<'info>,
    
    /// CHECK: User authority
    pub user_authority: AccountInfo<'info>,
    
    pub rec_validator: Signer<'info>,
    pub oracle_operator: Signer<'info>,
    
    /// CHECK: Token program
    pub token_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RequestOracleData<'info> {
    #[account(mut)]
    pub oracle_config: Account<'info, OracleConfig>,
    
    #[account(
        init,
        payer = requester,
        space = 8 + OracleRequest::LEN,
        seeds = [b"oracle_request", requester.key().as_ref(), &oracle_config.next_request_id.to_le_bytes()],
        bump
    )]
    pub oracle_request: Account<'info, OracleRequest>,
    
    #[account(mut)]
    pub requester: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct FulfillOracleRequest<'info> {
    pub oracle_config: Account<'info, OracleConfig>,
    
    #[account(mut)]
    pub oracle_request: Account<'info, OracleRequest>,
    
    pub oracle_operator: Signer<'info>,
}

#[derive(Accounts)]
pub struct TriggerMarketClearing<'info> {
    pub oracle_config: Account<'info, OracleConfig>,
    
    pub oracle_operator: Signer<'info>,
}

// Data structs
#[account]
pub struct OracleConfig {
    pub authority: Pubkey,
    pub registry_program: Pubkey,
    pub energy_token_program: Pubkey,
    pub trading_program: Pubkey,
    pub next_request_id: u64,
    pub operators: Vec<OracleOperator>,
    pub auto_market_clearing: bool,
    pub market_clearing_interval: i64,
    pub created_at: i64,
}

impl OracleConfig {
    pub const MAX_OPERATORS: usize = 20;
    pub const LEN: usize = 32 + 32 + 32 + 32 + 8 + 4 + (OracleOperator::LEN * Self::MAX_OPERATORS) + 1 + 8 + 8;
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct OracleOperator {
    pub pubkey: Pubkey,
    pub operator_type: OperatorType,
    pub active: bool,
    pub added_at: i64,
}

impl OracleOperator {
    pub const LEN: usize = 32 + 1 + 1 + 8;
}

#[account]
pub struct MeterData {
    pub meter_id: String,
    pub energy_generated: u64,
    pub energy_consumed: u64,
    pub reading_timestamp: i64,
    pub signature: Vec<u8>,
    pub submitted_at: i64,
    pub oracle_operator: Pubkey,
    pub processed: bool,
}

impl MeterData {
    pub const LEN: usize = 64 + 8 + 8 + 8 + 4 + 128 + 8 + 32 + 1; // signature max 128 bytes
}

#[account]
pub struct OracleRequest {
    pub request_id: u64,
    pub requester: Pubkey,
    pub request_type: RequestType,
    pub status: RequestStatus,
    pub requested_at: i64,
    pub expires_at: i64,
    pub fulfilled_at: Option<i64>,
    pub response_data: Vec<u8>,
}

impl OracleRequest {
    pub const LEN: usize = 8 + 32 + 1 + 1 + 8 + 8 + 9 + 4 + 256; // response_data max 256 bytes
}

// Enums
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum OperatorType {
    AmiIntegration,
    MarketClearing,
    PriceOracle,
    WeatherData,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum RequestType {
    EnergyData { meter_id: String },
    MarketClearing,
    PriceData,
    WeatherData { location: String },
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum RequestStatus {
    Pending,
    Fulfilled,
    Expired,
    Failed,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum RenewableSource {
    Solar,
    Wind,
    Hydro,
    Geothermal,
    Biomass,
}

// Events
#[event]
pub struct OracleInitialized {
    pub authority: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct OracleOperatorAdded {
    pub operator: Pubkey,
    pub operator_type: OperatorType,
    pub authority: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct MeterDataSubmitted {
    pub meter_id: String,
    pub energy_generated: u64,
    pub energy_consumed: u64,
    pub reading_timestamp: i64,
    pub oracle_operator: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct MeterDataProcessed {
    pub meter_id: String,
    pub net_energy: u64,
    pub oracle_operator: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct OracleDataRequested {
    pub request_id: u64,
    pub requester: Pubkey,
    pub request_type: RequestType,
    pub timestamp: i64,
}

#[event]
pub struct OracleRequestFulfilled {
    pub request_id: u64,
    pub oracle_operator: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct MarketClearingTriggered {
    pub oracle_operator: Pubkey,
    pub timestamp: i64,
}

// Errors
#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized authority")]
    UnauthorizedAuthority,
    #[msg("Unauthorized oracle operator")]
    UnauthorizedOracleOperator,
    #[msg("Data already processed")]
    DataAlreadyProcessed,
    #[msg("Request not pending")]
    RequestNotPending,
    #[msg("Request expired")]
    RequestExpired,
    #[msg("Market clearing disabled")]
    MarketClearingDisabled,
    #[msg("Invalid signature")]
    InvalidSignature,
}

// External CPI modules
pub mod energy_token {
    use super::*;
    
    pub mod cpi {
        use super::*;
        
        pub mod accounts {
            use super::*;
            
            pub struct MintEnergyTokensWithRec<'info> {
                pub token_info: AccountInfo<'info>,
                pub rec_certificate: AccountInfo<'info>,
                pub mint: AccountInfo<'info>,
                pub user_token_account: AccountInfo<'info>,
                pub token_authority: AccountInfo<'info>,
                pub user_authority: AccountInfo<'info>,
                pub rec_validator: AccountInfo<'info>,
                pub token_program: AccountInfo<'info>,
                pub system_program: AccountInfo<'info>,
            }
        }
        
        pub fn mint_energy_tokens_with_rec(
            _ctx: CpiContext<accounts::MintEnergyTokensWithRec>,
            _amount: u64,
            _meter_id: String,
            _rec_certificate_id: String,
            _renewable_source: RenewableSource,
        ) -> Result<()> {
            // This would be implemented as actual CPI call
            Ok(())
        }
    }
}
