use crate::prelude::*;
use crate::constants::FLUX_BEAM_PROGRAM;

fn create_instruction_data(discriminator: u8, amount_in: u64, minimum_amount_out: u64) -> Vec<u8> {
    let mut buffer = Vec::with_capacity(17);
    buffer.push(discriminator);
    buffer.extend_from_slice(&amount_in.to_le_bytes());
    buffer.extend_from_slice(&minimum_amount_out.to_le_bytes());
    buffer
}

pub fn create_fluxbeam_swap_ix(
    swap_info: &Pubkey,
    authority_acc_info: &Pubkey,
    swap_authority: &Pubkey,
    swap_source_token: &Pubkey,
    token_a_account: &Pubkey,
    token_b_account: &Pubkey,
    swap_destination_token: &Pubkey,
    pool_mint: &Pubkey,
    pool_fee: &Pubkey,
    source_mint: &Pubkey,
    destination_mint: &Pubkey,
    source_token_program: &Pubkey,
    destination_token_program: &Pubkey,
    token_program_2022: &Pubkey,
    amount_in: u64,
    minimum_amount_out: u64,
) -> Instruction {
    // Print detailed debug information about the instruction we're creating
    tracing::debug!("Creating FluxBeam swap instruction with the following details:");
    tracing::debug!("  Program ID: {}", FLUX_BEAM_PROGRAM);
    tracing::debug!("  Accounts:");
    tracing::debug!("    1. swap_info: {} (readonly)", swap_info);
    tracing::debug!("    2. authority_acc_info: {} (readonly)", authority_acc_info);
    tracing::debug!("    3. swap_authority: {} (signer, mutable)", swap_authority);
    tracing::debug!("    4. swap_source_token: {} (mutable)", swap_source_token);
    tracing::debug!("    5. token_a_account: {} (mutable)", token_a_account);
    tracing::debug!("    6. token_b_account: {} (mutable)", token_b_account);
    tracing::debug!("    7. swap_destination_token: {} (mutable)", swap_destination_token);
    tracing::debug!("    8. pool_mint: {} (mutable)", pool_mint);
    tracing::debug!("    9. pool_fee: {} (mutable)", pool_fee);
    tracing::debug!("    10. source_mint: {} (readonly)", source_mint);
    tracing::debug!("    11. destination_mint: {} (readonly)", destination_mint);
    tracing::debug!("    12. source_token_program: {} (readonly)", source_token_program);
    tracing::debug!("    13. destination_token_program: {} (readonly)", destination_token_program);
    tracing::debug!("    14. token_program_2022: {} (readonly)", token_program_2022);
    
    tracing::debug!("  Args:");
    tracing::debug!("    amount_in: {}", amount_in);
    tracing::debug!("    minimum_amount_out: {}", minimum_amount_out);

    Instruction {
        program_id: FLUX_BEAM_PROGRAM,
        accounts: vec![
            AccountMeta::new_readonly(*swap_info, false),
            AccountMeta::new_readonly(*authority_acc_info, false),
            AccountMeta::new(*swap_authority, true),
            AccountMeta::new(*swap_source_token, false),
            AccountMeta::new(*token_a_account, false),
            AccountMeta::new(*token_b_account, false),
            AccountMeta::new(*swap_destination_token, false),
            AccountMeta::new(*pool_mint, false),
            AccountMeta::new(*pool_fee, false),
            AccountMeta::new_readonly(*source_mint, false),
            AccountMeta::new_readonly(*destination_mint, false),
            AccountMeta::new_readonly(*source_token_program, false),
            AccountMeta::new_readonly(*destination_token_program, false),
            AccountMeta::new_readonly(*token_program_2022, false),
        ],
        data: create_instruction_data(1, amount_in, minimum_amount_out),
    }
}

// Helper function for determining pool token order - preserves original validation logic
pub fn determine_pool_tokens(
    swap_source_mint: &Pubkey,
    swap_destination_mint: &Pubkey,
    token_a_mint: &Pubkey,
    token_b_mint: &Pubkey,
    token_a_account: &Pubkey,
    token_b_account: &Pubkey,
) -> Result<(Pubkey, Pubkey), Box<dyn std::error::Error>> {
    if (swap_source_mint == token_a_mint) && (swap_destination_mint == token_b_mint) {
        Ok((*token_a_account, *token_b_account))
    } else if (swap_source_mint == token_b_mint) && (swap_destination_mint == token_a_mint) {
        Ok((*token_b_account, *token_a_account))
    } else {
        Err("Invalid pool token configuration".into())
    }
}