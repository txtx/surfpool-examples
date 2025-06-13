use crate::prelude::*;
use solana_instruction::{Instruction, account_meta::{AccountMeta}};
use solana_sysvar_id::ID as sysvarID;
use spl_associated_token_account::get_associated_token_address;
use crate::constants::{SOLFI_PROGRAM};

// Creates instruction data for Lifinity V2 swap with Anchor format
fn create_lifinity_v2_instruction_data(amount_in: u64, minimum_amount_out: u64) -> Vec<u8> {
    // Anchor discriminator (first 8 bytes of sha256("global:swap"))
    let discriminator: [u8; 8] = [248, 198, 158, 145, 225, 117, 135, 200];
    
    let mut buffer = Vec::with_capacity(24);
    buffer.extend_from_slice(&discriminator);
    buffer.extend_from_slice(&amount_in.to_le_bytes());
    buffer.extend_from_slice(&minimum_amount_out.to_le_bytes());
    buffer
}

pub fn create_lifinity_v2_swap_ix(
    pool_authority: &Pubkey,
    amm: &Pubkey,
    user: &Pubkey,
    from: &Pubkey,
    to: &Pubkey,
    pool_mint: &Pubkey,
    fee_account: &Pubkey,
    oracle_main: &Pubkey,
    oracle_sub: &Pubkey,
    oracle_pc: &Pubkey,
    pool_source_token: &Pubkey,
    pool_destination_token: &Pubkey,
    amount_in: u64,
    minimum_amount_out: u64,
) -> Instruction {
    // Print detailed debug information about the instruction we're creating
    tracing::debug!("Creating Lifinity V2 swap instruction with the following details:");
    tracing::debug!("  Program ID: {}", LIFINITY_V2_PROGRAM);
    tracing::debug!("  Accounts:");
    tracing::debug!("    1. authority: {} (not signer)", pool_authority);
    tracing::debug!("    2. amm: {} (mutable)", amm);
    tracing::debug!("    3. userTransferAuthority: {} (signer)", user);
    
    let user_source_token = get_associated_token_address(user, from);
    tracing::debug!("    4. sourceInfo: {} (mutable)", user_source_token);
    
    let user_destination_token = get_associated_token_address(user, to);
    tracing::debug!("    5. destinationInfo: {} (mutable)", user_destination_token);
    
    tracing::debug!("    6. swapSource: {} (mutable)", pool_source_token);
    tracing::debug!("    7. swapDestination: {} (mutable)", pool_destination_token);
    
    tracing::debug!("    8. poolMint: {} (mutable)", pool_mint);
    tracing::debug!("    9. feeAccount: {} (mutable)", fee_account);
    tracing::debug!("    10. tokenProgram: {} (not mutable)", spl_token::id());
    tracing::debug!("    11. oracleMainAccount: {} (not mutable)", oracle_main);
    tracing::debug!("    12. oracleSubAccount: {} (not mutable)", oracle_sub);
    tracing::debug!("    13. oraclePcAccount: {} (not mutable)", oracle_pc);
    tracing::debug!("  Args:");
    tracing::debug!("    amountIn: {}", amount_in);
    tracing::debug!("    minimumAmountOut: {}", minimum_amount_out);
    
    Instruction {
        program_id: LIFINITY_V2_PROGRAM,
        accounts: vec![
            AccountMeta::new_readonly(*pool_authority, false),    // Authority (not signer)
            AccountMeta::new(*amm, false),                        // AMM market account
            AccountMeta::new_readonly(*user, true),               // User transfer authority (signer)
            AccountMeta::new(user_source_token, false),           // User's source token account
            AccountMeta::new(user_destination_token, false),      // User's destination token account
            AccountMeta::new(*pool_source_token, false),          // Pool's source token account (passed in)
            AccountMeta::new(*pool_destination_token, false),     // Pool's destination token account (passed in)
            AccountMeta::new(*pool_mint, false),                  // Pool LP token mint
            AccountMeta::new(*fee_account, false),                // Fee account
            AccountMeta::new_readonly(spl_token::id(), false),    // Token program
            AccountMeta::new_readonly(*oracle_main, false),       // Oracle main account
            AccountMeta::new_readonly(*oracle_sub, false),        // Oracle sub account
            AccountMeta::new_readonly(*oracle_pc, false),         // Oracle pc account
        ],
        data: create_lifinity_v2_instruction_data(amount_in, minimum_amount_out),
    }
}