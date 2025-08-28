use crate::prelude::*;
use crate::constants::{RAYDIUM_LAUNCHPAD_PROGRAM, LETSBONK_PLATFORM_CONFIG};
use crate::{RAYDIUM_LAUNCHPAD_BUY_SELECTOR, RAYDIUM_LAUNCHPAD_SELL_SELECTOR};

fn create_instruction_data(is_buy: bool, amount_in: u64) -> Vec<u8> {
    let mut buffer = Vec::with_capacity(32);
    
    if is_buy {
        buffer.extend_from_slice(RAYDIUM_LAUNCHPAD_BUY_SELECTOR);
    } else {
        buffer.extend_from_slice(RAYDIUM_LAUNCHPAD_SELL_SELECTOR);
    }
    
    buffer.extend_from_slice(&amount_in.to_le_bytes());
    buffer.extend_from_slice(&1u64.to_le_bytes()); // minimum_amount_out
    buffer.extend_from_slice(&0u64.to_le_bytes()); // share_fee_rate
    buffer
}

pub fn create_raydium_launchpad_swap_ix(
    swap_authority: &Pubkey,
    launchpad_authority: &Pubkey,
    global_config: &Pubkey,
    platform_config: &Pubkey,
    pool_state: &Pubkey,
    base_vault: &Pubkey,
    quote_vault: &Pubkey,
    base_mint: &Pubkey,
    quote_mint: &Pubkey,
    user_source_token: &Pubkey,
    user_destination_token: &Pubkey,
    base_token_program: &Pubkey,
    quote_token_program: &Pubkey,
    event_authority: &Pubkey,
    amount: u64,
    source_mint: &Pubkey,
) -> Instruction {
    // Determine platform name for logging
    let platform_name = if platform_config == &LETSBONK_PLATFORM_CONFIG {
        "LetsBonkFun"
    } else {
        "RaydiumLaunchpad"
    };
    
    // Print detailed debug information about the instruction we're creating
    tracing::debug!("Creating {} swap instruction with the following details:", platform_name);
    tracing::debug!("  Program ID: {}", RAYDIUM_LAUNCHPAD_PROGRAM);
    tracing::debug!("  Accounts:");
    tracing::debug!("    1. swap_authority: {} (signer)", swap_authority);
    tracing::debug!("    2. launchpad_authority: {} (readonly)", launchpad_authority);
    tracing::debug!("    3. global_config: {} (readonly)", global_config);
    tracing::debug!("    4. platform_config: {} (readonly)", platform_config);
    tracing::debug!("    5. pool_state: {} (mutable)", pool_state);
    tracing::debug!("    6. base_token: {} (mutable)", user_source_token);
    tracing::debug!("    7. quote_token: {} (mutable)", user_destination_token);
    tracing::debug!("    8. base_vault: {} (mutable)", base_vault);
    tracing::debug!("    9. quote_vault: {} (mutable)", quote_vault);
    tracing::debug!("    10. base_mint: {} (readonly)", base_mint);
    tracing::debug!("    11. quote_mint: {} (readonly)", quote_mint);
    tracing::debug!("    12. base_token_program: {} (readonly)", base_token_program);
    tracing::debug!("    13. quote_token_program: {} (readonly)", quote_token_program);
    tracing::debug!("    14. event_authority: {} (readonly)", event_authority);
    tracing::debug!("    15. program_id: {} (readonly)", RAYDIUM_LAUNCHPAD_PROGRAM);
    
    // Determine if this is a buy or sell operation
    let is_buy = source_mint == quote_mint;
    
    tracing::debug!("  Args:");
    tracing::debug!("    amount: {}", amount);
    tracing::debug!("    operation: {}", if is_buy { "BUY" } else { "SELL" });
    
    Instruction {
        program_id: RAYDIUM_LAUNCHPAD_PROGRAM,
        accounts: vec![
            AccountMeta::new_readonly(*swap_authority, true),
            AccountMeta::new_readonly(*launchpad_authority, false),
            AccountMeta::new_readonly(*global_config, false),
            AccountMeta::new_readonly(*platform_config, false),
            AccountMeta::new(*pool_state, false),
            AccountMeta::new(*user_source_token, false),
            AccountMeta::new(*user_destination_token, false),
            AccountMeta::new(*base_vault, false),
            AccountMeta::new(*quote_vault, false),
            AccountMeta::new_readonly(*base_mint, false),
            AccountMeta::new_readonly(*quote_mint, false),
            AccountMeta::new_readonly(*base_token_program, false),
            AccountMeta::new_readonly(*quote_token_program, false),
            AccountMeta::new_readonly(*event_authority, false),
            AccountMeta::new_readonly(RAYDIUM_LAUNCHPAD_PROGRAM, false),
        ],
        data: create_instruction_data(is_buy, amount),
    }
}