use crate::prelude::*;
use crate::constants::{SABER_STABLE_PROGRAM};

fn create_instruction_data(amount_in: u64) -> Vec<u8> {
    let mut buffer = Vec::with_capacity(17);
    buffer.push(1); // Swap discriminator
    buffer.extend_from_slice(&amount_in.to_le_bytes());
    buffer.extend_from_slice(&1u64.to_le_bytes()); // min_amount_out
    buffer
}

/// Validates and determines the correct pool token accounts based on source/destination tokens
fn validate_and_get_pool_accounts(
    source_mint: &Pubkey,
    destination_mint: &Pubkey,
    token_a_mint: &Pubkey,
    token_b_mint: &Pubkey,
    token_a_account: &Pubkey,
    token_b_account: &Pubkey,
) -> Result<(Pubkey, Pubkey), &'static str> {
    if (source_mint == token_a_mint) && (destination_mint == token_b_mint) {
        Ok((*token_a_account, *token_b_account))
    } else if (source_mint == token_b_mint) && (destination_mint == token_a_mint) {
        Ok((*token_b_account, *token_a_account))
    } else {
        Err("Invalid token mint configuration for pool")
    }
}

pub fn create_saber_stable_swap_ix(
    user: &Pubkey,
    user_source_token: &Pubkey,
    user_destination_token: &Pubkey,
    source_mint: &Pubkey,
    destination_mint: &Pubkey,
    swap_info: &Pubkey,
    swap_authority: &Pubkey,
    token_a_account: &Pubkey,
    token_b_account: &Pubkey,
    token_a_mint: &Pubkey,
    token_b_mint: &Pubkey,
    admin_fee_account: &Pubkey,
    amount: u64,
) -> Instruction {
    tracing::debug!("Creating Saber Stable swap instruction with the following details:");
    tracing::debug!("  Program ID: {}", SABER_STABLE_PROGRAM);
    tracing::debug!("  Accounts:");
    tracing::debug!("    1. swap_info: {} (readonly)", swap_info);
    tracing::debug!("    2. swap_authority: {} (readonly)", swap_authority);
    tracing::debug!("    3. user: {} (signer)", user);
    tracing::debug!("    4. user_source_token: {} (mutable)", user_source_token);
    tracing::debug!("    5. token_a_account: {} (mutable)", token_a_account);
    tracing::debug!("    6. token_b_account: {} (mutable)", token_b_account);
    tracing::debug!("    7. user_destination_token: {} (mutable)", user_destination_token);
    tracing::debug!("    8. admin_fee_account: {} (mutable)", admin_fee_account);
    tracing::debug!("    9. token_program: {} (readonly)", spl_token::id());

    // Validate pool configuration and get correct pool accounts
    let (pool_source_account, pool_destination_account) = validate_and_get_pool_accounts(
        source_mint,
        destination_mint,
        token_a_mint,
        token_b_mint,
        token_a_account,
        token_b_account,
    ).expect("Invalid pool token configuration");

    tracing::debug!("  Pool Mapping:");
    tracing::debug!("    pool_source_account: {}", pool_source_account);
    tracing::debug!("    pool_destination_account: {}", pool_destination_account);

    tracing::debug!("  Args:");
    tracing::debug!("    discriminator: 1 (swap)");
    tracing::debug!("    amount: {}", amount);
    tracing::debug!("    min_amount_out: 1");

    Instruction {
        program_id: SABER_STABLE_PROGRAM,
        accounts: vec![
            AccountMeta::new_readonly(*swap_info, false),
            AccountMeta::new_readonly(*swap_authority, false),
            AccountMeta::new_readonly(*user, true),
            AccountMeta::new(*user_source_token, false),
            AccountMeta::new(pool_source_account, false),
            AccountMeta::new(pool_destination_account, false),
            AccountMeta::new(*user_destination_token, false),
            AccountMeta::new(*admin_fee_account, false),
            AccountMeta::new_readonly(spl_token::id(), false),
        ],
        data: create_instruction_data(amount),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_create_instruction_data() {
        let amount_in = 100u64;
        let data = create_instruction_data(amount_in);
        
        assert_eq!(data.len(), 17);
        assert_eq!(data[0], 1); // Swap discriminator
        
        let amount_bytes = &data[1..9];
        let parsed_amount = u64::from_le_bytes(amount_bytes.try_into().unwrap());
        assert_eq!(parsed_amount, amount_in);
        
        let min_amount_bytes = &data[9..17];
        let parsed_min_amount = u64::from_le_bytes(min_amount_bytes.try_into().unwrap());
        assert_eq!(parsed_min_amount, 1u64);
    }
}