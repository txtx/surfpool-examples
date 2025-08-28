use crate::prelude::*;
use crate::constants::GOOSEFX_GAMMA_PROGRAM;

fn create_gamma_oracle_swap_data(amount_in: u64, minimum_amount_out: u64) -> Vec<u8> {
    let mut buffer = Vec::with_capacity(24);
    // GAMMA_ORACLE_SWAP_SELECTOR (8 bytes)
    buffer.extend_from_slice(&[0x15, 0x2a, 0x7d, 0x94, 0x6e, 0x8c, 0x3b, 0x42]); // Example selector
    buffer.extend_from_slice(&amount_in.to_le_bytes());
    buffer.extend_from_slice(&minimum_amount_out.to_le_bytes());
    buffer
}

pub fn create_goosefx_gamma_swap_ix(
    payer: &Pubkey,
    authority: &Pubkey,
    amm_config: &Pubkey,
    pool_state: &Pubkey,
    input_token_account: &Pubkey,
    output_token_account: &Pubkey,
    input_vault: &Pubkey,
    output_vault: &Pubkey,
    input_token_mint: &Pubkey,
    output_token_mint: &Pubkey,
    observation_state: &Pubkey,
    amount_in: u64,
) -> Instruction {
    tracing::debug!("Creating GooseFX Gamma swap instruction with the following details:");
    tracing::debug!("  Program ID: {}", GOOSEFX_GAMMA_PROGRAM);
    tracing::debug!("  Accounts:");
    tracing::debug!("    1. payer: {} (signer)", payer);
    tracing::debug!("    2. authority: {} (readonly)", authority);
    tracing::debug!("    3. amm_config: {} (readonly)", amm_config);
    tracing::debug!("    4. pool_state: {} (mutable)", pool_state);
    tracing::debug!("    5. input_token_account: {} (mutable)", input_token_account);
    tracing::debug!("    6. output_token_account: {} (mutable)", output_token_account);
    tracing::debug!("    7. input_vault: {} (mutable)", input_vault);
    tracing::debug!("    8. output_vault: {} (mutable)", output_vault);
    tracing::debug!("    9. input_token_program: {} (readonly)", spl_token::id());
    tracing::debug!("    10. output_token_program: {} (readonly)", spl_token::id());
    tracing::debug!("    11. input_token_mint: {} (readonly)", input_token_mint);
    tracing::debug!("    12. output_token_mint: {} (readonly)", output_token_mint);
    tracing::debug!("    13. observation_state: {} (mutable)", observation_state);
    
    tracing::debug!("  Args:");
    tracing::debug!("    amount_in: {}", amount_in);
    tracing::debug!("    minimum_amount_out: 0 (allow any slippage)");
    
    Instruction {
        program_id: GOOSEFX_GAMMA_PROGRAM,
        accounts: vec![
            AccountMeta::new(*payer, true),
            AccountMeta::new_readonly(*authority, false),
            AccountMeta::new_readonly(*amm_config, false),
            AccountMeta::new(*pool_state, false),
            AccountMeta::new(*input_token_account, false),
            AccountMeta::new(*output_token_account, false),
            AccountMeta::new(*input_vault, false),
            AccountMeta::new(*output_vault, false),
            AccountMeta::new_readonly(spl_token::id(), false), // input_token_program
            AccountMeta::new_readonly(spl_token::id(), false), // output_token_program
            AccountMeta::new_readonly(*input_token_mint, false),
            AccountMeta::new_readonly(*output_token_mint, false),
            AccountMeta::new(*observation_state, false),
        ],
        data: create_gamma_oracle_swap_data(amount_in, 0),
    }
}