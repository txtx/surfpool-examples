#![allow(unexpected_cfgs)]
mod instructions;

use anchor_lang::prelude::*;
use borsh::BorshDeserialize;

use instructions::*;

declare_id!("5cungpYw9gdabacb6kBT2nkYkd21rHrXdKSiBXZATRU9");

#[program]
pub mod split_token_transfer {

    use anchor_lang::prelude::*;

    use super::*;
    pub fn split_token_transfer(ctx: Context<SplitTokenTransfer>, amount: u64) -> Result<()> {
        // Calculate the amounts to transfer to each recipient
        // If the amount is odd, one recipient will get one more token than the other
        let half = amount / 2;
        let (recipient_1_amount, recipient_2_amount) = if amount % 2 == 0 {
            (half, half)
        } else {
            (half, half + 1)
        };

        ctx.accounts
            .transfer_tokens(recipient_1_amount, recipient_2_amount)?;
        emit_cpi!(SplitTransferEvent::new(
            recipient_1_amount,
            ctx.accounts.recipient_1.key(),
            recipient_2_amount,
            ctx.accounts.recipient_2.key(),
            ctx.accounts.mint.key(),
            ctx.accounts.token_program.key()
        ));
        Ok(())
    }
}
