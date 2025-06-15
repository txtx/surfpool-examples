use crate::prelude::*;
use crate::constants::{SOLFI_PROGRAM};

fn create_instruction_data(discriminator: u8, amount_in: u64) -> Vec<u8> {
    let mut buffer = Vec::with_capacity(9);
    buffer.push(discriminator);
    buffer.extend_from_slice(&amount_in.to_le_bytes());
    buffer.resize(18, 0);
    buffer
}

pub fn create_solfi_swap_ix(
    market: &Pubkey,
    user: &Pubkey,
    from: &Pubkey,
    to: &Pubkey,
    amount: u64,
) -> Instruction {
    // Print detailed debug information about the instruction we're creating
    tracing::debug!("Creating SolFi swap instruction with the following details:");
    tracing::debug!("  Program ID: {}", SOLFI_PROGRAM);
    tracing::debug!("  Accounts:");
    tracing::debug!("    1. user: {} (signer)", user);
    tracing::debug!("    2. market: {} (mutable)", market);

    let market_source_token = get_associated_token_address(market, from);
    let market_destination_token = get_associated_token_address(market, to);
    tracing::debug!("    3. market source token: {} (mutable)", market_source_token);
    tracing::debug!("    4. market destination token: {} (mutable)", market_destination_token);

    let user_source_token = get_associated_token_address(user, from);
    let user_destination_token = get_associated_token_address(user, to);
    tracing::debug!("    5. user source token: {} (mutable)", user_source_token);
    tracing::debug!("    6. user destination token: {} (mutable)", user_destination_token);

    tracing::debug!("    7. tokenProgram: {} (readonly)", spl_token::id());
    tracing::debug!("    8. sysvarID: {} (readonly)", sysvarID);
    
    tracing::debug!("  Args:");
    tracing::debug!("    amount: {}", amount);
    Instruction {
        program_id: SOLFI_PROGRAM,
        accounts: vec![
            AccountMeta::new(*user, true),
            AccountMeta::new(*market, false),
            AccountMeta::new(get_associated_token_address(market, from), false),
            AccountMeta::new(get_associated_token_address(market, to), false),
            AccountMeta::new(get_associated_token_address(user, from), false),
            AccountMeta::new(get_associated_token_address(user, to), false),
            AccountMeta::new_readonly(spl_token::id(), false),
            AccountMeta::new_readonly(sysvarID, false),
        ],
        data: create_instruction_data(7, amount),
    }
}