#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

declare_id!("Akbajztf6ZSGXytTXKDd7MsGVviNCaEumvigp5BWaV8a");

#[program]
pub mod simple_pda {
    use super::*;

    pub fn run(ctx: Context<Run>) -> Result<()> {
        let custom = &mut ctx.accounts.custom;
        custom.counter = custom.counter.wrapping_add(1);
        msg!(
            "Greetings from: {:?}. The counter for custom account {:?} is {}",
            ctx.program_id,
            custom.key(),
            custom.counter
        );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Run<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,
    #[account(
        init_if_needed,
        payer = sender,
        seeds = [b"custom", sender.key().as_ref()],
        bump,
        space = 8 + CustomAccount::INIT_SPACE,
    )]
    pub custom: Account<'info, CustomAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct CustomAccount {
    pub counter: u64,
}
