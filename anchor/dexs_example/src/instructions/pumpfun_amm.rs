use crate::prelude::*;
use crate::constants::PUMPFUN_PROGRAM;

fn create_pumpfun_sell_instruction_data(base_amount_in: u64, min_quote_amount_out: u64) -> Vec<u8> {
    let mut buffer = Vec::with_capacity(24);
    buffer.extend_from_slice(&[0x33, 0xe6, 0x85, 0xa1, 0x2b, 0x34, 0x3d, 0x28]); // PUMPFUN_SELL_SELECTOR
    buffer.extend_from_slice(&base_amount_in.to_le_bytes());
    buffer.extend_from_slice(&min_quote_amount_out.to_le_bytes());
    buffer
}

fn create_pumpfun_buy_instruction_data(base_amount_out: u64, max_quote_amount_in: u64) -> Vec<u8> {
    let mut buffer = Vec::with_capacity(24);
    buffer.extend_from_slice(&[0x66, 0x63, 0xd4, 0xf3, 0xab, 0xfc, 0x12, 0x95]); // PUMPFUN_BUY_SELECTOR
    buffer.extend_from_slice(&base_amount_out.to_le_bytes());
    buffer.extend_from_slice(&max_quote_amount_in.to_le_bytes());
    buffer
}

pub fn create_pumpfun_sell_ix(
    pool: &Pubkey,
    user: &Pubkey,
    global_config: &Pubkey,
    base_mint: &Pubkey,
    quote_mint: &Pubkey,
    user_base_token: &Pubkey,
    user_quote_token: &Pubkey,
    pool_base_token_account: &Pubkey,
    pool_quote_token_account: &Pubkey,
    protocol_fee_recipient: &Pubkey,
    protocol_fee_recipient_token_account: &Pubkey,
    base_token_program: &Pubkey,
    quote_token_program: &Pubkey,
    system_program: &Pubkey,
    associated_token_program: &Pubkey,
    event_authority: &Pubkey,
    coin_creator_vault_ata: &Pubkey,
    coin_creator_vault_authority: &Pubkey,
    amount_in: u64,
) -> Instruction {
    tracing::debug!("Creating PumpFun sell instruction with the following details:");
    tracing::debug!("  Program ID: {}", PUMPFUN_PROGRAM);
    tracing::debug!("  Accounts:");
    tracing::debug!("    1. pool: {} (readonly)", pool);
    tracing::debug!("    2. user: {} (signer)", user);
    tracing::debug!("    3. global_config: {} (readonly)", global_config);
    tracing::debug!("    4. base_mint: {} (readonly)", base_mint);
    tracing::debug!("    5. quote_mint: {} (readonly)", quote_mint);
    tracing::debug!("    6. user_base_token: {} (mutable)", user_base_token);
    tracing::debug!("    7. user_quote_token: {} (mutable)", user_quote_token);
    tracing::debug!("    8. pool_base_token_account: {} (mutable)", pool_base_token_account);
    tracing::debug!("    9. pool_quote_token_account: {} (mutable)", pool_quote_token_account);
    tracing::debug!("    10. protocol_fee_recipient: {} (readonly)", protocol_fee_recipient);
    tracing::debug!("    11. protocol_fee_recipient_token_account: {} (mutable)", protocol_fee_recipient_token_account);
    tracing::debug!("    12. base_token_program: {} (readonly)", base_token_program);
    tracing::debug!("    13. quote_token_program: {} (readonly)", quote_token_program);
    tracing::debug!("    14. system_program: {} (readonly)", system_program);
    tracing::debug!("    15. associated_token_program: {} (readonly)", associated_token_program);
    tracing::debug!("    16. event_authority: {} (readonly)", event_authority);
    tracing::debug!("    17. dex_program: {} (readonly)", PUMPFUN_PROGRAM);
    tracing::debug!("    18. coin_creator_vault_ata: {} (mutable)", coin_creator_vault_ata);
    tracing::debug!("    19. coin_creator_vault_authority: {} (readonly)", coin_creator_vault_authority);
    
    tracing::debug!("  Args:");
    tracing::debug!("    amount_in: {}", amount_in);
    
    let min_quote_amount_out = 1u64; // Minimum slippage protection

    Instruction {
        program_id: PUMPFUN_PROGRAM,
        accounts: vec![
            AccountMeta::new_readonly(*pool, false),
            AccountMeta::new(*user, true),
            AccountMeta::new_readonly(*global_config, false),
            AccountMeta::new_readonly(*base_mint, false),
            AccountMeta::new_readonly(*quote_mint, false),
            AccountMeta::new(*user_base_token, false),
            AccountMeta::new(*user_quote_token, false),
            AccountMeta::new(*pool_base_token_account, false),
            AccountMeta::new(*pool_quote_token_account, false),
            AccountMeta::new_readonly(*protocol_fee_recipient, false),
            AccountMeta::new(*protocol_fee_recipient_token_account, false),
            AccountMeta::new_readonly(*base_token_program, false),
            AccountMeta::new_readonly(*quote_token_program, false),
            AccountMeta::new_readonly(*system_program, false),
            AccountMeta::new_readonly(*associated_token_program, false),
            AccountMeta::new_readonly(*event_authority, false),
            AccountMeta::new_readonly(PUMPFUN_PROGRAM, false),
            AccountMeta::new(*coin_creator_vault_ata, false),
            AccountMeta::new_readonly(*coin_creator_vault_authority, false),
        ],
        data: create_pumpfun_sell_instruction_data(amount_in, min_quote_amount_out),
    }
}

pub fn create_pumpfun_buy_ix(
    pool: &Pubkey,
    user: &Pubkey,
    global_config: &Pubkey,
    base_mint: &Pubkey,
    quote_mint: &Pubkey,
    user_base_token: &Pubkey,
    user_quote_token: &Pubkey,
    pool_base_token_account: &Pubkey,
    pool_quote_token_account: &Pubkey,
    protocol_fee_recipient: &Pubkey,
    protocol_fee_recipient_token_account: &Pubkey,
    base_token_program: &Pubkey,
    quote_token_program: &Pubkey,
    system_program: &Pubkey,
    associated_token_program: &Pubkey,
    event_authority: &Pubkey,
    coin_creator_vault_ata: &Pubkey,
    coin_creator_vault_authority: &Pubkey,
    global_volume_accumulator: &Pubkey,
    user_volume_accumulator: &Pubkey,
    amount_out: u64,
    max_amount_in: u64,
) -> Instruction {
    tracing::debug!("Creating PumpFun buy instruction with the following details:");
    tracing::debug!("  Program ID: {}", PUMPFUN_PROGRAM);
    tracing::debug!("  Accounts:");
    tracing::debug!("    1. pool: {} (readonly)", pool);
    tracing::debug!("    2. user: {} (signer)", user);
    tracing::debug!("    3. global_config: {} (readonly)", global_config);
    tracing::debug!("    4. base_mint: {} (readonly)", base_mint);
    tracing::debug!("    5. quote_mint: {} (readonly)", quote_mint);
    tracing::debug!("    6. user_base_token: {} (mutable)", user_base_token);
    tracing::debug!("    7. user_quote_token: {} (mutable)", user_quote_token);
    tracing::debug!("    8. pool_base_token_account: {} (mutable)", pool_base_token_account);
    tracing::debug!("    9. pool_quote_token_account: {} (mutable)", pool_quote_token_account);
    tracing::debug!("    10. protocol_fee_recipient: {} (readonly)", protocol_fee_recipient);
    tracing::debug!("    11. protocol_fee_recipient_token_account: {} (mutable)", protocol_fee_recipient_token_account);
    tracing::debug!("    12. base_token_program: {} (readonly)", base_token_program);
    tracing::debug!("    13. quote_token_program: {} (readonly)", quote_token_program);
    tracing::debug!("    14. system_program: {} (readonly)", system_program);
    tracing::debug!("    15. associated_token_program: {} (readonly)", associated_token_program);
    tracing::debug!("    16. event_authority: {} (readonly)", event_authority);
    tracing::debug!("    17. dex_program: {} (readonly)", PUMPFUN_PROGRAM);
    tracing::debug!("    18. coin_creator_vault_ata: {} (mutable)", coin_creator_vault_ata);
    tracing::debug!("    19. coin_creator_vault_authority: {} (readonly)", coin_creator_vault_authority);
    tracing::debug!("    20. global_volume_accumulator: {} (mutable)", global_volume_accumulator);
    tracing::debug!("    21. user_volume_accumulator: {} (mutable)", user_volume_accumulator);
    
    tracing::debug!("  Args:");
    tracing::debug!("    amount_out: {}", amount_out);
    tracing::debug!("    max_amount_in: {}", max_amount_in);

    Instruction {
        program_id: PUMPFUN_PROGRAM,
        accounts: vec![
            AccountMeta::new_readonly(*pool, false),
            AccountMeta::new(*user, true),
            AccountMeta::new_readonly(*global_config, false),
            AccountMeta::new_readonly(*base_mint, false),
            AccountMeta::new_readonly(*quote_mint, false),
            AccountMeta::new(*user_base_token, false),
            AccountMeta::new(*user_quote_token, false),
            AccountMeta::new(*pool_base_token_account, false),
            AccountMeta::new(*pool_quote_token_account, false),
            AccountMeta::new_readonly(*protocol_fee_recipient, false),
            AccountMeta::new(*protocol_fee_recipient_token_account, false),
            AccountMeta::new_readonly(*base_token_program, false),
            AccountMeta::new_readonly(*quote_token_program, false),
            AccountMeta::new_readonly(*system_program, false),
            AccountMeta::new_readonly(*associated_token_program, false),
            AccountMeta::new_readonly(*event_authority, false),
            AccountMeta::new_readonly(PUMPFUN_PROGRAM, false),
            AccountMeta::new(*coin_creator_vault_ata, false),
            AccountMeta::new_readonly(*coin_creator_vault_authority, false),
            AccountMeta::new(*global_volume_accumulator, false),
            AccountMeta::new(*user_volume_accumulator, false),
        ],
        data: create_pumpfun_buy_instruction_data(amount_out, max_amount_in),
    }
}

// Helper function to calculate base amount out for buy operations
pub fn calculate_base_amount_out(
    amount_in: u128,
    base_reserves: u64,
    quote_reserves: u64,
    lp_fee_bps: u64,
    protocol_fee_bps: u64,
    creator_fee_bps: u64,
) -> Result<u128, &'static str> {
    if base_reserves == 0 || quote_reserves == 0 {
        return Err("Invalid pool reserves");
    }

    let total_fee_bps = lp_fee_bps + protocol_fee_bps + creator_fee_bps;
    let denominator = (total_fee_bps as u128) + 10000u128;

    let effective_quote = (amount_in * 10000u128) / denominator;
    let numerator = (base_reserves as u128) * effective_quote;
    let denominator_effective = (quote_reserves as u128) + effective_quote;
    
    Ok(numerator / denominator_effective)
}