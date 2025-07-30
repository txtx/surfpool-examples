use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};

#[event_cpi]
#[derive(Accounts)]
pub struct SplitTokenTransfer<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,

    #[account(mut)]
    pub recipient_1: SystemAccount<'info>,
    #[account(mut)]
    pub recipient_2: SystemAccount<'info>,

    #[account(
        mint::token_program = token_program,
    )]
    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = sender,
        associated_token::token_program = token_program,
    )]
    pub sender_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = sender,
        seeds = [b"custom", sender.key().as_ref()],
        bump,
        space = 8 + CustomAccount::INIT_SPACE,
    )]
    pub custom: Account<'info, CustomAccount>,

    #[account(
        init_if_needed,
        payer = sender,
        associated_token::mint = mint,
        associated_token::authority = recipient_1,
        associated_token::token_program = token_program,
    )]
    pub recipient_1_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = sender,
        associated_token::mint = mint,
        associated_token::authority = recipient_2,
        associated_token::token_program = token_program,
    )]
    pub recipient_2_token_account: InterfaceAccount<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
}

#[account]
#[derive(InitSpace)]
pub struct CustomAccount {
    pub my_custom_data: u64,
}

impl<'info> SplitTokenTransfer<'info> {
    pub fn transfer_tokens(
        &mut self,
        recipient_1_amount: u64,
        recipient_2_amount: u64,
    ) -> Result<()> {
        assert!(
            self.sender_token_account.amount >= recipient_1_amount + recipient_2_amount,
            "Insufficient balance for split transfer"
        );

        // Transfer tokens from sender to recipient 1
        let transfer_1 = TransferChecked {
            from: self.sender_token_account.to_account_info(),
            to: self.recipient_1_token_account.to_account_info(),
            authority: self.sender.to_account_info(),
            mint: self.mint.to_account_info(),
        };
        let transfer_1_cpi_ctx = CpiContext::new(self.token_program.to_account_info(), transfer_1);
        transfer_checked(transfer_1_cpi_ctx, recipient_1_amount, self.mint.decimals)?;

        // Transfer tokens from sender to recipient 2
        let transfer_2 = TransferChecked {
            from: self.sender_token_account.to_account_info(),
            to: self.recipient_2_token_account.to_account_info(),
            authority: self.sender.to_account_info(),
            mint: self.mint.to_account_info(),
        };
        let transfer_2_cpi_ctx = CpiContext::new(self.token_program.to_account_info(), transfer_2);
        transfer_checked(transfer_2_cpi_ctx, recipient_2_amount, self.mint.decimals)?;

        msg!("Split transfer complete.");
        Ok(())
    }
}

#[event]
/// The split transfer event.
pub struct SplitTransferEvent {
    /// The amount transferred to recipient 1.
    pub recipient_1_amount: u64,
    /// The amount transferred to recipient 2.
    pub recipient_1: Pubkey,
    /// The amount transferred to recipient 2.
    pub recipient_2_amount: u64,
    /// The amount transferred to recipient 2.
    pub recipient_2: Pubkey,
    /// Token mint address.
    pub mint: Pubkey,
    /// The program ID of the token program.
    pub token_program_id: Pubkey,
}

impl SplitTransferEvent {
    /// Create a new `SplitTransferEvent`.
    pub fn new(
        recipient_1_amount: u64,
        recipient_1: Pubkey,
        recipient_2_amount: u64,
        recipient_2: Pubkey,
        mint: Pubkey,
        token_program_id: Pubkey,
    ) -> Self {
        Self {
            recipient_1_amount,
            recipient_1,
            recipient_2_amount,
            recipient_2,
            mint,
            token_program_id,
        }
    }
}
