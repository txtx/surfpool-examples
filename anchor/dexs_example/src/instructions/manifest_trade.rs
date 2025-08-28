use crate::prelude::*;
use crate::constants::{MANIFEST_PROGRAM, MANIFEST_SWAP_SELECTOR};
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshDeserialize, BorshSerialize)]
pub struct SwapParams {
    pub in_atoms: u64,
    pub out_atoms: u64,
    pub is_base_in: bool,
    pub is_exact_in: bool,
}

fn create_manifest_instruction_data(amount_in: u64, is_base_in: bool) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let swap_params = SwapParams {
        in_atoms: amount_in,
        out_atoms: 1, // Minimum output, let the market decide actual output
        is_base_in,
        is_exact_in: true, // Exact input mode
    };
    
    let mut data = Vec::with_capacity(19);
    data.extend_from_slice(&MANIFEST_SWAP_SELECTOR);
    data.extend_from_slice(&swap_params.try_to_vec()?);
    Ok(data)
}

pub fn create_manifest_swap_ix(
    market: &Pubkey,
    user: &Pubkey,
    base_vault: &Pubkey,
    quote_vault: &Pubkey,
    base_mint: &Pubkey,
    quote_mint: &Pubkey,
    user_base_token: &Pubkey,
    user_quote_token: &Pubkey,
    token_program_base: &Pubkey,
    token_program_quote: &Pubkey,
    global: &Pubkey,
    global_vault: &Pubkey,
    amount: u64,
    is_base_in: bool, // true if selling base to buy quote, false if selling quote to buy base
) -> Result<Instruction, Box<dyn std::error::Error>> {
    // Print detailed debug information about the instruction we're creating
    tracing::debug!("Creating Manifest swap instruction with the following details:");
    tracing::debug!("  Program ID: {}", MANIFEST_PROGRAM);
    tracing::debug!("  Accounts:");
    tracing::debug!("    1. user: {} (signer)", user);
    tracing::debug!("    2. market: {} (mutable)", market);
    tracing::debug!("    3. system_program: {} (readonly)", solana_program::system_program::id());
    tracing::debug!("    4. user_base_token: {} (mutable)", user_base_token);
    tracing::debug!("    5. user_quote_token: {} (mutable)", user_quote_token);
    tracing::debug!("    6. base_vault: {} (mutable)", base_vault);
    tracing::debug!("    7. quote_vault: {} (mutable)", quote_vault);
    tracing::debug!("    8. token_program_base: {} (readonly)", token_program_base);
    tracing::debug!("    9. base_mint: {} (readonly)", base_mint);
    tracing::debug!("    10. token_program_quote: {} (readonly)", token_program_quote);
    tracing::debug!("    11. quote_mint: {} (readonly)", quote_mint);
    tracing::debug!("    12. global: {} (mutable)", global);
    tracing::debug!("    13. global_vault: {} (mutable)", global_vault);

    tracing::debug!("  Args:");
    tracing::debug!("    amount: {}", amount);
    tracing::debug!("    is_base_in: {}", is_base_in);

    Ok(Instruction {
        program_id: MANIFEST_PROGRAM,
        accounts: vec![
            AccountMeta::new(*user, true), // Payer/signer
            AccountMeta::new(*market, false),
            AccountMeta::new_readonly(solana_program::system_program::id(), false),
            AccountMeta::new(*user_base_token, false), // trader_base
            AccountMeta::new(*user_quote_token, false), // trader_quote
            AccountMeta::new(*base_vault, false),
            AccountMeta::new(*quote_vault, false),
            AccountMeta::new_readonly(*token_program_base, false),
            AccountMeta::new_readonly(*base_mint, false),
            AccountMeta::new_readonly(*token_program_quote, false),
            AccountMeta::new_readonly(*quote_mint, false),
            AccountMeta::new(*global, false),
            AccountMeta::new(*global_vault, false),
        ],
        data: create_manifest_instruction_data(amount, is_base_in)?,
    })
}

// Helper function to determine swap direction based on source token mint
pub fn determine_manifest_swap_direction(
    source_mint: &Pubkey,
    base_mint: &Pubkey,
    quote_mint: &Pubkey,
) -> Result<bool, &'static str> {
    if source_mint == base_mint {
        Ok(true) // Selling base to buy quote
    } else if source_mint == quote_mint {
        Ok(false) // Selling quote to buy base
    } else {
        Err("Source token mint doesn't match base or quote mint")
    }
}

// Helper function to validate destination token mint matches expected output
pub fn validate_manifest_destination_mint(
    destination_mint: &Pubkey,
    is_base_in: bool,
    base_mint: &Pubkey,
    quote_mint: &Pubkey,
) -> Result<(), &'static str> {
    let expected_destination_mint = if is_base_in {
        quote_mint // base->quote, destination should be quote
    } else {
        base_mint // quote->base, destination should be base
    };

    if destination_mint != expected_destination_mint {
        return Err("Destination token mint doesn't match expected output mint");
    }
    Ok(())
}

// Validation helper function
pub fn validate_manifest_program_id(program_id: &Pubkey) -> Result<(), &'static str> {
    if program_id != &MANIFEST_PROGRAM {
        return Err("Invalid Manifest program ID");
    }
    Ok(())
}

// Helper function to order user token accounts correctly (base first, quote second)
pub fn order_manifest_user_tokens(
    source_token: &Pubkey,
    destination_token: &Pubkey,
    source_mint: &Pubkey,
    base_mint: &Pubkey,
) -> (Pubkey, Pubkey) {
    let is_source_base = source_mint == base_mint;
    
    if is_source_base {
        (*source_token, *destination_token) // (base, quote)
    } else {
        (*destination_token, *source_token) // (base, quote)
    }
}