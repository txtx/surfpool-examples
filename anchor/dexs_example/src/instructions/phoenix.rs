use crate::prelude::*;
use crate::constants::PHOENIX_PROGRAM;

fn create_phoenix_instruction_data(
    num_base_lots: u64,
    num_quote_lots: u64,
    side: u8,
    order_type: u8,
    self_trade_behavior: u8,
) -> Vec<u8> {
    let mut buffer = Vec::with_capacity(55);
    buffer.push(0); // discriminator
    buffer.push(order_type);
    buffer.push(side);
    buffer.push(0); // Indicates absence of price_in_ticks (market order)
    buffer.extend_from_slice(&num_base_lots.to_le_bytes());
    buffer.extend_from_slice(&num_quote_lots.to_le_bytes());
    buffer.extend_from_slice(&0u64.to_le_bytes()); // min_base_lots_to_fill
    buffer.extend_from_slice(&0u64.to_le_bytes()); // min_quote_lots_to_fill
    buffer.push(self_trade_behavior);
    buffer.push(0); // Indicates absence of match_limit
    buffer.extend_from_slice(&0u128.to_le_bytes()); // client_order_id
    buffer.push(0u8); // use_only_deposited_funds as false
    buffer
}

fn get_lot_sizes_from_market_data(market_data: &[u8]) -> Result<(u64, u64), &'static str> {
    if market_data.len() < 200 {
        return Err("Invalid market data length");
    }
    let base_lots_size = u64::from_le_bytes([
        market_data[112], market_data[113], market_data[114], market_data[115],
        market_data[116], market_data[117], market_data[118], market_data[119],
    ]);
    let quote_lots_size = u64::from_le_bytes([
        market_data[192], market_data[193], market_data[194], market_data[195],
        market_data[196], market_data[197], market_data[198], market_data[199],
    ]);
    Ok((base_lots_size, quote_lots_size))
}

pub fn create_phoenix_swap_ix(
    market: &Pubkey,
    user: &Pubkey,
    log_authority: &Pubkey,
    user_base_account: &Pubkey,
    user_quote_account: &Pubkey,
    base_vault: &Pubkey,
    quote_vault: &Pubkey,
    amount_in: u64,
    is_base_to_quote: bool,
    base_lot_size: u64,
    quote_lot_size: u64,
) -> Instruction {
    tracing::debug!("Creating Phoenix swap instruction with the following details:");
    tracing::debug!("  Program ID: {}", PHOENIX_PROGRAM);
    tracing::debug!("  Accounts:");
    tracing::debug!("    1. dex_program: {} (readonly)", PHOENIX_PROGRAM);
    tracing::debug!("    2. log_authority: {} (mutable)", log_authority);
    tracing::debug!("    3. market: {} (mutable)", market);
    tracing::debug!("    4. user: {} (signer)", user);
    tracing::debug!("    5. user_base_account: {} (mutable)", user_base_account);
    tracing::debug!("    6. user_quote_account: {} (mutable)", user_quote_account);
    tracing::debug!("    7. base_vault: {} (mutable)", base_vault);
    tracing::debug!("    8. quote_vault: {} (mutable)", quote_vault);
    tracing::debug!("    9. token_program: {} (readonly)", spl_token::id());
    
    tracing::debug!("  Args:");
    tracing::debug!("    amount_in: {}", amount_in);
    tracing::debug!("    is_base_to_quote: {}", is_base_to_quote);
    tracing::debug!("    base_lot_size: {}", base_lot_size);
    tracing::debug!("    quote_lot_size: {}", quote_lot_size);

    let (side, num_base_lots, num_quote_lots) = if is_base_to_quote {
        (1u8, amount_in / base_lot_size, 0u64) // 'ask' side
    } else {
        (0u8, 0u64, amount_in / quote_lot_size) // 'bid' side
    };

    let order_type = 2u8; // 'immediateOrCancel'
    let self_trade_behavior = 1u8; // 'cancelProvide'

    Instruction {
        program_id: PHOENIX_PROGRAM,
        accounts: vec![
            AccountMeta::new_readonly(PHOENIX_PROGRAM, false),
            AccountMeta::new(*log_authority, false),
            AccountMeta::new(*market, false),
            AccountMeta::new(*user, true),
            AccountMeta::new(*user_base_account, false),
            AccountMeta::new(*user_quote_account, false),
            AccountMeta::new(*base_vault, false),
            AccountMeta::new(*quote_vault, false),
            AccountMeta::new_readonly(spl_token::id(), false),
        ],
        data: create_phoenix_instruction_data(
            num_base_lots,
            num_quote_lots,
            side,
            order_type,
            self_trade_behavior,
        ),
    }
}