mod args;
mod cmd;
mod constants;
mod utils;

use crate::args::{App, Command};
use crate::cmd::{simulate_lifi,simulate_jup,simulate_dflow, simulate_solfi};
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
        Command::SimulateLiFi { amount } => simulate(amount)?,
        Command::SimulateSolFi { amount } => simulate(amount)?,
        Command::SimulateJup { amount } => simulate(amount)?,
        Command::SimulateDFlow { amount } => simulate(amount)?,
    }
    Ok(())
}
