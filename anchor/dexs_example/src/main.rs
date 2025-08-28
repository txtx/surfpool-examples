pub mod prelude;
pub mod args;
pub mod cmd;
pub mod constants;
pub mod utils;
pub mod instructions;
 
use crate::args::{App, Command};
use crate::cmd::*;
use crate::prelude::{Result};
use clap::Parser;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, fmt};

#[tokio::main]
async fn main() -> Result<()>{
       tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .init();

    let cmd = App::parse().command;
 
   Commands::Simulate { mode } => match mode {
            SimulateMode::Dex { dex, amount } => {
                println!("Simulating direct DEX swap on {} with amount {}", dex, amount);
            }

            SimulateMode::Aggregator { aggregator } => match aggregator {
                Aggregators::Jupiter {
                    input_mint,
                    output_mint,
                    amount_in,
                    min_amount_out,
                } => {
                    println!(
                        "Simulating Jupiter swap: {} -> {} | amount_in={} | min_out={:?}",
                        input_mint, output_mint, amount_in, min_amount_out
                    );
                }
                // Aggregators::Okdex {
                //     input_mint,
                //     output_mint,
                //     amount_in,
                //     min_amount_out,
                // } => {
                //     println!(
                //         "Simulating OkDex swap: {} -> {} | amount_in={} | min_out={:?}",
                //         input_mint, output_mint, amount_in, min_amount_out
                //     );
                // }
                // Aggregators::Dflow {
                //     input_mint,
                //     output_mint,
                //     amount_in,
                //     min_amount_out,
                // } => {
                //     println!(
                //         "Simulating OkDex swap: {} -> {} | amount_in={} | min_out={:?}",
                //         input_mint, output_mint, amount_in, min_amount_out
                //     );
                // }
            },
        },
    Ok(())
}
