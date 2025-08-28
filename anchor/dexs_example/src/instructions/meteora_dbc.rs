use crate::prelude::*;
use crate::constants::{METEORA_DBC_PROGRAM};

fn create_instruction_data(discriminator: &[u8], amount_in: u64, min_amount_out: u64) -> Vec<u8> {
    let mut buffer = Vec::with_capacity(24);
    buffer.extend_from_slice(discriminator);
    buffer.extend_from_slice(&amount_in.to_le_bytes());
    buffer.extend_from_slice(&min_amount_out.to_le_bytes());
    buffer
}

pub fn create_meteora_dbc_swap_ix(
    pool: &Pubkey,
    user: &Pubkey,
    user_source_token: &Pubkey,
    user_destination_token: &Pubkey,
    pool_authority: &Pubkey,
    config: &Pubkey,
    base_vault: &Pubkey,
    quote_vault: &Pubkey,
    base_mint: &Pubkey,
    quote_mint: &Pubkey,
    token_base_program: &Pubkey,
    token_quote_program: &Pubkey,
    event_authority: &Pubkey,
    referral_token_account: Option<&Pubkey>,
    amount: u64,
) -> Instruction {
    tracing::debug!("Creating Meteora DBC swap instruction with the following details:");
    tracing::debug!("  Program ID: {}", METEORA_DBC_PROGRAM);
    tracing::debug!("  Accounts:");
    tracing::debug!("    1. pool_authority: {} (readonly)", pool_authority);
    tracing::debug!("    2. config: {} (readonly)", config);
    tracing::debug!("    3. pool: {} (mutable)", pool);
    tracing::debug!("    4. user_source_token: {} (mutable)", user_source_token);
    tracing::debug!("    5. user_destination_token: {} (mutable)", user_destination_token);
    tracing::debug!("    6. base_vault: {} (mutable)", base_vault);
    tracing::debug!("    7. quote_vault: {} (mutable)", quote_vault);
    tracing::debug!("    8. base_mint: {} (readonly)", base_mint);
    tracing::debug!("    9. quote_mint: {} (readonly)", quote_mint);
    tracing::debug!("    10. user: {} (signer)", user);
    tracing::debug!("    11. token_base_program: {} (readonly)", token_base_program);
    tracing::debug!("    12. token_quote_program: {} (readonly)", token_quote_program);
    
    let referral_account = referral_token_account.unwrap_or(&METEORA_DBC_PROGRAM);
    tracing::debug!("    13. referral_token_account: {} ({})", 
        referral_account, 
        if referral_token_account.is_some() { "mutable" } else { "readonly" }
    );
    tracing::debug!("    14. event_authority: {} (readonly)", event_authority);
    tracing::debug!("    15. program_id: {} (readonly)", METEORA_DBC_PROGRAM);
    
    tracing::debug!("  Args:");
    tracing::debug!("    amount: {}", amount);

    let swap_selector = &[0x60, 0x3e, 0x2a, 0xd4, 0x2c, 0x6e, 0x5a, 0x3b]; // SWAP_SELECTOR
    
    Instruction {
        program_id: METEORA_DBC_PROGRAM,
        accounts: vec![
            AccountMeta::new_readonly(*pool_authority, false),
            AccountMeta::new_readonly(*config, false),
            AccountMeta::new(*pool, false),
            AccountMeta::new(*user_source_token, false),
            AccountMeta::new(*user_destination_token, false),
            AccountMeta::new(*base_vault, false),
            AccountMeta::new(*quote_vault, false),
            AccountMeta::new_readonly(*base_mint, false),
            AccountMeta::new_readonly(*quote_mint, false),
            AccountMeta::new(*user, true),
            AccountMeta::new_readonly(*token_base_program, false),
            AccountMeta::new_readonly(*token_quote_program, false),
            if referral_token_account.is_some() {
                AccountMeta::new(*referral_account, false)
            } else {
                AccountMeta::new_readonly(*referral_account, false)
            },
            AccountMeta::new_readonly(*event_authority, false),
            AccountMeta::new_readonly(METEORA_DBC_PROGRAM, false),
        ],
        data: create_instruction_data(swap_selector, amount, 1),
    }
}

// Version 2 with sysvar instructions
pub fn create_meteora_dbc_swap_ix_v2(
    pool: &Pubkey,
    user: &Pubkey,
    user_source_token: &Pubkey,
    user_destination_token: &Pubkey,
    pool_authority: &Pubkey,
    config: &Pubkey,
    base_vault: &Pubkey,
    quote_vault: &Pubkey,
    base_mint: &Pubkey,
    quote_mint: &Pubkey,
    token_base_program: &Pubkey,
    token_quote_program: &Pubkey,
    event_authority: &Pubkey,
    sysvar_instructions: &Pubkey,
    referral_token_account: Option<&Pubkey>,
    amount: u64,
) -> Instruction {
    tracing::debug!("Creating Meteora DBC v2 swap instruction with the following details:");
    tracing::debug!("  Program ID: {}", METEORA_DBC_PROGRAM);
    tracing::debug!("  Accounts (v2 with sysvar):");
    tracing::debug!("    1. pool_authority: {} (readonly)", pool_authority);
    tracing::debug!("    2. config: {} (readonly)", config);
    tracing::debug!("    3. pool: {} (mutable)", pool);
    tracing::debug!("    4. user_source_token: {} (mutable)", user_source_token);
    tracing::debug!("    5. user_destination_token: {} (mutable)", user_destination_token);
    tracing::debug!("    6. base_vault: {} (mutable)", base_vault);
    tracing::debug!("    7. quote_vault: {} (mutable)", quote_vault);
    tracing::debug!("    8. base_mint: {} (readonly)", base_mint);
    tracing::debug!("    9. quote_mint: {} (readonly)", quote_mint);
    tracing::debug!("    10. user: {} (signer)", user);
    tracing::debug!("    11. token_base_program: {} (readonly)", token_base_program);
    tracing::debug!("    12. token_quote_program: {} (readonly)", token_quote_program);
    
    let referral_account = referral_token_account.unwrap_or(&METEORA_DBC_PROGRAM);
    tracing::debug!("    13. referral_token_account: {} ({})", 
        referral_account, 
        if referral_token_account.is_some() { "mutable" } else { "readonly" }
    );
    tracing::debug!("    14. event_authority: {} (readonly)", event_authority);
    tracing::debug!("    15. program_id: {} (readonly)", METEORA_DBC_PROGRAM);
    tracing::debug!("    16. sysvar_instructions: {} (readonly)", sysvar_instructions);
    
    tracing::debug!("  Args:");
    tracing::debug!("    amount: {}", amount);

    let swap_selector = &[0x60, 0x3e, 0x2a, 0xd4, 0x2c, 0x6e, 0x5a, 0x3b]; // SWAP_SELECTOR
    
    Instruction {
        program_id: METEORA_DBC_PROGRAM,
        accounts: vec![
            AccountMeta::new_readonly(*pool_authority, false),
            AccountMeta::new_readonly(*config, false),
            AccountMeta::new(*pool, false),
            AccountMeta::new(*user_source_token, false),
            AccountMeta::new(*user_destination_token, false),
            AccountMeta::new(*base_vault, false),
            AccountMeta::new(*quote_vault, false),
            AccountMeta::new_readonly(*base_mint, false),
            AccountMeta::new_readonly(*quote_mint, false),
            AccountMeta::new(*user, true),
            AccountMeta::new_readonly(*token_base_program, false),
            AccountMeta::new_readonly(*token_quote_program, false),
            if referral_token_account.is_some() {
                AccountMeta::new(*referral_account, false)
            } else {
                AccountMeta::new_readonly(*referral_account, false)
            },
            AccountMeta::new_readonly(*event_authority, false),
            AccountMeta::new_readonly(METEORA_DBC_PROGRAM, false),
            AccountMeta::new_readonly(*sysvar_instructions, false),
        ],
        data: create_instruction_data(swap_selector, amount, 1),
    }
}