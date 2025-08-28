use crate::prelude::*;
use crate::constants::{PANCAKESWAPV3_PROGRAM};

fn create_instruction_data(amount_in: u64, other_amount_threshold: u64, sqrt_price_limit_x64: u128, is_base_input: bool) -> Vec<u8> {
    let mut buffer = Vec::with_capacity(41);
    buffer.extend_from_slice(&[0x24, 0x8c, 0xc7, 0x8d]); // SWAP_SELECTOR
    buffer.extend_from_slice(&amount_in.to_le_bytes());
    buffer.extend_from_slice(&other_amount_threshold.to_le_bytes());
    buffer.extend_from_slice(&sqrt_price_limit_x64.to_le_bytes());
    buffer.extend_from_slice(&(is_base_input as u8).to_le_bytes());
    buffer
}

fn create_instruction_data_v2(amount_in: u64, other_amount_threshold: u64, sqrt_price_limit_x64: u128, is_base_input: bool) -> Vec<u8> {
    let mut buffer = Vec::with_capacity(41);
    buffer.extend_from_slice(&[0x24, 0x8c, 0x7d, 0x8d]); // SWAP_V2_SELECTOR (different selector)
    buffer.extend_from_slice(&amount_in.to_le_bytes());
    buffer.extend_from_slice(&other_amount_threshold.to_le_bytes());
    buffer.extend_from_slice(&sqrt_price_limit_x64.to_le_bytes());
    buffer.extend_from_slice(&(is_base_input as u8).to_le_bytes());
    buffer
}

pub fn create_pancakeswap_v3_swap_ix(
    pool_state: &Pubkey,
    user: &Pubkey,
    from: &Pubkey,
    to: &Pubkey,
    amount: u64,
    amm_config: &Pubkey,
    input_vault: &Pubkey,
    output_vault: &Pubkey,
    observation_state: &Pubkey,
    tick_array0: &Pubkey,
    ex_bitmap: &Pubkey,
    tick_array1: Option<&Pubkey>,
    tick_array2: Option<&Pubkey>,
) -> Instruction {
    // Print detailed debug information about the instruction we're creating
    tracing::debug!("Creating PancakeSwap V3 swap instruction with the following details:");
    tracing::debug!("  Program ID: {}", PANCAKESWAPV3_PROGRAM);
    tracing::debug!("  Accounts:");
    tracing::debug!("    1. user: {} (signer)", user);
    tracing::debug!("    2. amm_config: {} (readonly)", amm_config);
    tracing::debug!("    3. pool_state: {} (mutable)", pool_state);
    
    let user_source_token = get_associated_token_address(user, from);
    let user_destination_token = get_associated_token_address(user, to);
    tracing::debug!("    4. user_source_token: {} (mutable)", user_source_token);
    tracing::debug!("    5. user_destination_token: {} (mutable)", user_destination_token);
    tracing::debug!("    6. input_vault: {} (mutable)", input_vault);
    tracing::debug!("    7. output_vault: {} (mutable)", output_vault);
    tracing::debug!("    8. observation_state: {} (mutable)", observation_state);
    tracing::debug!("    9. token_program: {} (readonly)", spl_token::id());
    tracing::debug!("    10. tick_array0: {} (mutable)", tick_array0);
    tracing::debug!("    11. ex_bitmap: {} (mutable)", ex_bitmap);
    
    if let Some(tick_array1) = tick_array1 {
        if *tick_array1 != Pubkey::default() && *tick_array1 != PANCAKESWAPV3_PROGRAM {
            tracing::debug!("    12. tick_array1: {} (mutable)", tick_array1);
        }
    }
    if let Some(tick_array2) = tick_array2 {
        if *tick_array2 != Pubkey::default() && *tick_array2 != PANCAKESWAPV3_PROGRAM {
            tracing::debug!("    13. tick_array2: {} (mutable)", tick_array2);
        }
    }

    tracing::debug!("  Args:");
    tracing::debug!("    amount: {}", amount);
    
    let is_base_input = true;
    let sqrt_price_limit_x64 = 0u128;
    let other_amount_threshold = 1u64;

    let mut accounts = vec![
        AccountMeta::new(*user, true),                          // payer
        AccountMeta::new_readonly(*amm_config, false),          // amm_config
        AccountMeta::new(*pool_state, false),                   // pool_state
        AccountMeta::new(user_source_token, false),             // user_source_token
        AccountMeta::new(user_destination_token, false),        // user_destination_token
        AccountMeta::new(*input_vault, false),                  // input_vault
        AccountMeta::new(*output_vault, false),                 // output_vault
        AccountMeta::new(*observation_state, false),            // observation_state
        AccountMeta::new_readonly(spl_token::id(), false),      // spl token
        AccountMeta::new(*tick_array0, false),                  // tick_array0
        AccountMeta::new(*ex_bitmap, false),                    // ex_bitmap
    ];

    // Add optional tick arrays if they're valid
    if let Some(tick_array1) = tick_array1 {
        if *tick_array1 != Pubkey::default() && *tick_array1 != PANCAKESWAPV3_PROGRAM {
            accounts.push(AccountMeta::new(*tick_array1, false));
        }
    }
    if let Some(tick_array2) = tick_array2 {
        if *tick_array2 != Pubkey::default() && *tick_array2 != PANCAKESWAPV3_PROGRAM {
            accounts.push(AccountMeta::new(*tick_array2, false));
        }
    }

    Instruction {
        program_id: PANCAKESWAPV3_PROGRAM,
        accounts,
        data: create_instruction_data(amount, other_amount_threshold, sqrt_price_limit_x64, is_base_input),
    }
}

pub fn create_pancakeswap_v3_swap_v2_ix(
    pool_state: &Pubkey,
    user: &Pubkey,
    from: &Pubkey,
    to: &Pubkey,
    amount: u64,
    amm_config: &Pubkey,
    input_vault: &Pubkey,
    output_vault: &Pubkey,
    observation_state: &Pubkey,
    input_vault_mint: &Pubkey,
    output_vault_mint: &Pubkey,
    ex_bitmap: &Pubkey,
    tick_array0: &Pubkey,
    tick_array1: Option<&Pubkey>,
    tick_array2: Option<&Pubkey>,
    memo_program: &Pubkey,
) -> Instruction {
    // Print detailed debug information about the instruction we're creating
    tracing::debug!("Creating PancakeSwap V3 swap v2 instruction with the following details:");
    tracing::debug!("  Program ID: {}", PANCAKESWAPV3_PROGRAM);
    tracing::debug!("  Accounts:");
    tracing::debug!("    1. user: {} (signer)", user);
    tracing::debug!("    2. amm_config: {} (readonly)", amm_config);
    tracing::debug!("    3. pool_state: {} (mutable)", pool_state);
    
    let user_source_token = get_associated_token_address(user, from);
    let user_destination_token = get_associated_token_address(user, to);
    tracing::debug!("    4. user_source_token: {} (mutable)", user_source_token);
    tracing::debug!("    5. user_destination_token: {} (mutable)", user_destination_token);
    tracing::debug!("    6. input_vault: {} (mutable)", input_vault);
    tracing::debug!("    7. output_vault: {} (mutable)", output_vault);
    tracing::debug!("    8. observation_state: {} (mutable)", observation_state);
    tracing::debug!("    9. token_program: {} (readonly)", spl_token::id());
    tracing::debug!("    10. token_program_2022: {} (readonly)", spl_token_2022::id());
    tracing::debug!("    11. memo_program: {} (readonly)", memo_program);
    tracing::debug!("    12. input_vault_mint: {} (readonly)", input_vault_mint);
    tracing::debug!("    13. output_vault_mint: {} (readonly)", output_vault_mint);
    tracing::debug!("    14. ex_bitmap: {} (mutable)", ex_bitmap);
    tracing::debug!("    15. tick_array0: {} (mutable)", tick_array0);
    
    if let Some(tick_array1) = tick_array1 {
        if *tick_array1 != Pubkey::default() && *tick_array1 != PANCAKESWAPV3_PROGRAM {
            tracing::debug!("    16. tick_array1: {} (mutable)", tick_array1);
        }
    }
    if let Some(tick_array2) = tick_array2 {
        if *tick_array2 != Pubkey::default() && *tick_array2 != PANCAKESWAPV3_PROGRAM {
            tracing::debug!("    17. tick_array2: {} (mutable)", tick_array2);
        }
    }

    tracing::debug!("  Args:");
    tracing::debug!("    amount: {}", amount);
    
    let is_base_input = true;
    let sqrt_price_limit_x64 = 0u128;
    let other_amount_threshold = 1u64;

    let mut accounts = vec![
        AccountMeta::new(*user, true),                          // payer
        AccountMeta::new_readonly(*amm_config, false),          // amm_config
        AccountMeta::new(*pool_state, false),                   // pool_state
        AccountMeta::new(user_source_token, false),             // user_source_token
        AccountMeta::new(user_destination_token, false),        // user_destination_token
        AccountMeta::new(*input_vault, false),                  // input_vault
        AccountMeta::new(*output_vault, false),                 // output_vault
        AccountMeta::new(*observation_state, false),            // observation_state
        AccountMeta::new_readonly(spl_token::id(), false),      // spl token
        AccountMeta::new_readonly(spl_token_2022::id(), false), // token 2022
        AccountMeta::new_readonly(*memo_program, false),        // memo_program
        AccountMeta::new_readonly(*input_vault_mint, false),    // input_vault_mint
        AccountMeta::new_readonly(*output_vault_mint, false),   // output_vault_mint
        AccountMeta::new(*ex_bitmap, false),                    // ex_bitmap
        AccountMeta::new(*tick_array0, false),                  // tick_array0
    ];

    // Add optional tick arrays if they're valid
    if let Some(tick_array1) = tick_array1 {
        if *tick_array1 != Pubkey::default() && *tick_array1 != PANCAKESWAPV3_PROGRAM {
            accounts.push(AccountMeta::new(*tick_array1, false));
        }
    }
    if let Some(tick_array2) = tick_array2 {
        if *tick_array2 != Pubkey::default() && *tick_array2 != PANCAKESWAPV3_PROGRAM {
            accounts.push(AccountMeta::new(*tick_array2, false));
        }
    }

    Instruction {
        program_id: PANCAKESWAPV3_PROGRAM,
        accounts,
        data: create_instruction_data_v2(amount, other_amount_threshold, sqrt_price_limit_x64, is_base_input),
    }
}