#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;
use pyth_solana_receiver_sdk::price_update::{get_feed_id_from_hex, PriceUpdateV2};

declare_id!("BDBbn95ytTakeVSuV5yFeMVGMQpiFMXt3V8SMsznHEAN");

pub const MAXIMUM_AGE: u64 = 60; // One minute
pub const FEED_ID: &str = "0xef0d8b6fda2ceba41da15d4095d1da392a0d2f8ed0c6c7bc0f4cfac8c280b56d"; // SOL/USD price feed id from https://pyth.network/developers/price-feed-ids

#[program]
pub mod price_feed {
    use super::*;

    pub fn fetch_price(ctx: Context<FetchPrice>) -> Result<()> {
        let custom = &mut ctx.accounts.price;
        let price_update = &mut ctx.accounts.pyth_price_feed;
        let price = price_update.get_price_no_older_than(
            &Clock::get()?,
            MAXIMUM_AGE,
            &get_feed_id_from_hex(FEED_ID)?,
        )?;

        let current_price = price.price;
        let expo = price.exponent;

        custom.price = (current_price as f64) * 10f64.powi(expo);
        custom.time_checked = Clock::get()?.unix_timestamp;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct FetchPrice<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,
    #[account(
        init_if_needed,
        payer = sender,
        seeds = [b"price", sender.key().as_ref()],
        bump,
        space = 8 + Price::INIT_SPACE,
    )]
    pub price: Account<'info, Price>,
    pub pyth_price_feed: Account<'info, PriceUpdateV2>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct Price {
    pub price: f64,
    pub time_checked: i64,
}
