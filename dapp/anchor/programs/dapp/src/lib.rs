#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("Count3AcZucFDPSFBAeHkQ6AvttieKUkyJ8HiQGhQwe");

#[program]
pub mod dapp {
    use super::*;

    pub fn close(_ctx: Context<CloseDapp>) -> Result<()> {
        Ok(())
    }

    pub fn decrement(ctx: Context<Update>) -> Result<()> {
        ctx.accounts.dapp.count = ctx.accounts.dapp.count.checked_sub(1).unwrap();
        Ok(())
    }

    pub fn increment(ctx: Context<Update>) -> Result<()> {
        ctx.accounts.dapp.count = ctx.accounts.dapp.count.checked_add(1).unwrap();
        Ok(())
    }

    pub fn initialize(_ctx: Context<InitializeDapp>) -> Result<()> {
        Ok(())
    }

    pub fn set(ctx: Context<Update>, value: u8) -> Result<()> {
        ctx.accounts.dapp.count = value.clone();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeDapp<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
  init,
  space = 8 + Dapp::INIT_SPACE,
  payer = payer
    )]
    pub dapp: Account<'info, Dapp>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct CloseDapp<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
  mut,
  close = payer, // close account and return lamports to payer
    )]
    pub dapp: Account<'info, Dapp>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub dapp: Account<'info, Dapp>,
}

#[account]
#[derive(InitSpace)]
pub struct Dapp {
    count: u8,
}
