use crate::prelude::*;
use crate::constants::{OBRIC_V2_PROGRAM};

fn create_instruction_data(discriminator: &[u8], x_to_y: bool, amount_in: u64, min_amount_out: u64) -> Vec<u8> {
    let mut buffer = Vec::with_capacity(25);
    buffer.extend_from_slice(discriminator);
    buffer.push(x_to_y as u8);
    buffer.extend_from_slice(&amount_in.to_le_bytes());
    buffer.extend_from_slice(&min_amount_out.to_le_bytes());
    buffer
}

/// Validates token mint compatibility and determines swap direction
fn validate_and_determine_direction(
    user_source_mint: &Pubkey,
    user_destination_mint: &Pubkey,
    reserve_x_mint: &Pubkey,
    reserve_y_mint: &Pubkey,
) -> Result<(bool, Pubkey, Pubkey), &'static str> {
    let x_to_y = if user_destination_mint == reserve_x_mint && user_source_mint == reserve_y_mint {
        false
    } else if user_destination_mint == reserve_y_mint && user_source_mint == reserve_x_mint {
        true
    } else {
        return Err("Invalid token mint configuration");
    };

    let (user_token_account_x, user_token_account_y) = if user_source_mint == reserve_x_mint
        && user_destination_mint == reserve_y_mint
    {
        (*user_source_mint, *user_destination_mint)
    } else if user_source_mint == reserve_y_mint && user_destination_mint == reserve_x_mint {
        (*user_destination_mint, *user_source_mint)
    } else {
        return Err("Invalid token mint configuration");
    };

    Ok((x_to_y, user_token_account_x, user_token_account_y))
}

pub fn create_obric_v2_swap_ix(
    trading_pair: &Pubkey,
    user: &Pubkey,
    user_source_token: &Pubkey,
    user_destination_token: &Pubkey,
    user_source_mint: &Pubkey,
    user_destination_mint: &Pubkey,
    second_reference_oracle: &Pubkey,
    third_reference_oracle: &Pubkey,
    reserve_x: &Pubkey,
    reserve_y: &Pubkey,
    reserve_x_mint: &Pubkey,
    reserve_y_mint: &Pubkey,
    reference_oracle: &Pubkey,
    x_price_feed: &Pubkey,
    y_price_feed: &Pubkey,
    token_program: &Pubkey,
    amount: u64,
) -> Result<Instruction, &'static str> {
    tracing::debug!("Creating Obric V2 swap instruction with the following details:");
    tracing::debug!("  Program ID: {}", OBRIC_V2_PROGRAM);
    
    // Validate token mints and determine swap direction
    let (x_to_y, user_token_account_x, user_token_account_y) = validate_and_determine_direction(
        user_source_mint,
        user_destination_mint,
        reserve_x_mint,
        reserve_y_mint,
    )?;

    tracing::debug!("  Swap Direction: {} (x_to_y: {})", 
        if x_to_y { "X -> Y" } else { "Y -> X" }, x_to_y);
    
    tracing::debug!("  Accounts:");
    tracing::debug!("    1. trading_pair: {} (mutable)", trading_pair);
    tracing::debug!("    2. second_reference_oracle: {} (readonly)", second_reference_oracle);
    tracing::debug!("    3. third_reference_oracle: {} (readonly)", third_reference_oracle);
    tracing::debug!("    4. reserve_x: {} (mutable)", reserve_x);
    tracing::debug!("    5. reserve_y: {} (mutable)", reserve_y);
    tracing::debug!("    6. user_token_account_x: {} (mutable)", user_token_account_x);
    tracing::debug!("    7. user_token_account_y: {} (mutable)", user_token_account_y);
    tracing::debug!("    8. reference_oracle: {} (mutable)", reference_oracle);
    tracing::debug!("    9. x_price_feed: {} (readonly)", x_price_feed);
    tracing::debug!("    10. y_price_feed: {} (readonly)", y_price_feed);
    tracing::debug!("    11. user: {} (signer)", user);
    tracing::debug!("    12. token_program: {} (readonly)", token_program);

    tracing::debug!("  Token Validation:");
    tracing::debug!("    user_source_mint: {}", user_source_mint);
    tracing::debug!("    user_destination_mint: {}", user_destination_mint);
    tracing::debug!("    reserve_x_mint: {}", reserve_x_mint);
    tracing::debug!("    reserve_y_mint: {}", reserve_y_mint);

    tracing::debug!("  Args:");
    tracing::debug!("    amount: {}", amount);
    tracing::debug!("    x_to_y: {}", x_to_y);

    let swap2_selector = &[0x71, 0x4f, 0x3b, 0xe5, 0x3d, 0x7f, 0x6a, 0x4c]; // SWAP2_SELECTOR
    
    Ok(Instruction {
        program_id: OBRIC_V2_PROGRAM,
        accounts: vec![
            AccountMeta::new(*trading_pair, false),
            AccountMeta::new_readonly(*second_reference_oracle, false),
            AccountMeta::new_readonly(*third_reference_oracle, false),
            AccountMeta::new(*reserve_x, false),
            AccountMeta::new(*reserve_y, false),
            AccountMeta::new(*user_source_token, false),
            AccountMeta::new(*user_destination_token, false),
            AccountMeta::new(*reference_oracle, false),
            AccountMeta::new_readonly(*x_price_feed, false),
            AccountMeta::new_readonly(*y_price_feed, false),
            AccountMeta::new_readonly(*user, true),
            AccountMeta::new_readonly(*token_program, false),
        ],
        data: create_instruction_data(swap2_selector, x_to_y, amount, 1),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_and_determine_direction() {
        let mint_a = Pubkey::new_unique();
        let mint_b = Pubkey::new_unique();
        
        // Test x_to_y direction (source is reserve_x, destination is reserve_y)
        let (x_to_y, _, _) = validate_and_determine_direction(&mint_a, &mint_b, &mint_a, &mint_b)
            .expect("Should validate successfully");
        assert!(x_to_y);
        
        // Test y_to_x direction (source is reserve_y, destination is reserve_x)
        let (x_to_y, _, _) = validate_and_determine_direction(&mint_b, &mint_a, &mint_a, &mint_b)
            .expect("Should validate successfully");
        assert!(!x_to_y);
        
        // Test invalid configuration
        let mint_c = Pubkey::new_unique();
        let result = validate_and_determine_direction(&mint_c, &mint_b, &mint_a, &mint_b);
        assert!(result.is_err());
    }

    #[test]
    fn test_instruction_data_creation() {
        let swap2_selector = &[0x71, 0x4f, 0x3b, 0xe5, 0x3d, 0x7f, 0x6a, 0x4c];
        let amount_in = 100u64;
        let x_to_y = true;
        
        let data = create_instruction_data(swap2_selector, x_to_y, amount_in, 1);
        
        assert_eq!(data.len(), 25);
        assert_eq!(&data[0..8], swap2_selector);
        assert_eq!(data[8], x_to_y as u8);
        assert_eq!(&data[9..17], &amount_in.to_le_bytes());
        assert_eq!(&data[17..25], &1u64.to_le_bytes());
    }
}