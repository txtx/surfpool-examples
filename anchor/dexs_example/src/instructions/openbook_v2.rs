use crate::prelude::*;
use crate::constants::{OPENBOOKV2_PROGRAM};
use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(
    AnchorDeserialize, AnchorSerialize, Clone, Debug, PartialEq, TryFromPrimitive, IntoPrimitive,
)]
#[repr(u8)]
pub enum Side {
    Bid = 0,
    Ask = 1,
}

fn create_instruction_data(
    side: Side,
    price_lots: i64,
    max_base_lots: i64,
    max_quote_lots_including_fees: i64,
    order_type: u8,
    limit: u8,
) -> Vec<u8> {
    let mut buffer = Vec::with_capacity(35);
    buffer.extend_from_slice(&[0x03, 0x2c, 0x47, 0x03]); // PLACE_TAKE_ORDER_SELECTOR
    buffer.push(Side::into(side));
    buffer.extend_from_slice(&price_lots.to_le_bytes());
    buffer.extend_from_slice(&max_base_lots.to_le_bytes());
    buffer.extend_from_slice(&max_quote_lots_including_fees.to_le_bytes());
    buffer.extend_from_slice(&order_type.to_le_bytes());
    buffer.extend_from_slice(&limit.to_le_bytes());
    buffer
}

fn get_lot_size(market: &Pubkey) -> Result<(i64, i64), Box<dyn std::error::Error>> {
    // This would need to be implemented to read from the market account
    // For now, returning placeholder values
    Ok((100000, 1)) // (base_lot_size, quote_lot_size)
}

pub fn create_openbook_v2_swap_ix(
    market: &Pubkey,
    user: &Pubkey,
    from: &Pubkey,
    to: &Pubkey,
    amount: u64,
    market_base_vault: &Pubkey,
    market_quote_vault: &Pubkey,
    market_authority: &Pubkey,
    bids: &Pubkey,
    asks: &Pubkey,
    event_heap: &Pubkey,
    oracle_a: &Pubkey,
    oracle_b: &Pubkey,
    open_orders_admin: &Pubkey,
    open_orders_accounts: &[Pubkey], // Optional open orders accounts
) -> Result<Instruction, Box<dyn std::error::Error>> {
    // Print detailed debug information about the instruction we're creating
    tracing::debug!("Creating OpenBook V2 swap instruction with the following details:");
    tracing::debug!("  Program ID: {}", OPENBOOKV2_PROGRAM);
    tracing::debug!("  Accounts:");
    tracing::debug!("    1. user: {} (signer)", user);
    tracing::debug!("    2. market: {} (mutable)", market);
    tracing::debug!("    3. market_authority: {} (readonly)", market_authority);
    tracing::debug!("    4. bids: {} (mutable)", bids);
    tracing::debug!("    5. asks: {} (mutable)", asks);
    tracing::debug!("    6. market_base_vault: {} (mutable)", market_base_vault);
    tracing::debug!("    7. market_quote_vault: {} (mutable)", market_quote_vault);
    tracing::debug!("    8. event_heap: {} (mutable)", event_heap);
    
    let user_source_token = get_associated_token_address(user, from);
    let user_destination_token = get_associated_token_address(user, to);
    tracing::debug!("    9. user_source_token: {} (mutable)", user_source_token);
    tracing::debug!("    10. user_destination_token: {} (mutable)", user_destination_token);
    tracing::debug!("    11. oracle_a: {} (readonly)", oracle_a);
    tracing::debug!("    12. oracle_b: {} (readonly)", oracle_b);
    tracing::debug!("    13. tokenProgram: {} (readonly)", spl_token::id());
    tracing::debug!("    14. systemProgram: {} (readonly)", system_program::id());
    tracing::debug!("    15. open_orders_admin: {} (readonly)", open_orders_admin);
    
    tracing::debug!("  Args:");
    tracing::debug!("    amount: {}", amount);

    // Get lot sizes for calculations
    let (base_lot_size, quote_lot_size) = get_lot_size(market)?;
    
    // Determine trade direction and calculate parameters
    let (side, price_lots, max_base_lots, max_quote_lots_including_fees) = 
        if from == market_base_vault {
            // Selling base for quote
            let side = Side::Ask;
            let price_lots = 1i64;
            let max_base_lots = (amount as i64)
                .checked_div(base_lot_size)
                .ok_or("Calculation error")?;
            let max_quote_lots = i64::MAX
                .checked_div(quote_lot_size)
                .ok_or("Calculation error")?;
            (side, price_lots, max_base_lots, max_quote_lots)
        } else {
            // Buying base with quote
            let side = Side::Bid;
            let price_lots = i64::MAX;
            let max_base_lots = i64::MAX
                .checked_div(base_lot_size)
                .ok_or("Calculation error")?;
            let max_quote_lots = (amount as i64)
                .checked_div(quote_lot_size)
                .ok_or("Calculation error")?;
            (side, price_lots, max_base_lots, max_quote_lots)
        };

    // Determine user token accounts based on trade direction
    let (user_base_account, user_quote_account) = 
        if from == market_base_vault && to == market_quote_vault {
            (user_source_token, user_destination_token)
        } else if from == market_quote_vault && to == market_base_vault {
            (user_destination_token, user_source_token)
        } else {
            return Err("Invalid token mint configuration".into());
        };

    let order_type = 3u8;
    let limit = 50u8;

    let mut accounts = vec![
        AccountMeta::new(*user, true),                        // swap authority (signer)
        AccountMeta::new(*user, true),                        // payer (signer)  
        AccountMeta::new(*market, false),                     // market
        AccountMeta::new_readonly(*market_authority, false),  // market authority
        AccountMeta::new(*bids, false),                       // bids
        AccountMeta::new(*asks, false),                       // asks
        AccountMeta::new(*market_base_vault, false),          // market base vault
        AccountMeta::new(*market_quote_vault, false),         // market quote vault
        AccountMeta::new(*event_heap, false),                 // event heap
        AccountMeta::new(user_base_account, false),           // user base account
        AccountMeta::new(user_quote_account, false),          // user quote account
        AccountMeta::new_readonly(*oracle_a, false),          // oracle a
        AccountMeta::new_readonly(*oracle_b, false),          // oracle b
        AccountMeta::new_readonly(spl_token::id(), false),    // token program
        AccountMeta::new_readonly(system_program::id(), false), // system program
        AccountMeta::new_readonly(*open_orders_admin, false), // open orders admin
    ];

    // Add optional open orders accounts (filter out zero addresses)
    for open_orders_account in open_orders_accounts {
        if *open_orders_account != Pubkey::default() {
            accounts.push(AccountMeta::new(*open_orders_account, false));
            tracing::debug!("    open_orders_account: {} (mutable)", open_orders_account);
        }
    }

    Ok(Instruction {
        program_id: OPENBOOKV2_PROGRAM,
        accounts,
        data: create_instruction_data(
            side,
            price_lots,
            max_base_lots,
            max_quote_lots_including_fees,
            order_type,
            limit,
        ),
    })
}