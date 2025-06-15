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
 
    match cmd {
        Command::SimulateLiFi { amount } => simulate_lifi(amount).await?,
        Command::SimulateSolFi { amount } => simulate_solfi(amount).await?,
        Command::SimulateJup { amount } => simulate_jup(amount).await?,
        Command::SimulateDFlow { amount } => simulate_dflow(amount).await?,
    }
    Ok(())
}
