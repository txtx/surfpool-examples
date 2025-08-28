use crate::prelude::*;
use crate::constants::PERPETUALS_PROGRAM;

fn create_perpetuals_swap_data(amount_in: u64, min_amount_out: u64) -> Vec<u8> {
    let mut buffer = Vec::with_capacity(24);
    // PERPETUALS_SWAP_SELECTOR (8 bytes)
    buffer.extend_from_slice(&[0x24, 0x8c, 0xc9, 0x75, 0xa2, 0x4c, 0x1d, 0x31]); // Example selector
    buffer.extend_from_slice(&amount_in.to_le_bytes());
    buffer.extend_from_slice(&min_amount_out.to_le_bytes());
    buffer
}

fn create_perpetuals_add_liquidity_data(amount_in: u64, min_lp_amount_out: u64, token_amount_pre_swap: u8) -> Vec<u8> {
    let mut buffer = Vec::with_capacity(25);
    // PERPETUALS_ADDLIQ_SELECTOR (8 bytes)
    buffer.extend_from_slice(&[0x18, 0x7d, 0xe4, 0x92, 0x6f, 0xa1, 0x2b, 0x48]); // Example selector
    buffer.extend_from_slice(&amount_in.to_le_bytes());
    buffer.extend_from_slice(&min_lp_amount_out.to_le_bytes());
    buffer.push(token_amount_pre_swap);
    buffer
}

fn create_perpetuals_remove_liquidity_data(lp_amount_in: u64, min_amount_out: u64) -> Vec<u8> {
    let mut buffer = Vec::with_capacity(24);
    // PERPETUALS_REMOVELIQ_SELECTOR (8 bytes)
    buffer.extend_from_slice(&[0x3c, 0x91, 0x2f, 0x67, 0x8a, 0x52, 0x4e, 0x1d]); // Example selector
    buffer.extend_from_slice(&lp_amount_in.to_le_bytes());
    buffer.extend_from_slice(&min_amount_out.to_le_bytes());
    buffer
}

pub fn create_perpetuals_swap_ix(
    owner: &Pubkey,
    funding_account: &Pubkey,
    receiving_account: &Pubkey,
    perpetuals_vault_authority: &Pubkey,
    perpetuals_state: &Pubkey,
    perpetuals_pool: &Pubkey,
    receiving_custody: &Pubkey,
    receiving_custody_doves_price_account: &Pubkey,
    receiving_custody_pythnet_price_account: &Pubkey,
    receiving_custody_token_account: &Pubkey,
    dispensing_custody: &Pubkey,
    dispensing_custody_doves_price_account: &Pubkey,
    dispensing_custody_pythnet_price_account: &Pubkey,
    dispensing_custody_token_account: &Pubkey,
    event_authority: &Pubkey,
    amount_in: u64,
) -> Instruction {
    tracing::debug!("Creating Perpetuals swap instruction with the following details:");
    tracing::debug!("  Program ID: {}", PERPETUALS_PROGRAM);
    tracing::debug!("  Accounts:");
    tracing::debug!("    1. owner: {} (signer)", owner);
    tracing::debug!("    2. funding_account: {} (mutable)", funding_account);
    tracing::debug!("    3. receiving_account: {} (mutable)", receiving_account);
    tracing::debug!("    4. perpetuals_vault_authority: {} (readonly)", perpetuals_vault_authority);
    tracing::debug!("    5. perpetuals_state: {} (readonly)", perpetuals_state);
    tracing::debug!("    6. perpetuals_pool: {} (mutable)", perpetuals_pool);
    tracing::debug!("    7. receiving_custody: {} (mutable)", receiving_custody);
    tracing::debug!("    8. receiving_custody_doves_price_account: {} (readonly)", receiving_custody_doves_price_account);
    tracing::debug!("    9. receiving_custody_pythnet_price_account: {} (readonly)", receiving_custody_pythnet_price_account);
    tracing::debug!("    10. receiving_custody_token_account: {} (mutable)", receiving_custody_token_account);
    tracing::debug!("    11. dispensing_custody: {} (mutable)", dispensing_custody);
    tracing::debug!("    12. dispensing_custody_doves_price_account: {} (readonly)", dispensing_custody_doves_price_account);
    tracing::debug!("    13. dispensing_custody_pythnet_price_account: {} (readonly)", dispensing_custody_pythnet_price_account);
    tracing::debug!("    14. dispensing_custody_token_account: {} (mutable)", dispensing_custody_token_account);
    tracing::debug!("    15. token_program: {} (readonly)", spl_token::id());
    tracing::debug!("    16. event_authority: {} (readonly)", event_authority);
    tracing::debug!("    17. program_id: {} (readonly)", PERPETUALS_PROGRAM);
    
    tracing::debug!("  Args:");
    tracing::debug!("    amount_in: {}", amount_in);
    tracing::debug!("    min_amount_out: 1");
    
    Instruction {
        program_id: PERPETUALS_PROGRAM,
        accounts: vec![
            AccountMeta::new(*owner, true),
            AccountMeta::new(*funding_account, false),
            AccountMeta::new(*receiving_account, false),
            AccountMeta::new_readonly(*perpetuals_vault_authority, false),
            AccountMeta::new_readonly(*perpetuals_state, false),
            AccountMeta::new(*perpetuals_pool, false),
            AccountMeta::new(*receiving_custody, false),
            AccountMeta::new_readonly(*receiving_custody_doves_price_account, false),
            AccountMeta::new_readonly(*receiving_custody_pythnet_price_account, false),
            AccountMeta::new(*receiving_custody_token_account, false),
            AccountMeta::new(*dispensing_custody, false),
            AccountMeta::new_readonly(*dispensing_custody_doves_price_account, false),
            AccountMeta::new_readonly(*dispensing_custody_pythnet_price_account, false),
            AccountMeta::new(*dispensing_custody_token_account, false),
            AccountMeta::new_readonly(spl_token::id(), false),
            AccountMeta::new_readonly(*event_authority, false),
            AccountMeta::new_readonly(PERPETUALS_PROGRAM, false),
        ],
        data: create_perpetuals_swap_data(amount_in, 1),
    }
}

pub fn create_perpetuals_add_liquidity_ix(
    owner: &Pubkey,
    funding_account: &Pubkey,
    jlp_token_account: &Pubkey,
    perpetuals_vault_authority: &Pubkey,
    perpetuals_state: &Pubkey,
    perpetuals_pool: &Pubkey,
    collateral_custody: &Pubkey,
    doves_price_account: &Pubkey,
    pythnet_price_account: &Pubkey,
    custody_token_account: &Pubkey,
    jlp_mint: &Pubkey,
    event_authority: &Pubkey,
    remaining_accounts: &[&Pubkey], // 15 additional accounts (5 custody + 5 doves + 5 pythnet)
    amount_in: u64,
) -> Instruction {
    tracing::debug!("Creating Perpetuals add liquidity instruction with the following details:");
    tracing::debug!("  Program ID: {}", PERPETUALS_PROGRAM);
    tracing::debug!("  Accounts:");
    tracing::debug!("    1. owner: {} (signer)", owner);
    tracing::debug!("    2. funding_account: {} (mutable)", funding_account);
    tracing::debug!("    3. jlp_token_account: {} (mutable)", jlp_token_account);
    tracing::debug!("    4. perpetuals_vault_authority: {} (readonly)", perpetuals_vault_authority);
    tracing::debug!("    5. perpetuals_state: {} (readonly)", perpetuals_state);
    tracing::debug!("    6. perpetuals_pool: {} (mutable)", perpetuals_pool);
    tracing::debug!("    7. collateral_custody: {} (mutable)", collateral_custody);
    tracing::debug!("    8. doves_price_account: {} (readonly)", doves_price_account);
    tracing::debug!("    9. pythnet_price_account: {} (readonly)", pythnet_price_account);
    tracing::debug!("    10. custody_token_account: {} (mutable)", custody_token_account);
    tracing::debug!("    11. jlp_mint: {} (mutable)", jlp_mint);
    tracing::debug!("    12. token_program: {} (readonly)", spl_token::id());
    tracing::debug!("    13. event_authority: {} (readonly)", event_authority);
    tracing::debug!("    14. program_id: {} (readonly)", PERPETUALS_PROGRAM);
    
    let mut accounts = vec![
        AccountMeta::new(*owner, true),
        AccountMeta::new(*funding_account, false),
        AccountMeta::new(*jlp_token_account, false),
        AccountMeta::new_readonly(*perpetuals_vault_authority, false),
        AccountMeta::new_readonly(*perpetuals_state, false),
        AccountMeta::new(*perpetuals_pool, false),
        AccountMeta::new(*collateral_custody, false),
        AccountMeta::new_readonly(*doves_price_account, false),
        AccountMeta::new_readonly(*pythnet_price_account, false),
        AccountMeta::new(*custody_token_account, false),
        AccountMeta::new(*jlp_mint, false),
        AccountMeta::new_readonly(spl_token::id(), false),
        AccountMeta::new_readonly(*event_authority, false),
        AccountMeta::new_readonly(PERPETUALS_PROGRAM, false),
    ];
    
    // Add remaining accounts (custody and price accounts)
    for (i, account) in remaining_accounts.iter().enumerate() {
        if **account == *collateral_custody {
            accounts.push(AccountMeta::new(**account, false));
        } else {
            accounts.push(AccountMeta::new_readonly(**account, false));
        }
        tracing::debug!("    {}. remaining_account[{}]: {} ({})", 15 + i, i, account, 
                       if **account == *collateral_custody { "mutable" } else { "readonly" });
    }
    
    tracing::debug!("  Args:");
    tracing::debug!("    amount_in: {}", amount_in);
    tracing::debug!("    min_lp_amount_out: 1");
    tracing::debug!("    token_amount_pre_swap: 0");
    
    Instruction {
        program_id: PERPETUALS_PROGRAM,
        accounts,
        data: create_perpetuals_add_liquidity_data(amount_in, 1, 0),
    }
}

pub fn create_perpetuals_remove_liquidity_ix(
    owner: &Pubkey,
    jlp_token_account: &Pubkey,
    receiving_account: &Pubkey,
    perpetuals_vault_authority: &Pubkey,
    perpetuals_state: &Pubkey,
    perpetuals_pool: &Pubkey,
    collateral_custody: &Pubkey,
    doves_price_account: &Pubkey,
    pythnet_price_account: &Pubkey,
    custody_token_account: &Pubkey,
    jlp_mint: &Pubkey,
    event_authority: &Pubkey,
    remaining_accounts: &[&Pubkey], // 15 additional accounts
    lp_amount_in: u64,
) -> Instruction {
    tracing::debug!("Creating Perpetuals remove liquidity instruction with the following details:");
    tracing::debug!("  Program ID: {}", PERPETUALS_PROGRAM);
    tracing::debug!("  Accounts:");
    tracing::debug!("    1. owner: {} (signer)", owner);
    tracing::debug!("    2. jlp_token_account: {} (mutable)", jlp_token_account);
    tracing::debug!("    3. receiving_account: {} (mutable)", receiving_account);
    tracing::debug!("    4. perpetuals_vault_authority: {} (readonly)", perpetuals_vault_authority);
    tracing::debug!("    5. perpetuals_state: {} (readonly)", perpetuals_state);
    tracing::debug!("    6. perpetuals_pool: {} (mutable)", perpetuals_pool);
    tracing::debug!("    7. collateral_custody: {} (mutable)", collateral_custody);
    tracing::debug!("    8. doves_price_account: {} (readonly)", doves_price_account);
    tracing::debug!("    9. pythnet_price_account: {} (readonly)", pythnet_price_account);
    tracing::debug!("    10. custody_token_account: {} (mutable)", custody_token_account);
    tracing::debug!("    11. jlp_mint: {} (mutable)", jlp_mint);
    tracing::debug!("    12. token_program: {} (readonly)", spl_token::id());
    tracing::debug!("    13. event_authority: {} (readonly)", event_authority);
    tracing::debug!("    14. program_id: {} (readonly)", PERPETUALS_PROGRAM);
    
    let mut accounts = vec![
        AccountMeta::new(*owner, true),
        AccountMeta::new(*jlp_token_account, false),
        AccountMeta::new(*receiving_account, false),
        AccountMeta::new_readonly(*perpetuals_vault_authority, false),
        AccountMeta::new_readonly(*perpetuals_state, false),
        AccountMeta::new(*perpetuals_pool, false),
        AccountMeta::new(*collateral_custody, false),
        AccountMeta::new_readonly(*doves_price_account, false),
        AccountMeta::new_readonly(*pythnet_price_account, false),
        AccountMeta::new(*custody_token_account, false),
        AccountMeta::new(*jlp_mint, false),
        AccountMeta::new_readonly(spl_token::id(), false),
        AccountMeta::new_readonly(*event_authority, false),
        AccountMeta::new_readonly(PERPETUALS_PROGRAM, false),
    ];
    
    // Add remaining accounts
    for (i, account) in remaining_accounts.iter().enumerate() {
        if **account == *collateral_custody {
            accounts.push(AccountMeta::new(**account, false));
        } else {
            accounts.push(AccountMeta::new_readonly(**account, false));
        }
        tracing::debug!("    {}. remaining_account[{}]: {} ({})", 15 + i, i, account,
                       if **account == *collateral_custody { "mutable" } else { "readonly" });
    }
    
    tracing::debug!("  Args:");
    tracing::debug!("    lp_amount_in: {}", lp_amount_in);
    tracing::debug!("    min_amount_out: 1");
    
    Instruction {
        program_id: PERPETUALS_PROGRAM,
        accounts,
        data: create_perpetuals_remove_liquidity_data(lp_amount_in, 1),
    }
}