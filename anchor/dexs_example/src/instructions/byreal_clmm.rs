use crate::prelude::*;
use crate::constants::BYREAL_CLMM_PROGRAM;

fn create_swap_v2_data(amount_in: u64, other_amount_threshold: u64, sqrt_price_limit_x64: u128, is_base_input: u8) -> Vec<u8> {
    let mut buffer = Vec::with_capacity(41);
    // SWAP_V2_SELECTOR (8 bytes)
    buffer.extend_from_slice(&[0xd0, 0x49, 0x3c, 0x8a, 0xb4, 0x5e, 0x7d, 0x2f]); // Example selector
    buffer.extend_from_slice(&amount_in.to_le_bytes());
    buffer.extend_from_slice(&other_amount_threshold.to_le_bytes());
    buffer.extend_from_slice(&sqrt_price_limit_x64.to_le_bytes());
    buffer.push(is_base_input);
    buffer
}

pub fn create_byreal_clmm_swap_v2_ix(
    payer: &Pubkey,
    amm_config: &Pubkey,
    pool_state: &Pubkey,
    input_token_account: &Pubkey,
    output_token_account: &Pubkey,
    input_vault: &Pubkey,
    output_vault: &Pubkey,
    observation_state: &Pubkey,
    input_vault_mint: &Pubkey,
    output_vault_mint: &Pubkey,
    tickarray_bitmap_extension: &Pubkey,
    tick_arrays: &[&Pubkey], // Variable number of tick arrays
    amount_in: u64,
) -> Instruction {
    tracing::debug!("Creating Byreal CLMM SwapV2 instruction with the following details:");
    tracing::debug!("  Program ID: {}", BYREAL_CLMM_PROGRAM);
    tracing::debug!("  Accounts:");
    tracing::debug!("    1. payer: {} (signer)", payer);
    tracing::debug!("    2. amm_config: {} (readonly)", amm_config);
    tracing::debug!("    3. pool_state: {} (mutable)", pool_state);
    tracing::debug!("    4. input_token_account: {} (mutable)", input_token_account);
    tracing::debug!("    5. output_token_account: {} (mutable)", output_token_account);
    tracing::debug!("    6. input_vault: {} (mutable)", input_vault);
    tracing::debug!("    7. output_vault: {} (mutable)", output_vault);
    tracing::debug!("    8. observation_state: {} (mutable)", observation_state);
    tracing::debug!("    9. token_program: {} (readonly)", spl_token::id());
    tracing::debug!("    10. token_program_2022: {} (readonly)", spl_token_2022::id());
    tracing::debug!("    11. memo_program: {} (readonly)", spl_memo::id());
    tracing::debug!("    12. input_vault_mint: {} (readonly)", input_vault_mint);
    tracing::debug!("    13. output_vault_mint: {} (readonly)", output_vault_mint);
    tracing::debug!("    14. tickarray_bitmap_extension: {} (mutable)", tickarray_bitmap_extension);
    
    let mut accounts = vec![
        AccountMeta::new(*payer, true),
        AccountMeta::new_readonly(*amm_config, false),
        AccountMeta::new(*pool_state, false),
        AccountMeta::new(*input_token_account, false),
        AccountMeta::new(*output_token_account, false),
        AccountMeta::new(*input_vault, false),
        AccountMeta::new(*output_vault, false),
        AccountMeta::new(*observation_state, false),
        AccountMeta::new_readonly(spl_token::id(), false),
        AccountMeta::new_readonly(spl_token_2022::id(), false),
        AccountMeta::new_readonly(spl_memo::id(), false),
        AccountMeta::new_readonly(*input_vault_mint, false),
        AccountMeta::new_readonly(*output_vault_mint, false),
        AccountMeta::new(*tickarray_bitmap_extension, false),
    ];
    
    // Add tick arrays dynamically (only non-program ID tick arrays)
    for (i, tick_array) in tick_arrays.iter().enumerate() {
        if **tick_array != BYREAL_CLMM_PROGRAM {
            accounts.push(AccountMeta::new(**tick_array, false));
            tracing::debug!("    {}. tick_array{}: {} (mutable)", 15 + i, i, tick_array);
        }
    }
    
    tracing::debug!("  Args:");
    tracing::debug!("    amount_in: {}", amount_in);
    tracing::debug!("    other_amount_threshold: 1");
    tracing::debug!("    sqrt_price_limit_x64: 0");
    tracing::debug!("    is_base_input: 1");
    
    Instruction {
        program_id: BYREAL_CLMM_PROGRAM,
        accounts,
        data: create_swap_v2_data(amount_in, 1, 0, 1),
    }
}

