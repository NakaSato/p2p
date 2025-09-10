use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("TradingProgramId1234567890123456789");

#[program]
pub mod trading {
    use super::*;
    
    /// Initialize the trading market
    pub fn initialize_market(ctx: Context<InitializeMarket>) -> Result<()> {
        let market = &mut ctx.accounts.market;
        market.authority = ctx.accounts.authority.key();
        market.active_orders = 0;
        market.total_volume = 0;
        market.total_trades = 0;
        market.created_at = Clock::get()?.unix_timestamp;
        market.clearing_enabled = true;
        market.market_fee_bps = 25; // 0.25% fee
        
        emit!(MarketInitialized {
            authority: ctx.accounts.authority.key(),
            timestamp: Clock::get()?.unix_timestamp,
        });
        
        Ok(())
    }
    
    /// Create a sell order for energy
    pub fn create_sell_order(
        ctx: Context<CreateSellOrder>,
        amount: u64,
        price_per_kwh: u64,
    ) -> Result<()> {
        let order = &mut ctx.accounts.order;
        let market = &mut ctx.accounts.market;
        
        require!(amount > 0, ErrorCode::InvalidAmount);
        require!(price_per_kwh > 0, ErrorCode::InvalidPrice);
        
        // Setup order data
        order.seller = ctx.accounts.seller.key();
        order.amount = amount;
        order.filled_amount = 0;
        order.price_per_kwh = price_per_kwh;
        order.order_type = OrderType::Sell;
        order.status = OrderStatus::Active;
        order.created_at = Clock::get()?.unix_timestamp;
        order.expires_at = Clock::get()?.unix_timestamp + (24 * 60 * 60); // 24 hours
        
        // Escrow seller's tokens
        let cpi_accounts = Transfer {
            from: ctx.accounts.seller_token_account.to_account_info(),
            to: ctx.accounts.escrow_token_account.to_account_info(),
            authority: ctx.accounts.seller.to_account_info(),
        };
        
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        
        token::transfer(cpi_ctx, amount)?;
        
        // Update market stats
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
    
    /// Create a buy order for energy
    pub fn create_buy_order(
        ctx: Context<CreateBuyOrder>,
        amount: u64,
        price_per_kwh: u64,
    ) -> Result<()> {
        let order = &mut ctx.accounts.order;
        let market = &mut ctx.accounts.market;
        
        require!(amount > 0, ErrorCode::InvalidAmount);
        require!(price_per_kwh > 0, ErrorCode::InvalidPrice);
        
        // Setup order data
        order.buyer = ctx.accounts.buyer.key();
        order.amount = amount;
        order.filled_amount = 0;
        order.price_per_kwh = price_per_kwh;
        order.order_type = OrderType::Buy;
        order.status = OrderStatus::Active;
        order.created_at = Clock::get()?.unix_timestamp;
        order.expires_at = Clock::get()?.unix_timestamp + (24 * 60 * 60); // 24 hours
        
        // Update market stats
        market.active_orders += 1;
        
        emit!(BuyOrderCreated {
            buyer: ctx.accounts.buyer.key(),
            order_id: order.key(),
            amount,
            price_per_kwh,
            timestamp: Clock::get()?.unix_timestamp,
        });
        
        Ok(())
    }
    
    /// Match a buy order with a sell order
    pub fn match_orders(ctx: Context<MatchOrders>) -> Result<()> {
        let sell_order = &mut ctx.accounts.sell_order;
        let buy_order = &mut ctx.accounts.buy_order;
        let market = &mut ctx.accounts.market;
        let trade_record = &mut ctx.accounts.trade_record;
        
        // Validate orders
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
        
        // Calculate trade parameters
        let remaining_sell = sell_order.amount - sell_order.filled_amount;
        let remaining_buy = buy_order.amount - buy_order.filled_amount;
        let trade_amount = std::cmp::min(remaining_sell, remaining_buy);
        let trade_price = sell_order.price_per_kwh; // Use seller's price
        let total_value = trade_amount * trade_price;
        
        // Calculate market fee
        let fee_amount = (total_value * market.market_fee_bps as u64) / 10000;
        let net_value = total_value - fee_amount;
        
        // Transfer tokens from escrow to buyer
        let escrow_seeds = &[b"escrow", sell_order.key().as_ref(), &[ctx.bumps.escrow_authority]];
        let escrow_signer = &[&escrow_seeds[..]];
        
        let transfer_cpi_accounts = Transfer {
            from: ctx.accounts.escrow_token_account.to_account_info(),
            to: ctx.accounts.buyer_token_account.to_account_info(),
            authority: ctx.accounts.escrow_authority.to_account_info(),
        };
        
        let transfer_cpi_program = ctx.accounts.token_program.to_account_info();
        let transfer_cpi_ctx = CpiContext::new_with_signer(
            transfer_cpi_program, 
            transfer_cpi_accounts, 
            escrow_signer
        );
        
        token::transfer(transfer_cpi_ctx, trade_amount)?;
        
        // Record the trade
        trade_record.sell_order = sell_order.key();
        trade_record.buy_order = buy_order.key();
        trade_record.seller = sell_order.seller;
        trade_record.buyer = buy_order.buyer;
        trade_record.amount = trade_amount;
        trade_record.price_per_kwh = trade_price;
        trade_record.total_value = total_value;
        trade_record.fee_amount = fee_amount;
        trade_record.executed_at = Clock::get()?.unix_timestamp;
        
        // Update order status
        sell_order.filled_amount += trade_amount;
        buy_order.filled_amount += trade_amount;
        
        if sell_order.filled_amount == sell_order.amount {
            sell_order.status = OrderStatus::Completed;
            market.active_orders = market.active_orders.saturating_sub(1);
        } else {
            sell_order.status = OrderStatus::PartiallyFilled;
        }
        
        if buy_order.filled_amount == buy_order.amount {
            buy_order.status = OrderStatus::Completed;
            market.active_orders = market.active_orders.saturating_sub(1);
        } else {
            buy_order.status = OrderStatus::PartiallyFilled;
        }
        
        // Update market stats
        market.total_volume += total_value;
        market.total_trades += 1;
        
        emit!(OrderMatched {
            sell_order: sell_order.key(),
            buy_order: buy_order.key(),
            seller: sell_order.seller,
            buyer: buy_order.buyer,
            amount: trade_amount,
            price: trade_price,
            total_value,
            fee_amount,
            timestamp: Clock::get()?.unix_timestamp,
        });
        
        Ok(())
    }
    
    /// Cancel an active order
    pub fn cancel_order(ctx: Context<CancelOrder>) -> Result<()> {
        let order = &mut ctx.accounts.order;
        let market = &mut ctx.accounts.market;
        
        require!(
            order.status == OrderStatus::Active || order.status == OrderStatus::PartiallyFilled,
            ErrorCode::OrderNotCancellable
        );
        
        // For sell orders, return escrowed tokens
        if order.order_type == OrderType::Sell && order.filled_amount < order.amount {
            let remaining_amount = order.amount - order.filled_amount;
            
            let escrow_seeds = &[b"escrow", order.key().as_ref(), &[ctx.bumps.escrow_authority]];
            let escrow_signer = &[&escrow_seeds[..]];
            
            let return_cpi_accounts = Transfer {
                from: ctx.accounts.escrow_token_account.to_account_info(),
                to: ctx.accounts.user_token_account.to_account_info(),
                authority: ctx.accounts.escrow_authority.to_account_info(),
            };
            
            let return_cpi_program = ctx.accounts.token_program.to_account_info();
            let return_cpi_ctx = CpiContext::new_with_signer(
                return_cpi_program, 
                return_cpi_accounts, 
                escrow_signer
            );
            
            token::transfer(return_cpi_ctx, remaining_amount)?;
        }
        
        order.status = OrderStatus::Cancelled;
        market.active_orders = market.active_orders.saturating_sub(1);
        
        emit!(OrderCancelled {
            order_id: order.key(),
            user: ctx.accounts.user_authority.key(),
            timestamp: Clock::get()?.unix_timestamp,
        });
        
        Ok(())
    }
    
    /// Update market parameters (admin only)
    pub fn update_market_params(
        ctx: Context<UpdateMarketParams>,
        market_fee_bps: u16,
        clearing_enabled: bool,
    ) -> Result<()> {
        let market = &mut ctx.accounts.market;
        
        require!(
            ctx.accounts.authority.key() == market.authority,
            ErrorCode::UnauthorizedAuthority
        );
        
        market.market_fee_bps = market_fee_bps;
        market.clearing_enabled = clearing_enabled;
        
        emit!(MarketParamsUpdated {
            authority: ctx.accounts.authority.key(),
            market_fee_bps,
            clearing_enabled,
            timestamp: Clock::get()?.unix_timestamp,
        });
        
        Ok(())
    }
}

// Account structs
#[derive(Accounts)]
pub struct InitializeMarket<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + Market::LEN,
        seeds = [b"market"],
        bump
    )]
    pub market: Account<'info, Market>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateSellOrder<'info> {
    #[account(mut)]
    pub market: Account<'info, Market>,
    
    #[account(
        init,
        payer = seller,
        space = 8 + Order::LEN,
        seeds = [b"order", seller.key().as_ref(), &Clock::get().unwrap().unix_timestamp.to_le_bytes()],
        bump
    )]
    pub order: Account<'info, Order>,
    
    #[account(mut)]
    pub seller_token_account: Account<'info, TokenAccount>,
    
    #[account(
        init,
        payer = seller,
        space = TokenAccount::LEN,
        seeds = [b"escrow", order.key().as_ref()],
        bump,
        token::mint = seller_token_account.mint,
        token::authority = escrow_authority,
    )]
    pub escrow_token_account: Account<'info, TokenAccount>,
    
    /// CHECK: Escrow authority PDA
    #[account(
        seeds = [b"escrow", order.key().as_ref()],
        bump
    )]
    pub escrow_authority: AccountInfo<'info>,
    
    #[account(mut)]
    pub seller: Signer<'info>,
    
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateBuyOrder<'info> {
    #[account(mut)]
    pub market: Account<'info, Market>,
    
    #[account(
        init,
        payer = buyer,
        space = 8 + Order::LEN,
        seeds = [b"order", buyer.key().as_ref(), &Clock::get().unwrap().unix_timestamp.to_le_bytes()],
        bump
    )]
    pub order: Account<'info, Order>,
    
    #[account(mut)]
    pub buyer: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MatchOrders<'info> {
    #[account(mut)]
    pub market: Account<'info, Market>,
    
    #[account(mut)]
    pub sell_order: Account<'info, Order>,
    
    #[account(mut)]
    pub buy_order: Account<'info, Order>,
    
    #[account(
        init,
        payer = matcher,
        space = 8 + TradeRecord::LEN,
        seeds = [b"trade", sell_order.key().as_ref(), buy_order.key().as_ref()],
        bump
    )]
    pub trade_record: Account<'info, TradeRecord>,
    
    #[account(mut)]
    pub escrow_token_account: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub buyer_token_account: Account<'info, TokenAccount>,
    
    /// CHECK: Escrow authority PDA
    #[account(
        seeds = [b"escrow", sell_order.key().as_ref()],
        bump
    )]
    pub escrow_authority: AccountInfo<'info>,
    
    #[account(mut)]
    pub matcher: Signer<'info>,
    
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CancelOrder<'info> {
    #[account(mut)]
    pub market: Account<'info, Market>,
    
    #[account(mut, constraint = order.seller == user_authority.key() || order.buyer == user_authority.key())]
    pub order: Account<'info, Order>,
    
    #[account(mut)]
    pub escrow_token_account: Account<'info, TokenAccount>,
    
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,
    
    /// CHECK: Escrow authority PDA
    #[account(
        seeds = [b"escrow", order.key().as_ref()],
        bump
    )]
    pub escrow_authority: AccountInfo<'info>,
    
    pub user_authority: Signer<'info>,
    
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct UpdateMarketParams<'info> {
    #[account(mut, has_one = authority @ ErrorCode::UnauthorizedAuthority)]
    pub market: Account<'info, Market>,
    
    pub authority: Signer<'info>,
}

// Data structs
#[account]
pub struct Market {
    pub authority: Pubkey,
    pub active_orders: u64,
    pub total_volume: u64,
    pub total_trades: u64,
    pub created_at: i64,
    pub clearing_enabled: bool,
    pub market_fee_bps: u16,
}

impl Market {
    pub const LEN: usize = 32 + 8 + 8 + 8 + 8 + 1 + 2;
}

#[account]
pub struct Order {
    pub seller: Pubkey,
    pub buyer: Pubkey,
    pub amount: u64,
    pub filled_amount: u64,
    pub price_per_kwh: u64,
    pub order_type: OrderType,
    pub status: OrderStatus,
    pub created_at: i64,
    pub expires_at: i64,
}

impl Order {
    pub const LEN: usize = 32 + 32 + 8 + 8 + 8 + 1 + 1 + 8 + 8;
}

#[account]
pub struct TradeRecord {
    pub sell_order: Pubkey,
    pub buy_order: Pubkey,
    pub seller: Pubkey,
    pub buyer: Pubkey,
    pub amount: u64,
    pub price_per_kwh: u64,
    pub total_value: u64,
    pub fee_amount: u64,
    pub executed_at: i64,
}

impl TradeRecord {
    pub const LEN: usize = 32 + 32 + 32 + 32 + 8 + 8 + 8 + 8 + 8;
}

// Enums
#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum OrderType {
    Sell,
    Buy,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum OrderStatus {
    Active,
    PartiallyFilled,
    Completed,
    Cancelled,
    Expired,
}

// Events
#[event]
pub struct MarketInitialized {
    pub authority: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct SellOrderCreated {
    pub seller: Pubkey,
    pub order_id: Pubkey,
    pub amount: u64,
    pub price_per_kwh: u64,
    pub timestamp: i64,
}

#[event]
pub struct BuyOrderCreated {
    pub buyer: Pubkey,
    pub order_id: Pubkey,
    pub amount: u64,
    pub price_per_kwh: u64,
    pub timestamp: i64,
}

#[event]
pub struct OrderMatched {
    pub sell_order: Pubkey,
    pub buy_order: Pubkey,
    pub seller: Pubkey,
    pub buyer: Pubkey,
    pub amount: u64,
    pub price: u64,
    pub total_value: u64,
    pub fee_amount: u64,
    pub timestamp: i64,
}

#[event]
pub struct OrderCancelled {
    pub order_id: Pubkey,
    pub user: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct MarketParamsUpdated {
    pub authority: Pubkey,
    pub market_fee_bps: u16,
    pub clearing_enabled: bool,
    pub timestamp: i64,
}

// Errors
#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized authority")]
    UnauthorizedAuthority,
    #[msg("Invalid amount")]
    InvalidAmount,
    #[msg("Invalid price")]
    InvalidPrice,
    #[msg("Inactive sell order")]
    InactiveSellOrder,
    #[msg("Inactive buy order")]
    InactiveBuyOrder,
    #[msg("Price mismatch")]
    PriceMismatch,
    #[msg("Order not cancellable")]
    OrderNotCancellable,
    #[msg("Insufficient escrow balance")]
    InsufficientEscrowBalance,
}
