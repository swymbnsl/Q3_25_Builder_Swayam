use anchor_lang::prelude::*;

declare_id!("13XYUg7sLLxxZen7jMRf6VPxPreVievei1JdQ4TrZU2h");

#[program]
pub mod anchor_vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
