use crate::prelude::*;
use crate::constants::{ZEROFI_PROGRAM};

fn create_instruction_data(amount_in: u64) -> Vec<u8> {
    let mut buffer = Vec::with_capacity(17);
    buffer.push(6u8); // discriminator
    buffer.extend_from_slice(&amount_in.to_le_bytes()); // amount_in
    buffer.extend_from_slice(&0u64.to_le_bytes()); // desired output token amount
    buffer
}

pub fn create_zerofi_swap_ix(
    pair: &Pubkey,
    vault_info_in: &Pubkey,
    vault_in: &Pubkey,
    vault_info_out: &Pubkey,
    vault_out: &Pubkey,
    user_source_token: &Pubkey,
    user_destination_token: &Pubkey,
    user: &Pubkey,
    token_program: &Pubkey,
    sysvar_instructions: &Pubkey,
    amount: u64,
) -> Instruction {
    // Print detailed debug information about the instruction we're creating
    tracing::debug!("Creating ZeroFi swap instruction with the following details:");
    tracing::debug!("  Program ID: {}", ZEROFI_PROGRAM);
    tracing::debug!("  Accounts:");
    tracing::debug!("    1. pair: {} (mutable)", pair);
    tracing::debug!("    2. vault_info_in: {} (mutable)", vault_info_in);
    tracing::debug!("    3. vault_in: {} (mutable)", vault_in);
    tracing::debug!("    4. vault_info_out: {} (mutable)", vault_info_out);
    tracing::debug!("    5. vault_out: {} (mutable)", vault_out);
    tracing::debug!("    6. user_source_token: {} (mutable)", user_source_token);
    tracing::debug!("    7. user_destination_token: {} (mutable)", user_destination_token);
    tracing::debug!("    8. user: {} (signer)", user);
    tracing::debug!("    9. token_program: {} (readonly)", token_program);
    tracing::debug!("    10. sysvar_instructions: {} (readonly)", sysvar_instructions);
    
    tracing::debug!("  Args:");
    tracing::debug!("    amount: {}", amount);
    
    Instruction {
        program_id: ZEROFI_PROGRAM,
        accounts: vec![
            AccountMeta::new(*pair, false),
            AccountMeta::new(*vault_info_in, false),
            AccountMeta::new(*vault_in, false),
            AccountMeta::new(*vault_info_out, false),
            AccountMeta::new(*vault_out, false),
            AccountMeta::new(*user_source_token, false),
            AccountMeta::new(*user_destination_token, false),
            AccountMeta::new_readonly(*user, true),
            AccountMeta::new_readonly(*token_program, false),
            AccountMeta::new_readonly(*sysvar_instructions, false),
        ],
        data: create_instruction_data(amount),
    }
}