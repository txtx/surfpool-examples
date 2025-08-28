use crate::prelude::*;
use crate::constants::{
    RAYDIUM_SWAP_PROGRAM, 
    RAYDIUM_STABLE_PROGRAM, 
    RAYDIUM_CLMM_PROGRAM, 
    RAYDIUM_CPMM_PROGRAM,
    SWAP_SELECTOR, 
    SWAP_V2_SELECTOR, 
    CPSWAP_SELECTOR,
    ZERO_ADDRESS
};

// Raydium Standard Swap Implementation
fn create_raydium_swap_instruction_data(amount_in: u64) -> Vec<u8> {
    let mut buffer = Vec::with_capacity(17);
    buffer.push(9u8); // instruction discriminator
    buffer.extend_from_slice(&amount_in.to_le_bytes());
    buffer.extend_from_slice(&1u64.to_le_bytes()); // minimum amount out
    buffer
}

pub fn create_raydium_swap_ix(
    amm_id: &Pubkey,
    amm_authority: &Pubkey,
    amm_open_orders: &Pubkey,
    amm_target_orders: &Pubkey,
    pool_coin_token_account: &Pubkey,
    pool_pc_token_account: &Pubkey,
    serum_program_id: &Pubkey,
    serum_market: &Pubkey,
    serum_bids: &Pubkey,
    serum_asks: &Pubkey,
    serum_event_queue: &Pubkey,
    serum_coin_vault_account: &Pubkey,
    serum_pc_vault_account: &Pubkey,
    serum_vault_signer: &Pubkey,
    user_source_token: &Pubkey,
    user_destination_token: &Pubkey,
    user: &Pubkey,
    amount: u64,
) -> Instruction {
    tracing::debug!("Creating Raydium swap instruction with the following details:");
    tracing::debug!("  Program ID: {}", RAYDIUM_SWAP_PROGRAM);
    tracing::debug!("  Accounts:");
    tracing::debug!("    1. token_program: {} (readonly)", spl_token::id());
    tracing::debug!("    2. amm_id: {} (mutable)", amm_id);
    tracing::debug!("    3. amm_authority: {} (readonly)", amm_authority);
    tracing::debug!("    4. amm_open_orders: {} (mutable)", amm_open_orders);
    tracing::debug!("    5. amm_target_orders: {} (mutable)", amm_target_orders);
    tracing::debug!("    6. pool_coin_token_account: {} (mutable)", pool_coin_token_account);
    tracing::debug!("    7. pool_pc_token_account: {} (mutable)", pool_pc_token_account);
    tracing::debug!("    8. serum_program_id: {} (readonly)", serum_program_id);
    tracing::debug!("    9. serum_market: {} (mutable)", serum_market);
    tracing::debug!("    10. serum_bids: {} (mutable)", serum_bids);
    tracing::debug!("    11. serum_asks: {} (mutable)", serum_asks);
    tracing::debug!("    12. serum_event_queue: {} (mutable)", serum_event_queue);
    tracing::debug!("    13. serum_coin_vault_account: {} (mutable)", serum_coin_vault_account);
    tracing::debug!("    14. serum_pc_vault_account: {} (mutable)", serum_pc_vault_account);
    tracing::debug!("    15. serum_vault_signer: {} (readonly)", serum_vault_signer);
    tracing::debug!("    16. user_source_token: {} (mutable)", user_source_token);
    tracing::debug!("    17. user_destination_token: {} (mutable)", user_destination_token);
    tracing::debug!("    18. user: {} (signer)", user);

    tracing::debug!("  Args:");
    tracing::debug!("    amount: {}", amount);

    Instruction {
        program_id: RAYDIUM_SWAP_PROGRAM,
        accounts: vec![
            AccountMeta::new_readonly(spl_token::id(), false),
            AccountMeta::new(*amm_id, false),
            AccountMeta::new_readonly(*amm_authority, false),
            AccountMeta::new(*amm_open_orders, false),
            AccountMeta::new(*amm_target_orders, false),
            AccountMeta::new(*pool_coin_token_account, false),
            AccountMeta::new(*pool_pc_token_account, false),
            AccountMeta::new_readonly(*serum_program_id, false),
            AccountMeta::new(*serum_market, false),
            AccountMeta::new(*serum_bids, false),
            AccountMeta::new(*serum_asks, false),
            AccountMeta::new(*serum_event_queue, false),
            AccountMeta::new(*serum_coin_vault_account, false),
            AccountMeta::new(*serum_pc_vault_account, false),
            AccountMeta::new_readonly(*serum_vault_signer, false),
            AccountMeta::new(*user_source_token, false),
            AccountMeta::new(*user_destination_token, false),
            AccountMeta::new_readonly(*user, true),
        ],
        data: create_raydium_swap_instruction_data(amount),
    }
}

// Raydium Stable Swap Implementation
pub fn create_raydium_stable_swap_ix(
    amm_id: &Pubkey,
    amm_authority: &Pubkey,
    amm_open_orders: &Pubkey,
    pool_coin_token_account: &Pubkey,
    pool_pc_token_account: &Pubkey,
    model_data_account: &Pubkey,
    serum_program_id: &Pubkey,
    serum_market: &Pubkey,
    serum_bids: &Pubkey,
    serum_asks: &Pubkey,
    serum_event_queue: &Pubkey,
    serum_coin_vault_account: &Pubkey,
    serum_pc_vault_account: &Pubkey,
    serum_vault_signer: &Pubkey,
    user_source_token: &Pubkey,
    user_destination_token: &Pubkey,
    user: &Pubkey,
    amount: u64,
) -> Instruction {
    tracing::debug!("Creating Raydium stable swap instruction with the following details:");
    tracing::debug!("  Program ID: {}", RAYDIUM_STABLE_PROGRAM);
    tracing::debug!("  Accounts:");
    tracing::debug!("    1. token_program: {} (readonly)", spl_token::id());
    tracing::debug!("    2. amm_id: {} (mutable)", amm_id);
    tracing::debug!("    3. amm_authority: {} (readonly)", amm_authority);
    tracing::debug!("    4. amm_open_orders: {} (mutable)", amm_open_orders);
    tracing::debug!("    5. pool_coin_token_account: {} (mutable)", pool_coin_token_account);
    tracing::debug!("    6. pool_pc_token_account: {} (mutable)", pool_pc_token_account);
    tracing::debug!("    7. model_data_account: {} (mutable)", model_data_account);
    tracing::debug!("    8-18. serum accounts and user accounts...");

    tracing::debug!("  Args:");
    tracing::debug!("    amount: {}", amount);

    Instruction {
        program_id: RAYDIUM_STABLE_PROGRAM,
        accounts: vec![
            AccountMeta::new_readonly(spl_token::id(), false),
            AccountMeta::new(*amm_id, false),
            AccountMeta::new_readonly(*amm_authority, false),
            AccountMeta::new(*amm_open_orders, false),
            AccountMeta::new(*pool_coin_token_account, false),
            AccountMeta::new(*pool_pc_token_account, false),
            AccountMeta::new(*model_data_account, false),
            AccountMeta::new_readonly(*serum_program_id, false),
            AccountMeta::new(*serum_market, false),
            AccountMeta::new(*serum_bids, false),
            AccountMeta::new(*serum_asks, false),
            AccountMeta::new(*serum_event_queue, false),
            AccountMeta::new(*serum_coin_vault_account, false),
            AccountMeta::new(*serum_pc_vault_account, false),
            AccountMeta::new_readonly(*serum_vault_signer, false),
            AccountMeta::new(*user_source_token, false),
            AccountMeta::new(*user_destination_token, false),
            AccountMeta::new_readonly(*user, true),
        ],
        data: create_raydium_swap_instruction_data(amount),
    }
}

// Raydium CLMM Implementation
fn create_raydium_clmm_instruction_data(amount_in: u64, is_base_input: bool) -> Vec<u8> {
    let other_amount_threshold = 1u64;
    let sqrt_price_limit_x64 = 0u128;
    
    let mut buffer = Vec::with_capacity(41);
    buffer.extend_from_slice(&SWAP_SELECTOR);
    buffer.extend_from_slice(&amount_in.to_le_bytes());
    buffer.extend_from_slice(&other_amount_threshold.to_le_bytes());
    buffer.extend_from_slice(&sqrt_price_limit_x64.to_le_bytes());
    buffer.extend_from_slice(&(is_base_input as u8).to_le_bytes());
    buffer
}

pub fn create_raydium_clmm_swap_ix(
    amm_config_id: &Pubkey,
    pool_id: &Pubkey,
    user_source_token: &Pubkey,
    user_destination_token: &Pubkey,
    input_vault: &Pubkey,
    output_vault: &Pubkey,
    observation_id: &Pubkey,
    tick_array0: &Pubkey,
    ex_bitmap: &Pubkey,
    tick_array1: Option<&Pubkey>,
    tick_array2: Option<&Pubkey>,
    user: &Pubkey,
    amount: u64,
    is_base_input: bool,
) -> Instruction {
    tracing::debug!("Creating Raydium CLMM swap instruction with the following details:");
    tracing::debug!("  Program ID: {}", RAYDIUM_CLMM_PROGRAM);
    tracing::debug!("  Accounts:");
    tracing::debug!("    1. user: {} (signer)", user);
    tracing::debug!("    2. amm_config_id: {} (readonly)", amm_config_id);
    tracing::debug!("    3. pool_id: {} (mutable)", pool_id);
    tracing::debug!("    4. user_source_token: {} (mutable)", user_source_token);
    tracing::debug!("    5. user_destination_token: {} (mutable)", user_destination_token);
    tracing::debug!("    6. input_vault: {} (mutable)", input_vault);
    tracing::debug!("    7. output_vault: {} (mutable)", output_vault);
    tracing::debug!("    8. observation_id: {} (mutable)", observation_id);
    tracing::debug!("    9. token_program: {} (readonly)", spl_token::id());
    tracing::debug!("    10. tick_array0: {} (mutable)", tick_array0);
    tracing::debug!("    11. ex_bitmap: {} (mutable)", ex_bitmap);
    if let Some(ta1) = tick_array1 {
        tracing::debug!("    12. tick_array1: {} (mutable)", ta1);
    }
    if let Some(ta2) = tick_array2 {
        tracing::debug!("    13. tick_array2: {} (mutable)", ta2);
    }

    tracing::debug!("  Args:");
    tracing::debug!("    amount: {}", amount);
    tracing::debug!("    is_base_input: {}", is_base_input);

    let mut accounts = vec![
        AccountMeta::new(*user, true),
        AccountMeta::new_readonly(*amm_config_id, false),
        AccountMeta::new(*pool_id, false),
        AccountMeta::new(*user_source_token, false),
        AccountMeta::new(*user_destination_token, false),
        AccountMeta::new(*input_vault, false),
        AccountMeta::new(*output_vault, false),
        AccountMeta::new(*observation_id, false),
        AccountMeta::new_readonly(spl_token::id(), false),
        AccountMeta::new(*tick_array0, false),
        AccountMeta::new(*ex_bitmap, false),
    ];

    if let Some(tick_array1_key) = tick_array1 {
        if tick_array1_key != &ZERO_ADDRESS {
            accounts.push(AccountMeta::new(*tick_array1_key, false));
        }
    }
    
    if let Some(tick_array2_key) = tick_array2 {
        if tick_array2_key != &ZERO_ADDRESS {
            accounts.push(AccountMeta::new(*tick_array2_key, false));
        }
    }

    Instruction {
        program_id: RAYDIUM_CLMM_PROGRAM,
        accounts,
        data: create_raydium_clmm_instruction_data(amount, is_base_input),
    }
}

// Raydium CLMM V2 Implementation
fn create_raydium_clmm_v2_instruction_data(amount_in: u64, is_base_input: bool) -> Vec<u8> {
    let other_amount_threshold = 1u64;
    let sqrt_price_limit_x64 = 0u128;
    
    let mut buffer = Vec::with_capacity(41);
    buffer.extend_from_slice(&SWAP_V2_SELECTOR);
    buffer.extend_from_slice(&amount_in.to_le_bytes());
    buffer.extend_from_slice(&other_amount_threshold.to_le_bytes());
    buffer.extend_from_slice(&sqrt_price_limit_x64.to_le_bytes());
    buffer.extend_from_slice(&(is_base_input as u8).to_le_bytes());
    buffer
}

pub fn create_raydium_clmm_v2_swap_ix(
    amm_config_id: &Pubkey,
    pool_id: &Pubkey,
    user_source_token: &Pubkey,
    user_destination_token: &Pubkey,
    input_vault: &Pubkey,
    output_vault: &Pubkey,
    observation_id: &Pubkey,
    token_program_2022: &Pubkey,
    memo_program: &Pubkey,
    input_vault_mint: &Pubkey,
    output_vault_mint: &Pubkey,
    ex_bitmap: &Pubkey,
    tick_array0: &Pubkey,
    tick_array1: Option<&Pubkey>,
    tick_array2: Option<&Pubkey>,
    user: &Pubkey,
    amount: u64,
    is_base_input: bool,
) -> Instruction {
    tracing::debug!("Creating Raydium CLMM V2 swap instruction with the following details:");
    tracing::debug!("  Program ID: {}", RAYDIUM_CLMM_PROGRAM);
    tracing::debug!("  Accounts:");
    tracing::debug!("    1. user: {} (signer)", user);
    tracing::debug!("    2. amm_config_id: {} (readonly)", amm_config_id);
    tracing::debug!("    3. pool_id: {} (mutable)", pool_id);
    tracing::debug!("    4-15. various vault and program accounts...");

    tracing::debug!("  Args:");
    tracing::debug!("    amount: {}", amount);
    tracing::debug!("    is_base_input: {}", is_base_input);

    let mut accounts = vec![
        AccountMeta::new(*user, true),
        AccountMeta::new_readonly(*amm_config_id, false),
        AccountMeta::new(*pool_id, false),
        AccountMeta::new(*user_source_token, false),
        AccountMeta::new(*user_destination_token, false),
        AccountMeta::new(*input_vault, false),
        AccountMeta::new(*output_vault, false),
        AccountMeta::new(*observation_id, false),
        AccountMeta::new_readonly(spl_token::id(), false),
        AccountMeta::new_readonly(*token_program_2022, false),
        AccountMeta::new_readonly(*memo_program, false),
        AccountMeta::new_readonly(*input_vault_mint, false),
        AccountMeta::new_readonly(*output_vault_mint, false),
        AccountMeta::new(*ex_bitmap, false),
        AccountMeta::new(*tick_array0, false),
    ];

    if let Some(tick_array1_key) = tick_array1 {
        if tick_array1_key != &ZERO_ADDRESS {
            accounts.push(AccountMeta::new(*tick_array1_key, false));
        }
    }
    
    if let Some(tick_array2_key) = tick_array2 {
        if tick_array2_key != &ZERO_ADDRESS {
            accounts.push(AccountMeta::new(*tick_array2_key, false));
        }
    }

    Instruction {
        program_id: RAYDIUM_CLMM_PROGRAM,
        accounts,
        data: create_raydium_clmm_v2_instruction_data(amount, is_base_input),
    }
}

// Raydium CPMM Implementation
fn create_raydium_cpmm_instruction_data(amount_in: u64) -> Vec<u8> {
    let minimum_amount_out = 0u64;
    
    let mut buffer = Vec::with_capacity(24);
    buffer.extend_from_slice(&CPSWAP_SELECTOR);
    buffer.extend_from_slice(&amount_in.to_le_bytes());
    buffer.extend_from_slice(&minimum_amount_out.to_le_bytes());
    buffer
}

pub fn create_raydium_cpmm_swap_ix(
    authority: &Pubkey,
    amm_config: &Pubkey,
    pool_state: &Pubkey,
    user_source_token: &Pubkey,
    user_destination_token: &Pubkey,
    input_vault: &Pubkey,
    output_vault: &Pubkey,
    input_token_program: &Pubkey,
    output_token_program: &Pubkey,
    input_token_mint: &Pubkey,
    output_token_mint: &Pubkey,
    observation_state: &Pubkey,
    user: &Pubkey,
    amount: u64,
) -> Instruction {
    tracing::debug!("Creating Raydium CPMM swap instruction with the following details:");
    tracing::debug!("  Program ID: {}", RAYDIUM_CPMM_PROGRAM);
    tracing::debug!("  Accounts:");
    tracing::debug!("    1. user: {} (signer)", user);
    tracing::debug!("    2. authority: {} (readonly)", authority);
    tracing::debug!("    3. amm_config: {} (readonly)", amm_config);
    tracing::debug!("    4. pool_state: {} (mutable)", pool_state);
    tracing::debug!("    5. user_source_token: {} (mutable)", user_source_token);
    tracing::debug!("    6. user_destination_token: {} (mutable)", user_destination_token);
    tracing::debug!("    7. input_vault: {} (mutable)", input_vault);
    tracing::debug!("    8. output_vault: {} (mutable)", output_vault);
    tracing::debug!("    9. input_token_program: {} (readonly)", input_token_program);
    tracing::debug!("    10. output_token_program: {} (readonly)", output_token_program);
    tracing::debug!("    11. input_token_mint: {} (readonly)", input_token_mint);
    tracing::debug!("    12. output_token_mint: {} (readonly)", output_token_mint);
    tracing::debug!("    13. observation_state: {} (mutable)", observation_state);

    tracing::debug!("  Args:");
    tracing::debug!("    amount: {}", amount);

    Instruction {
        program_id: RAYDIUM_CPMM_PROGRAM,
        accounts: vec![
            AccountMeta::new(*user, true),
            AccountMeta::new_readonly(*authority, false),
            AccountMeta::new_readonly(*amm_config, false),
            AccountMeta::new(*pool_state, false),
            AccountMeta::new(*user_source_token, false),
            AccountMeta::new(*user_destination_token, false),
            AccountMeta::new(*input_vault, false),
            AccountMeta::new(*output_vault, false),
            AccountMeta::new_readonly(*input_token_program, false),
            AccountMeta::new_readonly(*output_token_program, false),
            AccountMeta::new_readonly(*input_token_mint, false),
            AccountMeta::new_readonly(*output_token_mint, false),
            AccountMeta::new(*observation_state, false),
        ],
        data: create_raydium_cpmm_instruction_data(amount),
    }
}

// Validation helper functions
pub fn validate_raydium_swap_program_id(program_id: &Pubkey) -> Result<(), &'static str> {
    if program_id != &RAYDIUM_SWAP_PROGRAM {
        return Err("Invalid Raydium Swap program ID");
    }
    Ok(())
}

pub fn validate_raydium_stable_program_id(program_id: &Pubkey) -> Result<(), &'static str> {
    if program_id != &RAYDIUM_STABLE_PROGRAM {
        return Err("Invalid Raydium Stable program ID");
    }
    Ok(())
}

pub fn validate_raydium_clmm_program_id(program_id: &Pubkey) -> Result<(), &'static str> {
    if program_id != &RAYDIUM_CLMM_PROGRAM {
        return Err("Invalid Raydium CLMM program ID");
    }
    Ok(())
}

pub fn validate_raydium_cpmm_program_id(program_id: &Pubkey) -> Result<(), &'static str> {
    if program_id != &RAYDIUM_CPMM_PROGRAM {
        return Err("Invalid Raydium CPMM program ID");
    }
    Ok(())
}

// Helper function to check if tick array should be included
pub fn should_include_tick_array(tick_array: &Pubkey) -> bool {
    tick_array != &ZERO_ADDRESS
}