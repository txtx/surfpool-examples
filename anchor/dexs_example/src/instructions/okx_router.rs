//https://docs.rs/carbon-okx-dex-decoder/0.9.1/src/carbon_okx_dex_decoder/instructions/commission_sol_from_swap.rs.html

//https://github.com/okxlabs/DEX-Router-Solana-V1/blob/8a847ad3cae9ae92fe8e5fadcf7e26d34aedb6cf/programs/dex-solana/src/lib.rs\


//https://github.com/okx/dex-api-library/blob/main/lib/solana/swap/solana-swap-instructions.ts 

use crate::prelude::*;
use crate::constants::COMMISSION_SOL_PROGRAM;
use spl_token::get_associated_token_address;

// Helper function to create swap instruction data
fn create_swap_instruction_data(
    amount_in: u64,
    expect_amount_out: u64,
    min_return: u64,
    amounts: &[u64],
    routes: &[Vec<Route>],
) -> Vec<u8> {
    let mut buffer = Vec::new();
    // Add discriminator for regular swap
    buffer.extend_from_slice(&[0x1e, 0x21, 0xd0, 0x5b, 0x1f, 0x9d, 0x25, 0x12]);
    
    // Serialize SwapArgs
    buffer.extend_from_slice(&amount_in.to_le_bytes());
    buffer.extend_from_slice(&expect_amount_out.to_le_bytes());
    buffer.extend_from_slice(&min_return.to_le_bytes());
    
    // Serialize amounts vector
    buffer.extend_from_slice(&(amounts.len() as u32).to_le_bytes());
    for amount in amounts {
        buffer.extend_from_slice(&amount.to_le_bytes());
    }
    
    // Serialize routes (simplified - in real implementation would need full Route serialization)
    buffer.extend_from_slice(&(routes.len() as u32).to_le_bytes());
    for route in routes {
        buffer.extend_from_slice(&(route.len() as u32).to_le_bytes());
        // Route serialization would go here
    }
    
    buffer
}

// Helper function to create commission swap instruction data
fn create_commission_swap_instruction_data(
    amount_in: u64,
    expect_amount_out: u64,
    min_return: u64,
    amounts: &[u64],
    routes: &[Vec<Route>],
    commission_rate: u16,
    commission_direction: bool,
    order_id: u64,
) -> Vec<u8> {
    let mut buffer = Vec::new();
    // Add discriminator for commission proxy swap
    buffer.extend_from_slice(&[0x1e, 0x21, 0xd0, 0x5b, 0x1f, 0x9d, 0x25, 0x12]);
    
    // Serialize SwapArgs
    buffer.extend_from_slice(&amount_in.to_le_bytes());
    buffer.extend_from_slice(&expect_amount_out.to_le_bytes());
    buffer.extend_from_slice(&min_return.to_le_bytes());
    
    // Serialize amounts vector
    buffer.extend_from_slice(&(amounts.len() as u32).to_le_bytes());
    for amount in amounts {
        buffer.extend_from_slice(&amount.to_le_bytes());
    }
    
    // Serialize routes
    buffer.extend_from_slice(&(routes.len() as u32).to_le_bytes());
    for route in routes {
        buffer.extend_from_slice(&(route.len() as u32).to_le_bytes());
    }
    
    // Commission specific fields
    buffer.extend_from_slice(&commission_rate.to_le_bytes());
    buffer.push(commission_direction as u8);
    buffer.extend_from_slice(&order_id.to_le_bytes());
    
    buffer
}

// Helper function to create bridge instruction data
fn create_bridge_instruction_data(
    amount_in: u64,
    expect_amount_out: u64,
    min_return: u64,
    amounts: &[u64],
    routes: &[Vec<Route>],
    commission_rate: u16,
    bridge_to_args: &BridgeToArgs,
    offset: u8,
    len: u8,
) -> Vec<u8> {
    let mut buffer = Vec::new();
    // Add discriminator for commission from swap
    buffer.extend_from_slice(&[0x81, 0x3b, 0x45, 0x0a, 0x84, 0x4c, 0x23, 0x14]);
    
    // Serialize SwapArgs
    buffer.extend_from_slice(&amount_in.to_le_bytes());
    buffer.extend_from_slice(&expect_amount_out.to_le_bytes());
    buffer.extend_from_slice(&min_return.to_le_bytes());
    
    // Serialize amounts and routes (simplified)
    buffer.extend_from_slice(&(amounts.len() as u32).to_le_bytes());
    buffer.extend_from_slice(&(routes.len() as u32).to_le_bytes());
    
    // Commission and bridge specific fields
    buffer.extend_from_slice(&commission_rate.to_le_bytes());
    buffer.push(offset);
    buffer.push(len);
    
    // BridgeToArgs would be serialized here
    
    buffer
}

pub fn create_commission_sol_proxy_swap_ix(
    payer: &Pubkey,
    source_mint: &Pubkey,
    destination_mint: &Pubkey,
    source_token_account: &Pubkey,
    destination_token_account: &Pubkey,
    commission_account: &Pubkey,
    sa_authority: &Pubkey,
    amount_in: u64,
    expect_amount_out: u64,
    min_return: u64,
    amounts: &[u64],
    routes: &[Vec<Route>],
    commission_rate: u16,
    commission_direction: bool,
    order_id: u64,
) -> Instruction {
    tracing::debug!("Creating Commission SOL proxy swap instruction with the following details:");
    tracing::debug!("  Program ID: {}", COMMISSION_SOL_PROGRAM);
    tracing::debug!("  Accounts:");
    tracing::debug!("    1. payer: {} (signer)", payer);
    tracing::debug!("    2. source_token_account: {} (mutable)", source_token_account);
    tracing::debug!("    3. destination_token_account: {} (mutable)", destination_token_account);
    tracing::debug!("    4. source_mint: {} (readonly)", source_mint);
    tracing::debug!("    5. destination_mint: {} (readonly)", destination_mint);
    tracing::debug!("    6. commission_account: {} (mutable)", commission_account);
    tracing::debug!("    7. sa_authority: {} (readonly)", sa_authority);
    
    let source_token_sa = get_associated_token_address(sa_authority, source_mint);
    let destination_token_sa = get_associated_token_address(sa_authority, destination_mint);
    
    tracing::debug!("    8. source_token_sa: {} (mutable)", source_token_sa);
    tracing::debug!("    9. destination_token_sa: {} (mutable)", destination_token_sa);
    tracing::debug!("    10. source_token_program: {} (readonly)", spl_token::id());
    tracing::debug!("    11. destination_token_program: {} (readonly)", spl_token::id());
    tracing::debug!("    12. associated_token_program: {} (readonly)", spl_associated_token_account::id());
    tracing::debug!("    13. system_program: {} (readonly)", solana_program::system_program::id());
    
    tracing::debug!("  Args:");
    tracing::debug!("    amount_in: {}", amount_in);
    tracing::debug!("    expect_amount_out: {}", expect_amount_out);
    tracing::debug!("    min_return: {}", min_return);
    tracing::debug!("    commission_rate: {}", commission_rate);
    tracing::debug!("    commission_direction: {}", commission_direction);
    tracing::debug!("    order_id: {}", order_id);
    tracing::debug!("    routes_count: {}", routes.len());

    Instruction {
        program_id: COMMISSION_SOL_PROGRAM,
        accounts: vec![
            AccountMeta::new(*payer, true),
            AccountMeta::new(*source_token_account, false),
            AccountMeta::new(*destination_token_account, false),
            AccountMeta::new_readonly(*source_mint, false),
            AccountMeta::new_readonly(*destination_mint, false),
            AccountMeta::new(*commission_account, false),
            AccountMeta::new_readonly(*sa_authority, false),
            AccountMeta::new(source_token_sa, false),
            AccountMeta::new(destination_token_sa, false),
            AccountMeta::new_readonly(spl_token::id(), false),
            AccountMeta::new_readonly(spl_token::id(), false),
            AccountMeta::new_readonly(spl_associated_token_account::id(), false),
            AccountMeta::new_readonly(solana_program::system_program::id(), false),
        ],
        data: create_commission_swap_instruction_data(
            amount_in,
            expect_amount_out,
            min_return,
            amounts,
            routes,
            commission_rate,
            commission_direction,
            order_id,
        ),
    }
}

pub fn create_commission_sol_from_swap_ix(
    payer: &Pubkey,
    source_mint: &Pubkey,
    destination_mint: &Pubkey,
    source_token_account: &Pubkey,
    destination_token_account: &Pubkey,
    bridge_program: &Pubkey,
    commission_account: &Pubkey,
    amount_in: u64,
    expect_amount_out: u64,
    min_return: u64,
    amounts: &[u64],
    routes: &[Vec<Route>],
    commission_rate: u16,
    bridge_to_args: &BridgeToArgs,
    offset: u8,
    len: u8,
) -> Instruction {
    tracing::debug!("Creating Commission SOL from swap instruction with the following details:");
    tracing::debug!("  Program ID: {}", COMMISSION_SOL_PROGRAM);
    tracing::debug!("  Accounts:");
    tracing::debug!("    1. payer: {} (signer)", payer);
    tracing::debug!("    2. source_token_account: {} (mutable)", source_token_account);
    tracing::debug!("    3. destination_token_account: {} (mutable)", destination_token_account);
    tracing::debug!("    4. source_mint: {} (readonly)", source_mint);
    tracing::debug!("    5. destination_mint: {} (readonly)", destination_mint);
    tracing::debug!("    6. bridge_program: {} (readonly)", bridge_program);
    tracing::debug!("    7. associated_token_program: {} (readonly)", spl_associated_token_account::id());
    tracing::debug!("    8. token_program: {} (readonly)", spl_token::id());
    tracing::debug!("    9. token_2022_program: {} (readonly)", spl_token_2022::id());
    tracing::debug!("    10. system_program: {} (readonly)", solana_program::system_program::id());
    tracing::debug!("    11. commission_account: {} (mutable)", commission_account);
    
    tracing::debug!("  Args:");
    tracing::debug!("    amount_in: {}", amount_in);
    tracing::debug!("    expect_amount_out: {}", expect_amount_out);
    tracing::debug!("    min_return: {}", min_return);
    tracing::debug!("    commission_rate: {}", commission_rate);
    tracing::debug!("    bridge_to_chain_id: {}", bridge_to_args.to_chain_id);
    tracing::debug!("    bridge_amount: {}", bridge_to_args.amount);
    tracing::debug!("    offset: {}", offset);
    tracing::debug!("    len: {}", len);

    Instruction {
        program_id: COMMISSION_SOL_PROGRAM,
        accounts: vec![
            AccountMeta::new(*payer, true),
            AccountMeta::new(*source_token_account, false),
            AccountMeta::new(*destination_token_account, false),
            AccountMeta::new_readonly(*source_mint, false),
            AccountMeta::new_readonly(*destination_mint, false),
            AccountMeta::new_readonly(*bridge_program, false),
            AccountMeta::new_readonly(spl_associated_token_account::id(), false),
            AccountMeta::new_readonly(spl_token::id(), false),
            AccountMeta::new_readonly(spl_token_2022::id(), false),
            AccountMeta::new_readonly(solana_program::system_program::id(), false),
            AccountMeta::new(*commission_account, false),
        ],
        data: create_bridge_instruction_data(
            amount_in,
            expect_amount_out,
            min_return,
            amounts,
            routes,
            commission_rate,
            bridge_to_args,
            offset,
            len,
        ),
    }
}

pub fn create_commission_sol_simple_swap_ix(
    payer: &Pubkey,
    source_mint: &Pubkey,
    destination_mint: &Pubkey,
    amount_in: u64,
    expect_amount_out: u64,
    min_return: u64,
    amounts: &[u64],
    routes: &[Vec<Route>],
) -> Instruction {
    tracing::debug!("Creating Commission SOL simple swap instruction with the following details:");
    tracing::debug!("  Program ID: {}", COMMISSION_SOL_PROGRAM);
    tracing::debug!("  Accounts:");
    tracing::debug!("    1. payer: {} (signer)", payer);
    
    let source_token_account = get_associated_token_address(payer, source_mint);
    let destination_token_account = get_associated_token_address(payer, destination_mint);
    
    tracing::debug!("    2. source_token_account: {} (mutable)", source_token_account);
    tracing::debug!("    3. destination_token_account: {} (mutable)", destination_token_account);
    tracing::debug!("    4. source_mint: {} (readonly)", source_mint);
    tracing::debug!("    5. destination_mint: {} (readonly)", destination_mint);
    tracing::debug!("    6. token_program: {} (readonly)", spl_token::id());
    tracing::debug!("    7. associated_token_program: {} (readonly)", spl_associated_token_account::id());
    tracing::debug!("    8. system_program: {} (readonly)", solana_program::system_program::id());
    
    tracing::debug!("  Args:");
    tracing::debug!("    amount_in: {}", amount_in);
    tracing::debug!("    expect_amount_out: {}", expect_amount_out);
    tracing::debug!("    min_return: {}", min_return);
    tracing::debug!("    routes_count: {}", routes.len());

    Instruction {
        program_id: COMMISSION_SOL_PROGRAM,
        accounts: vec![
            AccountMeta::new(*payer, true),
            AccountMeta::new(source_token_account, false),
            AccountMeta::new(destination_token_account, false),
            AccountMeta::new_readonly(*source_mint, false),
            AccountMeta::new_readonly(*destination_mint, false),
            AccountMeta::new_readonly(spl_token::id(), false),
            AccountMeta::new_readonly(spl_associated_token_account::id(), false),
            AccountMeta::new_readonly(solana_program::system_program::id(), false),
        ],
        data: create_swap_instruction_data(
            amount_in,
            expect_amount_out,
            min_return,
            amounts,
            routes,
        ),
    }
}





// Commission rate validation helper
pub fn validate_commission_rate(commission_rate: u16) -> Result<(), Box<dyn std::error::Error>> {
    const MAX_COMMISSION_RATE: u16 = 10000; // 100% in basis points
    
    if commission_rate > MAX_COMMISSION_RATE {
        return Err(format!("Commission rate {} exceeds maximum {}", commission_rate, MAX_COMMISSION_RATE).into());
    }
    
    Ok(())
}