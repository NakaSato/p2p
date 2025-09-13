use anchor_lang::prelude::*;

declare_id!("2R68FVjvq6oRtpzJBq4Mxsw165wCL6wbFRSxzAqNkJro");

#[program]
pub mod oracle {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        msg!("Oracle program initialized");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    pub authority: Signer<'info>,
}
