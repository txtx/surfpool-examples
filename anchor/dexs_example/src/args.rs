use clap::{Parser, Subcommand};


#[derive(Debug, Subcommand)]
pub enum Command {
  
}   

enum SimulateMode {
    ///Simulate on a single DEX
    Dex {
        /// The DEX name (e.g saros, raydium, )
        #[arg(short, long)]
        dex: String
        
        /// Amount in (raw, e.g. 0.1 SOL)
        #[arg(short, long)]
        amount: f64,
    }

    
    /// Simulate via an aggregator (e.g. Jupiter, OkDex)
    Aggregator {
        #[command(subcommand)]
        aggregator: Aggregators,
    },
}


#[derive(Subcommand, Debug)]
enum Aggregators {
    /// Simulate via Jupiter aggregator
    Jupiter {
        #[arg(long)]
        input_mint: String,

        #[arg(long)]
        output_mint: String,

        #[arg(long)]
        amount_in: f64,

        #[arg(long)]
        min_amount_out: Option<f64>,
    },

    /// Simulate via OkDex aggregator
    Okdex {
        #[arg(long)]
        input_mint: String,

        #[arg(long)]
        output_mint: String,

        #[arg(long)]
        amount_in: f64,

        #[arg(long)]
        min_amount_out: Option<f64>,
    },
    /// Simulate via Dflow aggregator
    Dflow {
        #[arg(long)]
        input_mint: String,

        #[arg(long)]
        output_mint: String,

        #[arg(long)]
        amount_in: f64,

        #[arg(long)]
        min_amount_out: Option<f64>,
    },
}

#[derive(Debug, Parser)]
#[clap(name = "app", version)]
pub struct App {
    #[clap(subcommand)]
    pub command: Command,
}
