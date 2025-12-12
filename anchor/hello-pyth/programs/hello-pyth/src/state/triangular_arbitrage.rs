#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;
use pyth_solana_receiver_sdk::price_update::{Price, PriceUpdateV2};

#[derive(Accounts)]
pub struct UpdateTriangularArbitrage<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,
    #[account(
        init_if_needed,
        payer = sender,
        seeds = [b"triangle", sender.key().as_ref()],
        bump,
        space = 8 + TriangularArbitrageReport::INIT_SPACE,
    )]
    pub report: Account<'info, TriangularArbitrageReport>,
    pub starting_pair_account: Account<'info, PriceUpdateV2>,
    pub bridging_pair_account: Account<'info, PriceUpdateV2>,
    pub crossing_pair_account: Account<'info, PriceUpdateV2>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct TriangularArbitrageReport {
    pub starting_price: i64,
    pub bridging_price: i64,
    pub crossing_price: i64,
    pub updated_at: i64,
    pub pnl: i64,
}

impl TriangularArbitrageReport {
    pub fn update(
        &mut self,
        starting_price: &Price,
        bridging_price: &Price,
        crossing_price: &Price,
        clock: &Clock,
    ) {
        self.starting_price = starting_price.price;
        self.bridging_price = bridging_price.price;
        self.crossing_price = crossing_price.price;
        self.updated_at = clock.unix_timestamp;

        // Combine exponents: starting_expo + bridging_expo - crossing_expo
        let total_expo =
            starting_price.exponent + bridging_price.exponent - crossing_price.exponent;

        // Scale factor to bring result back to basis points (10000 = 100%)
        let scale_adjustment = 10_000i64 * 10i64.pow((-total_expo) as u32);

        // Calculate: (bridging * starting) / crossing
        // This checks: (ETH/BTC * BTC/USD) / ETH/USD > 1 for arbitrage
        let numerator = (bridging_price.price as i128) * (starting_price.price as i128);
        let result = numerator / (crossing_price.price as i128);

        // Convert to PNL in basis points (subtract 10000 for 0% baseline)
        self.pnl = ((result * scale_adjustment as i128) / 10i64.pow(16) as i128) as i64 - 10_000;
    }
}
