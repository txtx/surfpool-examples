use crate::prelude::*;
use crate::constants::{STABBLE_STABLE_PROGRAM, STABBLE_WEIGHTED_PROGRAM};
use anchor_spl::{
    token::Token,
    token_2022::Token2022,
};

fn create_instruction_data(amount_in: u64) -> Vec<u8> {
    let mut buffer = Vec::with_capacity(25);
    buffer.extend_from_slice(&[0x83, 0xa7, 0x4a, 0x95, 0x68, 0xd4, 0xf6, 0xb2]); // STABBLE_SWAP_SELECTOR
    buffer.extend_from_slice(&(Some(amount_in)).try_to_vec().unwrap());
    buffer.extend_from_slice(&1u64.to_le_bytes());
    buffer
}

pub fn create_stabble_swap_ix(
    program_id: &Pubkey,
    user: &Pubkey,
    mint_in: &Pubkey,
    mint_out: &Pubkey,
    user_source_token: &Pubkey,
    user_destination_token: &Pubkey,
    vault_token_in: &Pubkey,
    vault_token_out: &Pubkey,
    beneficiary_token_out: &Pubkey,
    pool_token_in: &Pubkey,
    withdraw_authority: &Pubkey,
    vault: &Pubkey,
    vault_authority: &Pubkey,
    vault_program: &Pubkey,
    amount: u64,
) -> Instruction {
    tracing::debug!("Creating Stabble swap instruction with the following details:");
    tracing::debug!("  Program ID: {}", program_id);
    tracing::debug!("  Accounts:");
    tracing::debug!("    1. user: {} (signer)", user);
    tracing::debug!("    2. mint_in: {} (readonly)", mint_in);
    tracing::debug!("    3. mint_out: {} (readonly)", mint_out);
    tracing::debug!("    4. user_source_token: {} (mutable)", user_source_token);
    tracing::debug!("    5. user_destination_token: {} (mutable)", user_destination_token);
    tracing::debug!("    6. vault_token_in: {} (mutable)", vault_token_in);
    tracing::debug!("    7. vault_token_out: {} (mutable)", vault_token_out);
    tracing::debug!("    8. beneficiary_token_out: {} (mutable)", beneficiary_token_out);
    tracing::debug!("    9. pool_token_in: {} (mutable)", pool_token_in);
    tracing::debug!("    10. withdraw_authority: {} (readonly)", withdraw_authority);
    tracing::debug!("    11. vault: {} (readonly)", vault);
    tracing::debug!("    12. vault_authority: {} (readonly)", vault_authority);
    tracing::debug!("    13. vault_program: {} (readonly)", vault_program);
    tracing::debug!("    14. token_program: {} (readonly)", spl_token::id());
    tracing::debug!("    15. token_2022_program: {} (readonly)", spl_token_2022::id());

    tracing::debug!("  Args:");
    tracing::debug!("    amount: {}", amount);
    tracing::debug!("    min_amount_out: 1");

    // Validate program ID
    if program_id != &STABBLE_STABLE_PROGRAM && program_id != &STABBLE_WEIGHTED_PROGRAM {
        panic!("Invalid program ID for Stabble swap");
    }

    Instruction {
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new(*user, true),
            AccountMeta::new_readonly(*mint_in, false),
            AccountMeta::new_readonly(*mint_out, false),
            AccountMeta::new(*user_source_token, false),
            AccountMeta::new(*user_destination_token, false),
            AccountMeta::new(*vault_token_in, false),
            AccountMeta::new(*vault_token_out, false),
            AccountMeta::new(*beneficiary_token_out, false),
            AccountMeta::new(*pool_token_in, false),
            AccountMeta::new_readonly(*withdraw_authority, false),
            AccountMeta::new_readonly(*vault, false),
            AccountMeta::new_readonly(*vault_authority, false),
            AccountMeta::new_readonly(*vault_program, false),
            AccountMeta::new_readonly(spl_token::id(), false),
            AccountMeta::new_readonly(spl_token_2022::id(), false),
        ],
        data: create_instruction_data(amount),
    }
}