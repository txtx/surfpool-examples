use anchor_lang::prelude::*;

declare_id!("2ZLd4kXwWg9by1ZxXYWNZNwDMiEDQbNrXuxRKY7ZU7LK");

#[program]
pub mod price_feed {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
