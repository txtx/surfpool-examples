use crate::prelude::*;
use crate::constants::{TESSERA_PROGRAM};
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshDeserialize, BorshSerialize)]
pub struct SwapParams {
    pub side: u8,
    pub amount_in: u64,
    pub min_amount_out: u64,
}

fn create_instruction_data(side: u8, amount_in: u64) -> Vec<u8> {
    let swap_params = SwapParams {
        side,
        amount_in,
        min_amount_out: 1,
    };
    
    let mut buffer = Vec::with_capacity(18);
    buffer.extend_from_slice(&[0x6f, 0xc4, 0x16, 0xa0, 0x8c, 0x90, 0x4b, 0x4c]); // TESSERA_SWAP_SELECTOR
    buffer.extend_from_slice(&swap_params.try_to_vec().unwrap());
    buffer
}

pub fn create_tessera_swap_ix(
    global_state: &Pubkey,
    pool_state: &Pubkey,
    user: &Pubkey,
    base_vault: &Pubkey,
    quote_vault: &Pubkey,
    user_base_token: &Pubkey,
    user_quote_token: &Pubkey,
    base_mint: &Pubkey,
    quote_mint: &Pubkey,
    base_token_program: &Pubkey,
    quote_token_program: &Pubkey,
    user_source_token_mint: &Pubkey,
    amount: u64,
) -> Instruction {
    tracing::debug!("Creating Tessera swap instruction with the following details:");
    tracing::debug!("  Program ID: {}", TESSERA_PROGRAM);
    tracing::debug!("  Accounts:");
    tracing::debug!("    1. global_state: {} (readonly)", global_state);
    tracing::debug!("    2. pool_state: {} (mutable)", pool_state);
    tracing::debug!("    3. user: {} (signer)", user);
    tracing::debug!("    4. base_vault: {} (mutable)", base_vault);
    tracing::debug!("    5. quote_vault: {} (mutable)", quote_vault);
    tracing::debug!("    6. user_base_token: {} (mutable)", user_base_token);
    tracing::debug!("    7. user_quote_token: {} (mutable)", user_quote_token);
    tracing::debug!("    8. base_mint: {} (readonly)", base_mint);
    tracing::debug!("    9. quote_mint: {} (readonly)", quote_mint);
    tracing::debug!("    10. base_token_program: {} (readonly)", base_token_program);
    tracing::debug!("    11. quote_token_program: {} (readonly)", quote_token_program);
    tracing::debug!("    12. sysvar_instructions: {} (readonly)", sysvar::instructions::id());

    // Determine swap side: 1 for base->quote, 0 for quote->base
    let side = if user_source_token_mint == base_mint { 1 } else { 0 };
    
    tracing::debug!("  Args:");
    tracing::debug!("    side: {} ({})", side, if side == 1 { "base->quote" } else { "quote->base" });
    tracing::debug!("    amount: {}", amount);

    Instruction {
        program_id: TESSERA_PROGRAM,
        accounts: vec![
            AccountMeta::new_readonly(*global_state, false),
            AccountMeta::new(*pool_state, false),
            AccountMeta::new(*user, true),
            AccountMeta::new(*base_vault, false),
            AccountMeta::new(*quote_vault, false),
            AccountMeta::new(*user_base_token, false),
            AccountMeta::new(*user_quote_token, false),
            AccountMeta::new_readonly(*base_mint, false),
            AccountMeta::new_readonly(*quote_mint, false),
            AccountMeta::new_readonly(*base_token_program, false),
            AccountMeta::new_readonly(*quote_token_program, false),
            AccountMeta::new_readonly(sysvar::instructions::id(), false),
        ],
        data: create_instruction_data(side, amount),
    }
}