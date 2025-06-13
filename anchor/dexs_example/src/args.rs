use clap::{Parser, Subcommand};


#[derive(Debug, Subcommand)]
pub enum Command {
    /// Simulate a WSOL -> USDC swap in all the a solfi wsol/usdc pool
    SolFi {
        /// Amount of SOL to swap to USDC
        #[arg(short, long)]
        amount: Option<f64>,
    },
    /// Simulate a WSOL -> USDC swap in all the a lifi wsol/usdc pool
    LiFi {
        /// Amount of SOL to swap to USDC
        #[arg(short, long)]
        amount: Option<f64>,
    },
    /// Simulate a WSOL -> USDC swap in all the wsol/usdc pools, aggregated 
    Jupiter {
        /// Amount of SOL to swap to USDC
        #[arg(short, long)]
        amount: Option<f64>,
    },
    /// Simulate a WSOL -> USDC swap in all the wsol/usdc pools, aggregated
    DFlow {
        /// Amount of SOL to swap to USDC
        #[arg(short, long)]
        amount: Option<f64>,
    },
}

#[derive(Debug, Parser)]
#[clap(name = "app", version)]
pub struct App {
    #[clap(subcommand)]
    pub command: Command,
}
