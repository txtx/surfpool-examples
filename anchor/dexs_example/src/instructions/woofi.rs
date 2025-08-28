use crate::prelude::*;
use crate::constants::{WOOFI_PROGRAM};
use crate::{WOOFI_SWAP_SELECTOR};

fn create_instruction_data(amount_in: u64) -> Vec<u8> {
    let mut buffer = Vec::with_capacity(40);
    buffer.extend_from_slice(WOOFI_SWAP_SELECTOR);
    buffer.extend_from_slice(&(amount_in as u128).to_le_bytes()); // amount_in as u128
    buffer.extend_from_slice(&1u128.to_le_bytes()); // mini_amount_out
    buffer
}

pub fn create_woofi_swap_ix(
    wooficonfig: &Pubkey,
    token_program: &Pubkey,
    user: &Pubkey,
    token_a_wooracle: &Pubkey,
    token_a_woopool: &Pubkey,
    user_source_token: &Pubkey,
    a_token_vault: &Pubkey,
    token_a_price_update: &Pubkey,
    token_b_wooracle: &Pubkey,
    token_b_woopool: &Pubkey,
    user_destination_token: &Pubkey,
    b_token_vault: &Pubkey,
    token_b_price_update: &Pubkey,
    quote_pool: &Pubkey,
    quote_price_update: &Pubkey,
    quote_token_vault: &Pubkey,
    rebate_to: &Pubkey,
    amount: u64,
) -> Instruction {
    // Print detailed debug information about the instruction we're creating
    tracing::debug!("Creating WooFi swap instruction with the following details:");
    tracing::debug!("  Program ID: {}", WOOFI_PROGRAM);
    tracing::debug!("  Accounts:");
    tracing::debug!("    1. wooficonfig: {} (mutable)", wooficonfig);
    tracing::debug!("    2. token_program: {} (readonly)", token_program);
    tracing::debug!("    3. user: {} (signer)", user);
    tracing::debug!("    4. token_a_wooracle: {} (mutable)", token_a_wooracle);
    tracing::debug!("    5. token_a_woopool: {} (mutable)", token_a_woopool);
    tracing::debug!("    6. user_source_token: {} (mutable)", user_source_token);
    tracing::debug!("    7. a_token_vault: {} (mutable)", a_token_vault);
    tracing::debug!("    8. token_a_price_update: {} (mutable)", token_a_price_update);
    tracing::debug!("    9. token_b_wooracle: {} (mutable)", token_b_wooracle);
    tracing::debug!("    10. token_b_woopool: {} (mutable)", token_b_woopool);
    tracing::debug!("    11. user_destination_token: {} (mutable)", user_destination_token);
    tracing::debug!("    12. b_token_vault: {} (mutable)", b_token_vault);
    tracing::debug!("    13. token_b_price_update: {} (mutable)", token_b_price_update);
    tracing::debug!("    14. quote_pool: {} (mutable)", quote_pool);
    tracing::debug!("    15. quote_price_update: {} (mutable)", quote_price_update);
    tracing::debug!("    16. quote_token_vault: {} (mutable)", quote_token_vault);
    tracing::debug!("    17. rebate_to: {} (mutable)", rebate_to);
    
    tracing::debug!("  Args:");
    tracing::debug!("    amount: {} (as u128: {})", amount, amount as u128);
    
    Instruction {
        program_id: WOOFI_PROGRAM,
        accounts: vec![
            AccountMeta::new(*wooficonfig, false),
            AccountMeta::new_readonly(*token_program, false),
            AccountMeta::new(*user, true),
            AccountMeta::new(*token_a_wooracle, false),
            AccountMeta::new(*token_a_woopool, false),
            AccountMeta::new(*user_source_token, false),
            AccountMeta::new(*a_token_vault, false),
            AccountMeta::new(*token_a_price_update, false),
            AccountMeta::new(*token_b_wooracle, false),
            AccountMeta::new(*token_b_woopool, false),
            AccountMeta::new(*user_destination_token, false),
            AccountMeta::new(*b_token_vault, false),
            AccountMeta::new(*token_b_price_update, false),
            AccountMeta::new(*quote_pool, false),
            AccountMeta::new(*quote_price_update, false),
            AccountMeta::new(*quote_token_vault, false),
            AccountMeta::new(*rebate_to, false),
        ],
        data: create_instruction_data(amount),
    }
}