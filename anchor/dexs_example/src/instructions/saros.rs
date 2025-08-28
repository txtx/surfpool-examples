use crate::prelude::*;
use crate::constants::{SAROS_PROGRAM, SAROS_DLMM_PROGRAM, SWAP_SELECTOR};

// Saros Regular Swap Implementation
fn create_saros_instruction_data(amount_in: u64) -> Vec<u8> {
    let mut buffer = Vec::with_capacity(17);
    buffer.push(1u8); // instruction: 1 = swap
    buffer.extend_from_slice(&amount_in.to_le_bytes()); // amountIn
    buffer.extend_from_slice(&1u64.to_le_bytes()); // minimumAmountOut = 1
    buffer
}

pub fn create_saros_swap_ix(
    pool: &Pubkey,
    pool_authority: &Pubkey,
    user: &Pubkey,
    user_source_token: &Pubkey,
    user_destination_token: &Pubkey,
    pool_token_in: &Pubkey,
    pool_token_out: &Pubkey,
    pool_lp_token_mint: &Pubkey,
    protocol_lp_token: &Pubkey,
    amount: u64,
) -> Instruction {
    // Print detailed debug information about the instruction we're creating
    tracing::debug!("Creating Saros swap instruction with the following details:");
    tracing::debug!("  Program ID: {}", SAROS_PROGRAM);
    tracing::debug!("  Accounts:");
    tracing::debug!("    1. pool: {} (readonly)", pool);
    tracing::debug!("    2. pool_authority: {} (readonly)", pool_authority);
    tracing::debug!("    3. user: {} (signer)", user);
    tracing::debug!("    4. user_source_token: {} (mutable)", user_source_token);
    tracing::debug!("    5. pool_token_in: {} (mutable)", pool_token_in);
    tracing::debug!("    6. pool_token_out: {} (mutable)", pool_token_out);
    tracing::debug!("    7. user_destination_token: {} (mutable)", user_destination_token);
    tracing::debug!("    8. pool_lp_token_mint: {} (mutable)", pool_lp_token_mint);
    tracing::debug!("    9. protocol_lp_token: {} (mutable)", protocol_lp_token);
    tracing::debug!("    10. token_program: {} (readonly)", spl_token::id());

    tracing::debug!("  Args:");
    tracing::debug!("    amount: {}", amount);

    Instruction {
        program_id: SAROS_PROGRAM,
        accounts: vec![
            AccountMeta::new_readonly(*pool, false),
            AccountMeta::new_readonly(*pool_authority, false),
            AccountMeta::new_readonly(*user, true),
            AccountMeta::new(*user_source_token, false),
            AccountMeta::new(*pool_token_in, false),
            AccountMeta::new(*pool_token_out, false),
            AccountMeta::new(*user_destination_token, false),
            AccountMeta::new(*pool_lp_token_mint, false),
            AccountMeta::new(*protocol_lp_token, false),
            AccountMeta::new_readonly(spl_token::id(), false),
        ],
        data: create_saros_instruction_data(amount),
    }
}

// Saros DLMM Implementation
fn create_saros_dlmm_instruction_data(amount_in: u64, direction: bool) -> Vec<u8> {
    let mut buffer = Vec::with_capacity(26);
    buffer.extend_from_slice(&SWAP_SELECTOR);
    buffer.extend_from_slice(&amount_in.to_le_bytes());
    buffer.extend_from_slice(&1u64.to_le_bytes()); // minimumAmountOut = 1
    buffer.extend_from_slice(&(direction as u8).to_le_bytes()); // swap direction
    buffer.extend_from_slice(&0u8.to_le_bytes()); // EXACT IN
    buffer
}

pub fn create_saros_dlmm_swap_ix(
    pair: &Pubkey,
    token_mint_x: &Pubkey,
    token_mint_y: &Pubkey,
    bin_array_lower: &Pubkey,
    bin_array_upper: &Pubkey,
    token_vault_x: &Pubkey,
    token_vault_y: &Pubkey,
    user_source_token: &Pubkey,
    user_destination_token: &Pubkey,
    user: &Pubkey,
    token_program_x: &Pubkey,
    token_program_y: &Pubkey,
    memo_program: &Pubkey,
    event_authority: &Pubkey,
    program: &Pubkey,
    amount: u64,
    direction: bool, // true if swapping X for Y
) -> Instruction {
    // Print detailed debug information about the instruction we're creating
    tracing::debug!("Creating Saros DLMM swap instruction with the following details:");
    tracing::debug!("  Program ID: {}", SAROS_DLMM_PROGRAM);
    tracing::debug!("  Accounts:");
    tracing::debug!("    1. pair: {} (mutable)", pair);
    tracing::debug!("    2. token_mint_x: {} (readonly)", token_mint_x);
    tracing::debug!("    3. token_mint_y: {} (readonly)", token_mint_y);
    tracing::debug!("    4. bin_array_lower: {} (mutable)", bin_array_lower);
    tracing::debug!("    5. bin_array_upper: {} (mutable)", bin_array_upper);
    tracing::debug!("    6. token_vault_x: {} (mutable)", token_vault_x);
    tracing::debug!("    7. token_vault_y: {} (mutable)", token_vault_y);
    tracing::debug!("    8. user_source_token: {} (mutable)", user_source_token);
    tracing::debug!("    9. user_destination_token: {} (mutable)", user_destination_token);
    tracing::debug!("    10. user: {} (signer)", user);
    tracing::debug!("    11. token_program_x: {} (readonly)", token_program_x);
    tracing::debug!("    12. token_program_y: {} (readonly)", token_program_y);
    tracing::debug!("    13. memo_program: {} (readonly)", memo_program);
    tracing::debug!("    14. event_authority: {} (readonly)", event_authority);
    tracing::debug!("    15. program: {} (readonly)", program);

    tracing::debug!("  Args:");
    tracing::debug!("    amount: {}", amount);
    tracing::debug!("    direction (X->Y): {}", direction);

    Instruction {
        program_id: SAROS_DLMM_PROGRAM,
        accounts: vec![
            AccountMeta::new(*pair, false),
            AccountMeta::new_readonly(*token_mint_x, false),
            AccountMeta::new_readonly(*token_mint_y, false),
            AccountMeta::new(*bin_array_lower, false),
            AccountMeta::new(*bin_array_upper, false),
            AccountMeta::new(*token_vault_x, false),
            AccountMeta::new(*token_vault_y, false),
            AccountMeta::new(*user_source_token, false),
            AccountMeta::new(*user_destination_token, false),
            AccountMeta::new(*user, true),
            AccountMeta::new_readonly(*token_program_x, false),
            AccountMeta::new_readonly(*token_program_y, false),
            AccountMeta::new_readonly(*memo_program, false),
            AccountMeta::new_readonly(*event_authority, false),
            AccountMeta::new_readonly(*program, false),
        ],
        data: create_saros_dlmm_instruction_data(amount, direction),
    }
}

// Helper function to determine swap direction for DLMM
pub fn determine_saros_dlmm_direction(
    source_mint: &Pubkey,
    token_mint_x: &Pubkey,
) -> bool {
    source_mint == token_mint_x
}

// Validation helper functions
pub fn validate_saros_program_id(program_id: &Pubkey) -> Result<(), &'static str> {
    if program_id != &SAROS_PROGRAM {
        return Err("Invalid Saros program ID");
    }
    Ok(())
}

pub fn validate_saros_dlmm_program_id(program_id: &Pubkey) -> Result<(), &'static str> {
    if program_id != &SAROS_DLMM_PROGRAM {
        return Err("Invalid Saros DLMM program ID");
    }
    Ok(())
}