use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer, MintTo, Burn};

declare_id!("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");

#[program]
pub mod energy_token {
    use super::*;
    
    /// Initialize the energy token program
    pub fn initialize_token(ctx: Context<InitializeToken>) -> Result<()> {
        let token_info = &mut ctx.accounts.token_info;
        token_info.authority = ctx.accounts.authority.key();
        token_info.mint = ctx.accounts.mint.key();
        token_info.total_supply = 0;
        token_info.rec_validators = Vec::new();
        token_info.created_at = Clock::get()?.unix_timestamp;
        
        emit!(TokenInitialized {
            authority: ctx.accounts.authority.key(),
            mint: ctx.accounts.mint.key(),
            timestamp: Clock::get()?.unix_timestamp,
        });
        
        Ok(())
    }
    
    /// Add a REC validator to the system
    pub fn add_rec_validator(
        ctx: Context<AddRecValidator>,
        validator_pubkey: Pubkey,
        authority_name: String, // e.g., "University Sustainability Office"
    ) -> Result<()> {
        let token_info = &mut ctx.accounts.token_info;
        
        require!(
            ctx.accounts.authority.key() == token_info.authority,
            ErrorCode::UnauthorizedAuthority
        );
        
        // Check if validator already exists
        require!(
            !token_info.rec_validators.iter().any(|v| v.pubkey == validator_pubkey),
            ErrorCode::ValidatorAlreadyExists
        );
        
        let validator_info = RecValidatorInfo {
            pubkey: validator_pubkey,
            authority_name,
            certification_authority: true,
            active: true,
            added_at: Clock::get()?.unix_timestamp,
        };
        
        token_info.rec_validators.push(validator_info);
        
        emit!(RecValidatorAdded {
            validator: validator_pubkey,
            authority: ctx.accounts.authority.key(),
            timestamp: Clock::get()?.unix_timestamp,
        });
        
        Ok(())
    }
    
    /// Mint energy tokens with REC validation
    pub fn mint_energy_tokens_with_rec(
        ctx: Context<MintEnergyTokensWithRec>,
        amount: u64,
        meter_id: String,
        rec_certificate_id: String,
        renewable_source: RenewableSource,
    ) -> Result<()> {
        let token_info = &ctx.accounts.token_info;
        
        // Verify REC validator is authorized
        require!(
            token_info.rec_validators.iter().any(|v| 
                v.pubkey == ctx.accounts.rec_validator.key() && 
                v.active && 
                v.certification_authority
            ),
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
        rec_certificate.retired_at = None;
        rec_certificate.retired_by = Pubkey::default();
        
        // Mint tokens representing certified renewable energy
        let seeds = &[b"token_authority", &[ctx.bumps.token_authority]];
        let signer = &[&seeds[..]];
        
        let cpi_accounts = MintTo {
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
    
    /// Retire a REC certificate
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
    
    /// Transfer energy tokens between accounts
    pub fn transfer_tokens(
        ctx: Context<TransferTokens>,
        amount: u64,
    ) -> Result<()> {
        let cpi_accounts = Transfer {
            from: ctx.accounts.from_token_account.to_account_info(),
            to: ctx.accounts.to_token_account.to_account_info(),
            authority: ctx.accounts.from_authority.to_account_info(),
        };
        
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        
        token::transfer(cpi_ctx, amount)?;
        
        emit!(TokensTransferred {
            from: ctx.accounts.from_authority.key(),
            to: ctx.accounts.to_token_account.owner,
            amount,
            timestamp: Clock::get()?.unix_timestamp,
        });
        
        Ok(())
    }
    
    /// Burn energy tokens (for energy consumption)
    pub fn burn_tokens(
        ctx: Context<BurnTokens>,
        amount: u64,
    ) -> Result<()> {
        let cpi_accounts = Burn {
            mint: ctx.accounts.mint.to_account_info(),
            from: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        
        token::burn(cpi_ctx, amount)?;
        
        let token_info = &mut ctx.accounts.token_info;
        token_info.total_supply = token_info.total_supply.saturating_sub(amount);
        
        emit!(TokensBurned {
            user: ctx.accounts.authority.key(),
            amount,
            timestamp: Clock::get()?.unix_timestamp,
        });
        
        Ok(())
    }
}

// Account structs
#[derive(Accounts)]
pub struct InitializeToken<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + TokenInfo::LEN,
        seeds = [b"token_info"],
        bump
    )]
    pub token_info: Account<'info, TokenInfo>,
    
    pub mint: Account<'info, Mint>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddRecValidator<'info> {
    #[account(mut, has_one = authority @ ErrorCode::UnauthorizedAuthority)]
    pub token_info: Account<'info, TokenInfo>,
    
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(rec_certificate_id: String)]
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
    
    pub rec_validator: Signer<'info>,
    
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RetireRecCertificate<'info> {
    #[account(mut)]
    pub rec_certificate: Account<'info, RecCertificate>,
    
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct TransferTokens<'info> {
    #[account(mut)]
    pub from_token_account: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub to_token_account: Account<'info, TokenAccount>,
    
    pub from_authority: Signer<'info>,
    
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct BurnTokens<'info> {
    #[account(mut)]
    pub token_info: Account<'info, TokenInfo>,
    
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,
    
    pub authority: Signer<'info>,
    
    pub token_program: Program<'info, Token>,
}

// Data structs
#[account]
pub struct TokenInfo {
    pub authority: Pubkey,
    pub mint: Pubkey,
    pub total_supply: u64,
    pub rec_validators: Vec<RecValidatorInfo>,
    pub created_at: i64,
}

impl TokenInfo {
    pub const MAX_VALIDATORS: usize = 10;
    pub const LEN: usize = 32 + 32 + 8 + 4 + (RecValidatorInfo::LEN * Self::MAX_VALIDATORS) + 8;
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct RecValidatorInfo {
    pub pubkey: Pubkey,
    pub authority_name: String, // e.g., "University Sustainability Office"
    pub certification_authority: bool,
    pub active: bool,
    pub added_at: i64,
}

impl RecValidatorInfo {
    pub const LEN: usize = 32 + 64 + 1 + 1 + 8; // pubkey + authority_name + certification_authority + active + added_at
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

impl RecCertificate {
    pub const LEN: usize = 64 + 64 + 8 + 1 + 32 + 8 + 1 + 9 + 32;
}

// Enums
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

// Events
#[event]
pub struct TokenInitialized {
    pub authority: Pubkey,
    pub mint: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct RecValidatorAdded {
    pub validator: Pubkey,
    pub authority: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct EnergyTokensWithRecMinted {
    pub user: Pubkey,
    pub amount: u64,
    pub meter_id: String,
    pub rec_certificate_id: String,
    pub renewable_source: RenewableSource,
    pub validator: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct RecCertificateRetired {
    pub certificate_id: String,
    pub retired_by: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct TokensTransferred {
    pub from: Pubkey,
    pub to: Pubkey,
    pub amount: u64,
    pub timestamp: i64,
}

#[event]
pub struct TokensBurned {
    pub user: Pubkey,
    pub amount: u64,
    pub timestamp: i64,
}

// Errors
#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized authority")]
    UnauthorizedAuthority,
    #[msg("Unauthorized REC validator")]
    UnauthorizedRecValidator,
    #[msg("Invalid meter")]
    InvalidMeter,
    #[msg("Invalid certificate ID")]
    InvalidCertificateId,
    #[msg("Certificate already retired")]
    CertificateAlreadyRetired,
    #[msg("Validator already exists")]
    ValidatorAlreadyExists,
    #[msg("Insufficient token balance")]
    InsufficientBalance,
}
