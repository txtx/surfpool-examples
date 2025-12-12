mod state;

use anchor_lang::prelude::*;
use pyth_solana_receiver_sdk::price_update::{get_feed_id_from_hex, VerificationLevel};
pub use state::triangular_arbitrage::*;

declare_id!("7ah529NarmVZLTH2qdQLNwrMNSqo3JMFvMTgucpLmZuV");

/// The price point TTL in seconds
pub const MAXIMUM_AGE: u64 = 6000;

#[program]
pub mod hello_pyth {
    use super::*;

    pub const ID: Pubkey = crate::ID;
    pub mod client {
        use super::*;
        pub mod accounts {
            pub fn report_pda_seed(sender: &Pubkey) -> Vec<&[u8]> {
                vec![b"triangle", sender.as_ref()]
            }
            pub struct UpdateTriangularArbitrage {
                pub sender: Pubkey,
                pub report: Pubkey,
                pub starting_pair_account: Pubkey,
                pub bridging_pair_account: Pubkey,
                pub crossing_pair_account: Pubkey,
                pub system_program: Pubkey,
            }
        }
        pub mod args {
            use super::*;
            pub struct UpdateTriangularArbitrage {
                pub starting_pair_feed_id: String,
                pub bridging_pair_feed_id: String,
                pub crossing_pair_feed_id: String,
            }
        }
    }

    pub fn update_triangular_arbitrage(
        ctx: Context<UpdateTriangularArbitrage>,
        starting_pair_feed_id: String,
        bridging_pair_feed_id: String,
        crossing_pair_feed_id: String,
    ) -> Result<()> {
        let report = &mut ctx.accounts.report;
        let clock = Clock::get()?;
        let verification = VerificationLevel::Partial { num_signatures: 4 };

        let starting = ctx
            .accounts
            .starting_pair_account
            .get_price_no_older_than_with_custom_verification_level(
                &clock,
                MAXIMUM_AGE,
                &get_feed_id_from_hex(&starting_pair_feed_id)?,
                verification.clone(),
            )?;

        // This pair is not longer updated, we'll skip the checks
        let bridging = ctx
            .accounts
            .bridging_pair_account
            .get_price_unchecked(&get_feed_id_from_hex(&bridging_pair_feed_id)?)?;

        let crossing = ctx
            .accounts
            .crossing_pair_account
            .get_price_no_older_than_with_custom_verification_level(
                &clock,
                MAXIMUM_AGE,
                &get_feed_id_from_hex(&crossing_pair_feed_id)?,
                verification.clone(),
            )?;

        report.update(&starting, &bridging, &crossing, &clock);

        msg!(
            "Triangular Arbitrage | USD/ETH: (${:.2}) | SOL/ETH: ({:.8}) | SOL/USD: (${:.2}) | PNL: {} bps",
            starting.price as f64 * 10f64.powi(starting.exponent),
            bridging.price as f64 * 10f64.powi(bridging.exponent),
            crossing.price as f64 * 10f64.powi(crossing.exponent),
            report.pnl,
        );

        if report.pnl > 0 {
            msg!("*** Arbitrage opportunity detected! ***");
        } else {
            return Err(HelloPythError::NoArbitrageOpportunity.into());
        }
        Ok(())
    }
}

#[error_code]
pub enum HelloPythError {
    #[msg("The price data is too old.")]
    PriceTooOld,
    #[msg("No arbitrage opportunity detected.")]
    NoArbitrageOpportunity,
}

#[derive(Accounts)]
pub struct Initialize {}

#[cfg(test)]
mod tests;
