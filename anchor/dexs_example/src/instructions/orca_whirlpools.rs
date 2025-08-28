use crate::prelude::*;
use crate::constants::WHIRLPOOL_PROGRAM;

fn create_whirlpool_instruction_data(amount_in: u64, other_amount_threshold: u64, sqrt_price_limit: i128, amount_specified_is_input: bool, a_to_b: bool) -> Vec<u8> {
    let mut buffer = Vec::with_capacity(42);
    buffer.extend_from_slice(&[0x09, 0xf3, 0xcd, 0x7d, 0x76, 0x0f, 0xd9, 0xa9]); // SWAP_SELECTOR
    buffer.extend_from_slice(&amount_in.to_le_bytes());
    buffer.extend_from_slice(&other_amount_threshold.to_le_bytes());
    buffer.extend_from_slice(&sqrt_price_limit.to_le_bytes());
    buffer.push(amount_specified_is_input as u8);
    buffer.push(a_to_b as u8);
    buffer
}

fn create_whirlpool_v2_instruction_data(amount_in: u64, other_amount_threshold: u64, sqrt_price_limit: i128, amount_specified_is_input: bool, a_to_b: bool) -> Vec<u8> {
    let mut buffer = Vec::with_capacity(43);
    buffer.extend_from_slice(&[0x10, 0xf8, 0xce, 0x8e, 0x87, 0x1f, 0xea, 0xba]); // SWAP_V2_SELECTOR
    buffer.extend_from_slice(&amount_in.to_le_bytes());
    buffer.extend_from_slice(&other_amount_threshold.to_le_bytes());
    buffer.extend_from_slice(&sqrt_price_limit.to_le_bytes());
    buffer.push(amount_specified_is_input as u8);
    buffer.push(a_to_b as u8);
    buffer.push(0u8);
    buffer
}

pub fn create_whirlpool_swap_ix(
    whirlpool: &Pubkey,
    user: &Pubkey,
    source_token: &Pubkey,
    destination_token: &Pubkey,
    token_vault_a: &Pubkey,
    token_vault_b: &Pubkey,
    tick_array0: &Pubkey,
    tick_array1: &Pubkey,
    tick_array2: &Pubkey,
    oracle: &Pubkey,
    amount_in: u64,
) -> Result<Instruction> {
    tracing::debug!("Creating Whirlpool swap instruction with the following details:");
    tracing::debug!("  Program ID: {}", WHIRLPOOL_PROGRAM);
    tracing::debug!("  Accounts:");
    tracing::debug!("    1. token_program: {} (readonly)", spl_token::id());
    tracing::debug!("    2. user: {} (signer)", user);
    tracing::debug!("    3. whirlpool: {} (mutable)", whirlpool);
    tracing::debug!("    4. source_token: {} (mutable)", source_token);
    tracing::debug!("    5. token_vault_a: {} (mutable)", token_vault_a);
    tracing::debug!("    6. destination_token: {} (mutable)", destination_token);
    tracing::debug!("    7. token_vault_b: {} (mutable)", token_vault_b);
    tracing::debug!("    8. tick_array0: {} (mutable)", tick_array0);
    tracing::debug!("    9. tick_array1: {} (mutable)", tick_array1);
    tracing::debug!("    10. tick_array2: {} (mutable)", tick_array2);
    tracing::debug!("    11. oracle: {} (readonly)", oracle);
    
    tracing::debug!("  Args:");
    tracing::debug!("    amount_in: {}", amount_in);
    
    // Determine swap direction and price limits
    let a_to_b = true; // This would need to be determined based on token mints
    let sqrt_price_limit = if a_to_b {
        4295048016i128 // Minimum sqrt-price
    } else {
        79226673515401279992447579055i128 // Maximum sqrt-price
    };
    
    let other_amount_threshold = 1u64;
    let amount_specified_is_input = true;
    
    Ok(Instruction {
        program_id: WHIRLPOOL_PROGRAM,
        accounts: vec![
            AccountMeta::new_readonly(spl_token::id(), false),
            AccountMeta::new(*user, true),
            AccountMeta::new(*whirlpool, false),
            AccountMeta::new(*source_token, false),
            AccountMeta::new(*token_vault_a, false),
            AccountMeta::new(*destination_token, false),
            AccountMeta::new(*token_vault_b, false),
            AccountMeta::new(*tick_array0, false),
            AccountMeta::new(*tick_array1, false),
            AccountMeta::new(*tick_array2, false),
            AccountMeta::new_readonly(*oracle, false),
        ],
        data: create_whirlpool_instruction_data(
            amount_in,
            other_amount_threshold,
            sqrt_price_limit,
            amount_specified_is_input,
            a_to_b,
        ),
    })
}

pub fn create_whirlpool_v2_swap_ix(
    whirlpool: &Pubkey,
    user: &Pubkey,
    source_token: &Pubkey,
    destination_token: &Pubkey,
    token_program_a: &Pubkey,
    token_program_b: &Pubkey,
    memo_program: &Pubkey,
    token_mint_a: &Pubkey,
    token_mint_b: &Pubkey,
    token_vault_a: &Pubkey,
    token_vault_b: &Pubkey,
    tick_array0: &Pubkey,
    tick_array1: &Pubkey,
    tick_array2: &Pubkey,
    oracle: &Pubkey,
    amount_in: u64,
) -> Result<Instruction> {
    tracing::debug!("Creating Whirlpool V2 swap instruction with the following details:");
    tracing::debug!("  Program ID: {}", WHIRLPOOL_PROGRAM);
    tracing::debug!("  Accounts:");
    tracing::debug!("    1. token_program_a: {} (readonly)", token_program_a);
    tracing::debug!("    2. token_program_b: {} (readonly)", token_program_b);
    tracing::debug!("    3. memo_program: {} (readonly)", memo_program);
    tracing::debug!("    4. user: {} (signer)", user);
    tracing::debug!("    5. whirlpool: {} (mutable)", whirlpool);
    tracing::debug!("    6. token_mint_a: {} (readonly)", token_mint_a);
    tracing::debug!("    7. token_mint_b: {} (readonly)", token_mint_b);
    tracing::debug!("    8. source_token: {} (mutable)", source_token);
    tracing::debug!("    9. token_vault_a: {} (mutable)", token_vault_a);
    tracing::debug!("    10. destination_token: {} (mutable)", destination_token);
    tracing::debug!("    11. token_vault_b: {} (mutable)", token_vault_b);
    tracing::debug!("    12. tick_array0: {} (mutable)", tick_array0);
    tracing::debug!("    13. tick_array1: {} (mutable)", tick_array1);
    tracing::debug!("    14. tick_array2: {} (mutable)", tick_array2);
    tracing::debug!("    15. oracle: {} (mutable)", oracle);
    
    tracing::debug!("  Args:");
    tracing::debug!("    amount_in: {}", amount_in);
    
    // Determine swap direction and price limits
    let a_to_b = true; // This would need to be determined based on token mints
    let sqrt_price_limit = if a_to_b {
        4295048016i128 // Minimum sqrt-price
    } else {
        79226673515401279992447579055i128 // Maximum sqrt-price
    };
    
    let other_amount_threshold = 1u64;
    let amount_specified_is_input = true;
    
    Ok(Instruction {
        program_id: WHIRLPOOL_PROGRAM,
        accounts: vec![
            AccountMeta::new_readonly(*token_program_a, false),
            AccountMeta::new_readonly(*token_program_b, false),
            AccountMeta::new_readonly(*memo_program, false),
            AccountMeta::new(*user, true),
            AccountMeta::new(*whirlpool, false),
            AccountMeta::new_readonly(*token_mint_a, false),
            AccountMeta::new_readonly(*token_mint_b, false),
            AccountMeta::new(*source_token, false),
            AccountMeta::new(*token_vault_a, false),
            AccountMeta::new(*destination_token, false),
            AccountMeta::new(*token_vault_b, false),
            AccountMeta::new(*tick_array0, false),
            AccountMeta::new(*tick_array1, false),
            AccountMeta::new(*tick_array2, false),
            AccountMeta::new(*oracle, false),
        ],
        data: create_whirlpool_v2_instruction_data(
            amount_in,
            other_amount_threshold,
            sqrt_price_limit,
            amount_specified_is_input,
            a_to_b,
        ),
    })
}