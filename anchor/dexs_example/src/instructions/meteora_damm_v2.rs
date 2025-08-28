use crate::prelude::*;
use crate::constants::{METEORA_DYNAMIC_POOL_PROGRAM, METEORA_VAULT_PROGRAM, METEORA_DLMM_PROGRAM, METEORA_DAMM_V2_PROGRAM};

fn create_instruction_data(discriminator: &[u8], amount_in: u64, min_amount_out: u64) -> Vec<u8> {
    let mut buffer = Vec::with_capacity(24);
    buffer.extend_from_slice(discriminator);
    buffer.extend_from_slice(&amount_in.to_le_bytes());
    buffer.extend_from_slice(&min_amount_out.to_le_bytes());
    buffer
}

fn create_vault_instruction_data(discriminator: &[u8], amount_in: u64, min_amount_out: u64) -> Vec<u8> {
    let mut buffer = Vec::with_capacity(24);
    buffer.extend_from_slice(discriminator);
    buffer.extend_from_slice(&amount_in.to_le_bytes());
    buffer.extend_from_slice(&min_amount_out.to_le_bytes());
    buffer
}

fn create_dlmm_swap2_instruction_data(discriminator: &[u8], amount_in: u64, min_amount_out: u64) -> Vec<u8> {
    let mut buffer = Vec::with_capacity(28);
    buffer.extend_from_slice(discriminator);
    buffer.extend_from_slice(&amount_in.to_le_bytes());
    buffer.extend_from_slice(&min_amount_out.to_le_bytes());
    buffer.extend_from_slice(&0u32.to_le_bytes()); // Additional field for swap2
    buffer
}

// Meteora Dynamic Pool Swap
pub fn create_meteora_dynamic_pool_swap_ix(
    pool: &Pubkey,
    user: &Pubkey,
    user_source_token: &Pubkey,
    user_destination_token: &Pubkey,
    a_vault: &Pubkey,
    b_vault: &Pubkey,
    a_token_vault: &Pubkey,
    b_token_vault: &Pubkey,
    a_vault_lp_mint: &Pubkey,
    b_vault_lp_mint: &Pubkey,
    a_vault_lp: &Pubkey,
    b_vault_lp: &Pubkey,
    admin_token_fee: &Pubkey,
    vault_program: &Pubkey,
    token_program: &Pubkey,
    amount: u64,
) -> Instruction {
    tracing::debug!("Creating Meteora Dynamic Pool swap instruction with the following details:");
    tracing::debug!("  Program ID: {}", METEORA_DYNAMIC_POOL_PROGRAM);
    tracing::debug!("  Accounts:");
    tracing::debug!("    1. pool: {} (mutable)", pool);
    tracing::debug!("    2. user_source_token: {} (mutable)", user_source_token);
    tracing::debug!("    3. user_destination_token: {} (mutable)", user_destination_token);
    tracing::debug!("    4. a_vault: {} (mutable)", a_vault);
    tracing::debug!("    5. b_vault: {} (mutable)", b_vault);
    tracing::debug!("    6. a_token_vault: {} (mutable)", a_token_vault);
    tracing::debug!("    7. b_token_vault: {} (mutable)", b_token_vault);
    tracing::debug!("    8. a_vault_lp_mint: {} (mutable)", a_vault_lp_mint);
    tracing::debug!("    9. b_vault_lp_mint: {} (mutable)", b_vault_lp_mint);
    tracing::debug!("    10. a_vault_lp: {} (mutable)", a_vault_lp);
    tracing::debug!("    11. b_vault_lp: {} (mutable)", b_vault_lp);
    tracing::debug!("    12. admin_token_fee: {} (mutable)", admin_token_fee);
    tracing::debug!("    13. user: {} (signer)", user);
    tracing::debug!("    14. vault_program: {} (readonly)", vault_program);
    tracing::debug!("    15. token_program: {} (readonly)", token_program);

    tracing::debug!("  Args:");
    tracing::debug!("    amount: {}", amount);

    let swap_selector = &[0x60, 0x3e, 0x2a, 0xd4, 0x2c, 0x6e, 0x5a, 0x3b]; // SWAP_SELECTOR
    
    Instruction {
        program_id: METEORA_DYNAMIC_POOL_PROGRAM,
        accounts: vec![
            AccountMeta::new(*pool, false),
            AccountMeta::new(*user_source_token, false),
            AccountMeta::new(*user_destination_token, false),
            AccountMeta::new(*a_vault, false),
            AccountMeta::new(*b_vault, false),
            AccountMeta::new(*a_token_vault, false),
            AccountMeta::new(*b_token_vault, false),
            AccountMeta::new(*a_vault_lp_mint, false),
            AccountMeta::new(*b_vault_lp_mint, false),
            AccountMeta::new(*a_vault_lp, false),
            AccountMeta::new(*b_vault_lp, false),
            AccountMeta::new(*admin_token_fee, false),
            AccountMeta::new_readonly(*user, true),
            AccountMeta::new_readonly(*vault_program, false),
            AccountMeta::new_readonly(*token_program, false),
        ],
        data: create_instruction_data(swap_selector, amount, 1),
    }
}

// Meteora LST Pool Swap (with additional LST account)
pub fn create_meteora_lst_pool_swap_ix(
    pool: &Pubkey,
    user: &Pubkey,
    user_source_token: &Pubkey,
    user_destination_token: &Pubkey,
    a_vault: &Pubkey,
    b_vault: &Pubkey,
    a_token_vault: &Pubkey,
    b_token_vault: &Pubkey,
    a_vault_lp_mint: &Pubkey,
    b_vault_lp_mint: &Pubkey,
    a_vault_lp: &Pubkey,
    b_vault_lp: &Pubkey,
    admin_token_fee: &Pubkey,
    vault_program: &Pubkey,
    token_program: &Pubkey,
    lst: &Pubkey,
    amount: u64,
) -> Instruction {
    tracing::debug!("Creating Meteora LST Pool swap instruction with the following details:");
    tracing::debug!("  Program ID: {}", METEORA_DYNAMIC_POOL_PROGRAM);
    tracing::debug!("  Accounts (LST version):");
    tracing::debug!("    16. lst: {} (readonly)", lst);

    let swap_selector = &[0x60, 0x3e, 0x2a, 0xd4, 0x2c, 0x6e, 0x5a, 0x3b]; // SWAP_SELECTOR
    
    let mut accounts = vec![
        AccountMeta::new(*pool, false),
        AccountMeta::new(*user_source_token, false),
        AccountMeta::new(*user_destination_token, false),
        AccountMeta::new(*a_vault, false),
        AccountMeta::new(*b_vault, false),
        AccountMeta::new(*a_token_vault, false),
        AccountMeta::new(*b_token_vault, false),
        AccountMeta::new(*a_vault_lp_mint, false),
        AccountMeta::new(*b_vault_lp_mint, false),
        AccountMeta::new(*a_vault_lp, false),
        AccountMeta::new(*b_vault_lp, false),
        AccountMeta::new(*admin_token_fee, false),
        AccountMeta::new_readonly(*user, true),
        AccountMeta::new_readonly(*vault_program, false),
        AccountMeta::new_readonly(*token_program, false),
        AccountMeta::new_readonly(*lst, false),
    ];
    
    Instruction {
        program_id: METEORA_DYNAMIC_POOL_PROGRAM,
        accounts,
        data: create_instruction_data(swap_selector, amount, 1),
    }
}

// Meteora Vault Deposit
pub fn create_meteora_vault_deposit_ix(
    vault: &Pubkey,
    user: &Pubkey,
    user_source_token: &Pubkey,
    user_destination_token: &Pubkey,
    token_vault: &Pubkey,
    lp_mint: &Pubkey,
    token_program: &Pubkey,
    amount: u64,
) -> Instruction {
    tracing::debug!("Creating Meteora Vault deposit instruction with the following details:");
    tracing::debug!("  Program ID: {}", METEORA_VAULT_PROGRAM);
    tracing::debug!("  Accounts:");
    tracing::debug!("    1. vault: {} (mutable)", vault);
    tracing::debug!("    2. token_vault: {} (mutable)", token_vault);
    tracing::debug!("    3. lp_mint: {} (mutable)", lp_mint);
    tracing::debug!("    4. user_source_token: {} (mutable)", user_source_token);
    tracing::debug!("    5. user_destination_token: {} (mutable)", user_destination_token);
    tracing::debug!("    6. user: {} (signer)", user);
    tracing::debug!("    7. token_program: {} (readonly)", token_program);

    tracing::debug!("  Args:");
    tracing::debug!("    amount: {}", amount);

    let deposit_selector = &[0xd1, 0x2c, 0x3e, 0x4a, 0x5b, 0x6d, 0x7e, 0x8f]; // DEPOSIT_SELECTOR
    
    Instruction {
        program_id: METEORA_VAULT_PROGRAM,
        accounts: vec![
            AccountMeta::new(*vault, false),
            AccountMeta::new(*token_vault, false),
            AccountMeta::new(*lp_mint, false),
            AccountMeta::new(*user_source_token, false),
            AccountMeta::new(*user_destination_token, false),
            AccountMeta::new(*user, true),
            AccountMeta::new_readonly(*token_program, false),
        ],
        data: create_vault_instruction_data(deposit_selector, amount, 1),
    }
}

// Meteora Vault Withdraw
pub fn create_meteora_vault_withdraw_ix(
    vault: &Pubkey,
    user: &Pubkey,
    user_source_token: &Pubkey,
    user_destination_token: &Pubkey,
    token_vault: &Pubkey,
    lp_mint: &Pubkey,
    token_program: &Pubkey,
    amount: u64,
) -> Instruction {
    tracing::debug!("Creating Meteora Vault withdraw instruction with the following details:");
    tracing::debug!("  Program ID: {}", METEORA_VAULT_PROGRAM);

    let withdraw_selector = &[0xa2, 0x3d, 0x4f, 0x5b, 0x6c, 0x7e, 0x8f, 0x91]; // WITHDRAW_SELECTOR
    
    Instruction {
        program_id: METEORA_VAULT_PROGRAM,
        accounts: vec![
            AccountMeta::new(*vault, false),
            AccountMeta::new(*token_vault, false),
            AccountMeta::new(*lp_mint, false),
            AccountMeta::new(*user_destination_token, false),
            AccountMeta::new(*user_source_token, false),
            AccountMeta::new(*user, true),
            AccountMeta::new_readonly(*token_program, false),
        ],
        data: create_vault_instruction_data(withdraw_selector, amount, 1),
    }
}

// Meteora DLMM Swap
pub fn create_meteora_dlmm_swap_ix(
    lb_pair: &Pubkey,
    user: &Pubkey,
    user_source_token: &Pubkey,
    user_destination_token: &Pubkey,
    bin_array_bitmap_extension: &Pubkey,
    reserve_x: &Pubkey,
    reserve_y: &Pubkey,
    token_x_mint: &Pubkey,
    token_y_mint: &Pubkey,
    oracle: &Pubkey,
    host_fee_in: &Pubkey,
    token_x_program: &Pubkey,
    token_y_program: &Pubkey,
    event_authority: &Pubkey,
    bin_arrays: &[Pubkey], // Dynamic bin arrays
    amount: u64,
) -> Instruction {
    tracing::debug!("Creating Meteora DLMM swap instruction with the following details:");
    tracing::debug!("  Program ID: {}", METEORA_DLMM_PROGRAM);
    tracing::debug!("  Accounts:");
    tracing::debug!("    1. lb_pair: {} (mutable)", lb_pair);
    tracing::debug!("    2. bin_array_bitmap_extension: {} (readonly)", bin_array_bitmap_extension);
    tracing::debug!("    3. reserve_x: {} (mutable)", reserve_x);
    tracing::debug!("    4. reserve_y: {} (mutable)", reserve_y);
    tracing::debug!("    5. user_source_token: {} (mutable)", user_source_token);
    tracing::debug!("    6. user_destination_token: {} (mutable)", user_destination_token);
    tracing::debug!("    7. token_x_mint: {} (readonly)", token_x_mint);
    tracing::debug!("    8. token_y_mint: {} (readonly)", token_y_mint);
    tracing::debug!("    9. oracle: {} (mutable)", oracle);
    tracing::debug!("    10. host_fee_in: {} (mutable)", host_fee_in);
    tracing::debug!("    11. user: {} (signer)", user);
    tracing::debug!("    12. token_x_program: {} (readonly)", token_x_program);
    tracing::debug!("    13. token_y_program: {} (readonly)", token_y_program);
    tracing::debug!("    14. event_authority: {} (readonly)", event_authority);
    tracing::debug!("    15. program_id: {} (readonly)", METEORA_DLMM_PROGRAM);
    
    for (i, bin_array) in bin_arrays.iter().enumerate() {
        tracing::debug!("    {}: bin_array_{}: {} (mutable)", 16 + i, i, bin_array);
    }

    tracing::debug!("  Args:");
    tracing::debug!("    amount: {}", amount);

    let swap_selector = &[0x60, 0x3e, 0x2a, 0xd4, 0x2c, 0x6e, 0x5a, 0x3b]; // SWAP_SELECTOR
    
    let mut accounts = vec![
        AccountMeta::new(*lb_pair, false),
        AccountMeta::new_readonly(*bin_array_bitmap_extension, false),
        AccountMeta::new(*reserve_x, false),
        AccountMeta::new(*reserve_y, false),
        AccountMeta::new(*user_source_token, false),
        AccountMeta::new(*user_destination_token, false),
        AccountMeta::new_readonly(*token_x_mint, false),
        AccountMeta::new_readonly(*token_y_mint, false),
        AccountMeta::new(*oracle, false),
        AccountMeta::new(*host_fee_in, false),
        AccountMeta::new_readonly(*user, true),
        AccountMeta::new_readonly(*token_x_program, false),
        AccountMeta::new_readonly(*token_y_program, false),
        AccountMeta::new_readonly(*event_authority, false),
        AccountMeta::new_readonly(METEORA_DLMM_PROGRAM, false),
    ];

    // Add non-zero bin arrays
    for bin_array in bin_arrays {
        if *bin_array != Pubkey::default() {
            accounts.push(AccountMeta::new(*bin_array, false));
        }
    }
    
    Instruction {
        program_id: METEORA_DLMM_PROGRAM,
        accounts,
        data: create_instruction_data(swap_selector, amount, 1),
    }
}

// Meteora DLMM Swap2 (with memo program)
pub fn create_meteora_dlmm_swap2_ix(
    lb_pair: &Pubkey,
    user: &Pubkey,
    user_source_token: &Pubkey,
    user_destination_token: &Pubkey,
    bin_array_bitmap_extension: &Pubkey,
    reserve_x: &Pubkey,
    reserve_y: &Pubkey,
    token_x_mint: &Pubkey,
    token_y_mint: &Pubkey,
    oracle: &Pubkey,
    host_fee_in: &Pubkey,
    token_x_program: &Pubkey,
    token_y_program: &Pubkey,
    memo_program: &Pubkey,
    event_authority: &Pubkey,
    bin_arrays: &[Pubkey], // Dynamic bin arrays
    amount: u64,
) -> Instruction {
    tracing::debug!("Creating Meteora DLMM swap2 instruction with the following details:");
    tracing::debug!("  Program ID: {}", METEORA_DLMM_PROGRAM);
    tracing::debug!("    14. memo_program: {} (readonly)", memo_program);

    let swap2_selector = &[0x71, 0x4f, 0x3b, 0xe5, 0x3d, 0x7f, 0x6a, 0x4c]; // SWAP2_SELECTOR
    
    let mut accounts = vec![
        AccountMeta::new(*lb_pair, false),
        AccountMeta::new_readonly(*bin_array_bitmap_extension, false),
        AccountMeta::new(*reserve_x, false),
        AccountMeta::new(*reserve_y, false),
        AccountMeta::new(*user_source_token, false),
        AccountMeta::new(*user_destination_token, false),
        AccountMeta::new_readonly(*token_x_mint, false),
        AccountMeta::new_readonly(*token_y_mint, false),
        AccountMeta::new(*oracle, false),
        AccountMeta::new(*host_fee_in, false),
        AccountMeta::new_readonly(*user, true),
        AccountMeta::new_readonly(*token_x_program, false),
        AccountMeta::new_readonly(*token_y_program, false),
        AccountMeta::new_readonly(*memo_program, false),
        AccountMeta::new_readonly(*event_authority, false),
        AccountMeta::new_readonly(METEORA_DLMM_PROGRAM, false),
    ];

    // Add non-zero bin arrays
    for bin_array in bin_arrays {
        if *bin_array != Pubkey::default() {
            accounts.push(AccountMeta::new(*bin_array, false));
        }
    }
    
    Instruction {
        program_id: METEORA_DLMM_PROGRAM,
        accounts,
        data: create_dlmm_swap2_instruction_data(swap2_selector, amount, 1),
    }
}

// Meteora DAMM V2 Swap
pub fn create_meteora_damm_v2_swap_ix(
    pool: &Pubkey,
    pool_authority: &Pubkey,
    user: &Pubkey,
    input_token_account: &Pubkey,
    output_token_account: &Pubkey,
    token_a_vault: &Pubkey,
    token_b_vault: &Pubkey,
    token_a_mint: &Pubkey,
    token_b_mint: &Pubkey,
    token_a_program: &Pubkey,
    token_b_program: &Pubkey,
    referral_token_account: &Pubkey,
    event_authority: &Pubkey,
    amount: u64,
) -> Instruction {
    tracing::debug!("Creating Meteora DAMM V2 swap instruction with the following details:");
    tracing::debug!("  Program ID: {}", METEORA_DAMM_V2_PROGRAM);
    tracing::debug!("  Accounts:");
    tracing::debug!("    1. pool_authority: {} (readonly)", pool_authority);
    tr