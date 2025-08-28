use crate::prelude::*;
use crate::constants::SANCTUM_PROGRAM;
use spl_token::get_associated_token_address;
use anchor_lang::solana_program::sysvar;

// Helper function to create instruction data for LST->LST swaps
fn create_lst_lst_swap_data(
    src_lst_index: u32,
    dst_lst_index: u32,
    amount_in: u64,
) -> Vec<u8> {
    let mut buffer = Vec::with_capacity(27);
    buffer.push(1); // discriminant
    buffer.push(5); // src_lst_value_calc_accs
    buffer.push(5); // dst_lst_value_calc_accs
    buffer.extend_from_slice(&src_lst_index.to_le_bytes());
    buffer.extend_from_slice(&dst_lst_index.to_le_bytes());
    buffer.extend_from_slice(&1u64.to_le_bytes()); // min_amount_out
    buffer.extend_from_slice(&amount_in.to_le_bytes());
    buffer
}

// Helper function to create instruction data for LST->WSOL swaps
fn create_lst_wsol_swap_data(
    src_lst_index: u32,
    amount_in: u64,
) -> Vec<u8> {
    let mut buffer = Vec::with_capacity(27);
    buffer.push(1); // discriminant
    buffer.push(5); // src_lst_value_calc_accs
    buffer.push(1); // dst_lst_value_calc_accs
    buffer.extend_from_slice(&src_lst_index.to_le_bytes());
    buffer.extend_from_slice(&1u32.to_le_bytes()); // dst_lst_index (WSOL = 1)
    buffer.extend_from_slice(&1u64.to_le_bytes()); // min_amount_out
    buffer.extend_from_slice(&amount_in.to_le_bytes());
    buffer
}

// Helper function to create instruction data for WSOL->LST swaps
fn create_wsol_lst_swap_data(
    dst_lst_index: u32,
    amount_in: u64,
) -> Vec<u8> {
    let mut buffer = Vec::with_capacity(27);
    buffer.push(1); // discriminant
    buffer.push(1); // src_lst_value_calc_accs
    buffer.push(5); // dst_lst_value_calc_accs
    buffer.extend_from_slice(&1u32.to_le_bytes()); // src_lst_index (WSOL = 1)
    buffer.extend_from_slice(&dst_lst_index.to_le_bytes());
    buffer.extend_from_slice(&1u64.to_le_bytes()); // min_amount_out
    buffer.extend_from_slice(&amount_in.to_le_bytes());
    buffer
}

// Helper function to get LST index from states list
fn get_lst_index_from_states(lst_mint: &Pubkey, lst_states_list: &Pubkey) -> Result<u32, Box<dyn std::error::Error>> {
    // This would normally parse the account data to find the index
    // For now, returning a placeholder - in real implementation, you'd:
    // 1. Load the account data for lst_states_list
    // 2. Parse it as LstState array
    // 3. Find the position where mint matches lst_mint
    // 4. Return that position as u32
    Ok(0) // Placeholder
}

pub fn create_sanctum_lst_lst_swap_ix(
    pool_state: &Pubkey,
    user: &Pubkey,
    source_lst_mint: &Pubkey,
    dst_lst_mint: &Pubkey,
    source_lst_account: &Pubkey,
    dst_lst_account: &Pubkey,
    protocol_fee_accumulator: &Pubkey,
    lst_states_list: &Pubkey,
    source_pool_reserves: &Pubkey,
    dst_pool_reserves: &Pubkey,
    // Source LST calculator accounts
    src_spl_sol_calculator: &Pubkey,
    src_calculator_state: &Pubkey,
    src_staked_pool_state: &Pubkey,
    src_validator_pool_program: &Pubkey,
    src_validator_pool_program_data: &Pubkey,
    // Destination LST calculator accounts
    dst_spl_sol_calculator: &Pubkey,
    dst_calculator_state: &Pubkey,
    dst_staked_pool_state: &Pubkey,
    dst_validator_pool_program: &Pubkey,
    dst_validator_pool_program_data: &Pubkey,
    // Sanctum pricing accounts
    sanctum_flat_fee_pricing: &Pubkey,
    sanctum_src_flat_fee_pricing_account: &Pubkey,
    sanctum_dst_flat_fee_pricing_account: &Pubkey,
    amount: u64,
) -> Instruction {
    tracing::debug!("Creating Sanctum LST-LST swap instruction with the following details:");
    tracing::debug!("  Program ID: {}", SANCTUM_PROGRAM);
    tracing::debug!("  Accounts:");
    tracing::debug!("    1. user: {} (signer)", user);
    tracing::debug!("    2. source_lst_mint: {} (readonly)", source_lst_mint);
    tracing::debug!("    3. dst_lst_mint: {} (readonly)", dst_lst_mint);
    tracing::debug!("    4. source_lst_account: {} (mutable)", source_lst_account);
    tracing::debug!("    5. dst_lst_account: {} (mutable)", dst_lst_account);
    tracing::debug!("    6. protocol_fee_accumulator: {} (mutable)", protocol_fee_accumulator);
    tracing::debug!("    7. source_token_program: {} (readonly)", spl_token::id());
    tracing::debug!("    8. dst_token_program: {} (readonly)", spl_token::id());
    tracing::debug!("    9. pool_state: {} (mutable)", pool_state);
    tracing::debug!("    10. lst_states_list: {} (mutable)", lst_states_list);
    tracing::debug!("    11. source_pool_reserves: {} (mutable)", source_pool_reserves);
    tracing::debug!("    12. dst_pool_reserves: {} (mutable)", dst_pool_reserves);
    tracing::debug!("    13-17. source LST calculator accounts");
    tracing::debug!("    18-22. destination LST calculator accounts");
    tracing::debug!("    23-25. sanctum pricing accounts");

    // Get LST indices (in real implementation, these would be calculated)
    let src_lst_index = get_lst_index_from_states(source_lst_mint, lst_states_list).unwrap_or(0);
    let dst_lst_index = get_lst_index_from_states(dst_lst_mint, lst_states_list).unwrap_or(1);

    tracing::debug!("  Args:");
    tracing::debug!("    amount: {}", amount);
    tracing::debug!("    src_lst_index: {}", src_lst_index);
    tracing::debug!("    dst_lst_index: {}", dst_lst_index);

    Instruction {
        program_id: SANCTUM_PROGRAM,
        accounts: vec![
            AccountMeta::new(*user, true),
            AccountMeta::new_readonly(*source_lst_mint, false),
            AccountMeta::new_readonly(*dst_lst_mint, false),
            AccountMeta::new(*source_lst_account, false),
            AccountMeta::new(*dst_lst_account, false),
            AccountMeta::new(*protocol_fee_accumulator, false),
            AccountMeta::new_readonly(spl_token::id(), false),
            AccountMeta::new_readonly(spl_token::id(), false),
            AccountMeta::new(*pool_state, false),
            AccountMeta::new(*lst_states_list, false),
            AccountMeta::new(*source_pool_reserves, false),
            AccountMeta::new(*dst_pool_reserves, false),
            AccountMeta::new_readonly(*src_spl_sol_calculator, false),
            AccountMeta::new_readonly(*src_calculator_state, false),
            AccountMeta::new_readonly(*src_staked_pool_state, false),
            AccountMeta::new_readonly(*src_validator_pool_program, false),
            AccountMeta::new_readonly(*src_validator_pool_program_data, false),
            AccountMeta::new_readonly(*dst_spl_sol_calculator, false),
            AccountMeta::new_readonly(*dst_calculator_state, false),
            AccountMeta::new_readonly(*dst_staked_pool_state, false),
            AccountMeta::new_readonly(*dst_validator_pool_program, false),
            AccountMeta::new_readonly(*dst_validator_pool_program_data, false),
            AccountMeta::new_readonly(*sanctum_flat_fee_pricing, false),
            AccountMeta::new_readonly(*sanctum_src_flat_fee_pricing_account, false),
            AccountMeta::new_readonly(*sanctum_dst_flat_fee_pricing_account, false),
        ],
        data: create_lst_lst_swap_data(src_lst_index, dst_lst_index, amount),
    }
}

pub fn create_sanctum_lst_wsol_swap_ix(
    pool_state: &Pubkey,
    user: &Pubkey,
    source_lst_mint: &Pubkey,
    source_lst_account: &Pubkey,
    dst_wsol_account: &Pubkey,
    protocol_fee_accumulator: &Pubkey,
    lst_states_list: &Pubkey,
    source_pool_reserves: &Pubkey,
    dst_pool_reserves: &Pubkey,
    spl_sol_calculator: &Pubkey,
    calculator_state: &Pubkey,
    staked_pool_state: &Pubkey,
    validator_pool_program: &Pubkey,
    validator_pool_program_data: &Pubkey,
    wsol_calculator: &Pubkey,
    sanctum_flat_fee_pricing: &Pubkey,
    sanctum_src_flat_fee_pricing_account: &Pubkey,
    sanctum_dst_flat_fee_pricing_account: &Pubkey,
    amount: u64,
) -> Instruction {
    tracing::debug!("Creating Sanctum LST-WSOL swap instruction with the following details:");
    tracing::debug!("  Program ID: {}", SANCTUM_PROGRAM);
    tracing::debug!("  Accounts:");
    tracing::debug!("    1. user: {} (signer)", user);
    tracing::debug!("    2. source_lst_mint: {} (readonly)", source_lst_mint);
    tracing::debug!("    3. dst_wsol_mint: {} (readonly)", spl_token::native_mint::id());
    tracing::debug!("    4. source_lst_account: {} (mutable)", source_lst_account);
    tracing::debug!("    5. dst_wsol_account: {} (mutable)", dst_wsol_account);
    tracing::debug!("    6. protocol_fee_accumulator: {} (mutable)", protocol_fee_accumulator);
    tracing::debug!("    7. source_token_program: {} (readonly)", spl_token::id());
    tracing::debug!("    8. dst_token_program: {} (readonly)", spl_token::id());
    tracing::debug!("    9. pool_state: {} (mutable)", pool_state);
    tracing::debug!("    10. lst_states_list: {} (mutable)", lst_states_list);
    tracing::debug!("    11. source_pool_reserves: {} (mutable)", source_pool_reserves);
    tracing::debug!("    12. dst_pool_reserves: {} (mutable)", dst_pool_reserves);
    
    let src_lst_index = get_lst_index_from_states(source_lst_mint, lst_states_list).unwrap_or(0);
    
    tracing::debug!("  Args:");
    tracing::debug!("    amount: {}", amount);
    tracing::debug!("    src_lst_index: {}", src_lst_index);

    Instruction {
        program_id: SANCTUM_PROGRAM,
        accounts: vec![
            AccountMeta::new(*user, true),
            AccountMeta::new_readonly(*source_lst_mint, false),
            AccountMeta::new_readonly(spl_token::native_mint::id(), false),
            AccountMeta::new(*source_lst_account, false),
            AccountMeta::new(*dst_wsol_account, false),
            AccountMeta::new(*protocol_fee_accumulator, false),
            AccountMeta::new_readonly(spl_token::id(), false),
            AccountMeta::new_readonly(spl_token::id(), false),
            AccountMeta::new(*pool_state, false),
            AccountMeta::new(*lst_states_list, false),
            AccountMeta::new(*source_pool_reserves, false),
            AccountMeta::new(*dst_pool_reserves, false),
            AccountMeta::new_readonly(*spl_sol_calculator, false),
            AccountMeta::new_readonly(*calculator_state, false),
            AccountMeta::new_readonly(*staked_pool_state, false),
            AccountMeta::new_readonly(*validator_pool_program, false),
            AccountMeta::new_readonly(*validator_pool_program_data, false),
            AccountMeta::new_readonly(*wsol_calculator, false),
            AccountMeta::new_readonly(*sanctum_flat_fee_pricing, false),
            AccountMeta::new_readonly(*sanctum_src_flat_fee_pricing_account, false),
            AccountMeta::new_readonly(*sanctum_dst_flat_fee_pricing_account, false),
        ],
        data: create_lst_wsol_swap_data(src_lst_index, amount),
    }
}

pub fn create_sanctum_wsol_lst_swap_ix(
    pool_state: &Pubkey,
    user: &Pubkey,
    source_wsol_account: &Pubkey,
    dst_lst_mint: &Pubkey,
    dst_lst_account: &Pubkey,
    protocol_fee_accumulator: &Pubkey,
    lst_states_list: &Pubkey,
    source_pool_reserves: &Pubkey,
    dst_pool_reserves: &Pubkey,
    wsol_calculator: &Pubkey,
    spl_sol_calculator: &Pubkey,
    calculator_state: &Pubkey,
    staked_pool_state: &Pubkey,
    validator_pool_program: &Pubkey,
    validator_pool_program_data: &Pubkey,
    sanctum_flat_fee_pricing: &Pubkey,
    sanctum_src_flat_fee_pricing_account: &Pubkey,
    sanctum_dst_flat_fee_pricing_account: &Pubkey,
    amount: u64,
) -> Instruction {
    tracing::debug!("Creating Sanctum WSOL-LST swap instruction with the following details:");
    tracing::debug!("  Program ID: {}", SANCTUM_PROGRAM);
    tracing::debug!("  Accounts:");
    tracing::debug!("    1. user: {} (signer)", user);
    tracing::debug!("    2. source_wsol_mint: {} (readonly)", spl_token::native_mint::id());
    tracing::debug!("    3. dst_lst_mint: {} (readonly)", dst_lst_mint);
    tracing::debug!("    4. source_wsol_account: {} (mutable)", source_wsol_account);
    tracing::debug!("    5. dst_lst_account: {} (mutable)", dst_lst_account);
    tracing::debug!("    6. protocol_fee_accumulator: {} (mutable)", protocol_fee_accumulator);
    
    let dst_lst_index = get_lst_index_from_states(dst_lst_mint, lst_states_list).unwrap_or(0);
    
    tracing::debug!("  Args:");
    tracing::debug!("    amount: {}", amount);
    tracing::debug!("    dst_lst_index: {}", dst_lst_index);

    Instruction {
        program_id: SANCTUM_PROGRAM,
        accounts: vec![
            AccountMeta::new(*user, true),
            AccountMeta::new_readonly(spl_token::native_mint::id(), false),
            AccountMeta::new_readonly(*dst_lst_mint, false),
            AccountMeta::new(*source_wsol_account, false),
            AccountMeta::new(*dst_lst_account, false),
            AccountMeta::new(*protocol_fee_accumulator, false),
            AccountMeta::new_readonly(spl_token::id(), false),
            AccountMeta::new_readonly(spl_token::id(), false),
            AccountMeta::new(*pool_state, false),
            AccountMeta::new(*lst_states_list, false),
            AccountMeta::new(*source_pool_reserves, false),
            AccountMeta::new(*dst_pool_reserves, false),
            AccountMeta::new_readonly(*wsol_calculator, false),
            AccountMeta::new_readonly(*spl_sol_calculator, false),
            AccountMeta::new_readonly(*calculator_state, false),
            AccountMeta::new_readonly(*staked_pool_state, false),
            AccountMeta::new_readonly(*validator_pool_program, false),
            AccountMeta::new_readonly(*validator_pool_program_data, false),
            AccountMeta::new_readonly(*sanctum_flat_fee_pricing, false),
            AccountMeta::new_readonly(*sanctum_src_flat_fee_pricing_account, false),
            AccountMeta::new_readonly(*sanctum_dst_flat_fee_pricing_account, false),
        ],
        data: create_wsol_lst_swap_data(dst_lst_index, amount),
    }
}

// Helper function to create add liquidity instruction data
fn create_add_liquidity_data(
    lst_value_calc_accs: u8,
    lst_index: u32,
    amount_in: u64,
) -> Vec<u8> {
    let mut buffer = Vec::with_capacity(22);
    buffer.push(3); // add liquidity discriminant
    buffer.push(lst_value_calc_accs);
    buffer.extend_from_slice(&lst_index.to_le_bytes());
    buffer.extend_from_slice(&amount_in.to_le_bytes());
    buffer.extend_from_slice(&1u64.to_le_bytes()); // min_lp_out
    buffer
}

pub fn create_sanctum_add_lst_liquidity_ix(
    pool_state: &Pubkey,
    user: &Pubkey,
    lst_mint: &Pubkey,
    source_lst_account: &Pubkey,
    dst_lp_account: &Pubkey,
    lp_token_mint: &Pubkey,
    protocol_fee_accumulator: &Pubkey,
    lst_states_list: &Pubkey,
    pool_reserves: &Pubkey,
    spl_sol_calculator: &Pubkey,
    calculator_state: &Pubkey,
    staked_pool_state: &Pubkey,
    validator_pool_program: &Pubkey,
    validator_pool_program_data: &Pubkey,
    sanctum_flat_fee_pricing: &Pubkey,
    amount: u64,
) -> Instruction {
    tracing::debug!("Creating Sanctum add LST liquidity instruction with the following details:");
    tracing::debug!("  Program ID: {}", SANCTUM_PROGRAM);
    tracing::debug!("  Pool State: {}", pool_state);
    tracing::debug!("  User: {} (signer)", user);
    tracing::debug!("  LST Mint: {}", lst_mint);
    tracing::debug!("  Amount: {}", amount);
    
    let lst_index = get_lst_index_from_states(lst_mint, lst_states_list).unwrap_or(0);

    Instruction {
        program_id: SANCTUM_PROGRAM,
        accounts: vec![
            AccountMeta::new(*user, true),
            AccountMeta::new_readonly(*lst_mint, false),
            AccountMeta::new(*source_lst_account, false),
            AccountMeta::new(*dst_lp_account, false),
            AccountMeta::new(*lp_token_mint, false),
            AccountMeta::new(*protocol_fee_accumulator, false),
            AccountMeta::new_readonly(spl_token::id(), false),
            AccountMeta::new_readonly(spl_token::id(), false),
            AccountMeta::new(*pool_state, false),
            AccountMeta::new(*lst_states_list, false),
            AccountMeta::new(*pool_reserves, false),
            AccountMeta::new_readonly(*spl_sol_calculator, false),
            AccountMeta::new_readonly(*calculator_state, false),
            AccountMeta::new_readonly(*staked_pool_state, false),
            AccountMeta::new_readonly(*validator_pool_program, false),
            AccountMeta::new_readonly(*validator_pool_program_data, false),
            AccountMeta::new_readonly(*sanctum_flat_fee_pricing, false),
        ],
        data: create_add_liquidity_data(5, lst_index, amount), // 5 = LST value calc accounts
    }
}

pub fn create_sanctum_add_wsol_liquidity_ix(
    pool_state: &Pubkey,
    user: &Pubkey,
    wsol_account: &Pubkey,
    dst_lp_account: &Pubkey,
    lp_token_mint: &Pubkey,
    protocol_fee_accumulator: &Pubkey,
    lst_states_list: &Pubkey,
    pool_reserves: &Pubkey,
    wsol_calculator: &Pubkey,
    sanctum_flat_fee_pricing: &Pubkey,
    amount: u64,
) -> Instruction {
    tracing::debug!("Creating Sanctum add WSOL liquidity instruction with the following details:");
    tracing::debug!("  Program ID: {}", SANCTUM_PROGRAM);
    tracing::debug!("  Pool State: {}", pool_state);
    tracing::debug!("  User: {} (signer)", user);
    tracing::debug!("  WSOL Account: {}", wsol_account);
    tracing::debug!("  Amount: {}", amount);

    Instruction {
        program_id: SANCTUM_PROGRAM,
        accounts: vec![
            AccountMeta::new(*user, true),
            AccountMeta::new_readonly(spl_token::native_mint::id(), false),
            AccountMeta::new(*wsol_account, false),
            AccountMeta::new(*dst_lp_account, false),
            AccountMeta::new(*lp_token_mint, false),
            AccountMeta::new(*protocol_fee_accumulator, false),
            AccountMeta::new_readonly(spl_token::id(), false),
            AccountMeta::new_readonly(spl_token::id(), false),
            AccountMeta::new(*pool_state, false),
            AccountMeta::new(*lst_states_list, false),
            AccountMeta::new(*pool_reserves, false),
            AccountMeta::new_readonly(*wsol_calculator, false),
            AccountMeta::new_readonly(*sanctum_flat_fee_pricing, false),
        ],
        data: create_add_liquidity_data(1, 1, amount), // 1 = WSOL value calc accounts, 1 = WSOL index
    }
}

        // Dex::SanctumAddLiq => sanctum::add_liquidity_handler,
        // Dex::SanctumRemoveLiq => sanctum::remove_liquidity_handler,
        // Dex::SanctumNonWsolSwap => sanctum::swap_without_wsol_handler,
        // Dex::SanctumWsolSwap => sanctum::swap_with_wsol_handler,