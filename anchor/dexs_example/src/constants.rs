use crate::prelude::{pubkey, Pubkey};

pub const DEFAULT_RPC_URL: &str = "127.0.0.1:8899";

pub const WSOL: Pubkey = pubkey!("So11111111111111111111111111111111111111112");
pub const USDC: Pubkey = pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");
pub const DEFAULT_SWAP_AMOUNT: f64 = 10.0;
pub const USDC_DECIMALS: i32 = 6;


pub const SOLFI_PROGRAM: Pubkey = pubkey!("SoLFiHG9TfgtdUXUjWAxi3LtvYuFyDLVhBWxdMZxyCe");

//SOLFI SOL-USDC market with the highest TVL:
pub const SOLFI_SOL_USDC_MARKET: Pubkey = pubkey!("CAPhoEse9xEH95XmdnJjYrZdNCA8xfUWdy3aWymHa1Vj");

// Constants for Lifinity
pub const LIFINITY_V2_PROGRAM: Pubkey = pubkey!("2wT8Yq49kHgDzXuPxZSaeLaH1qbmGXtEyPy64bL7aD3c");

// Lifinity V2 SOL/USDC Pool Data
pub struct LifinityPoolInfo {
    pub authority: Pubkey,
    pub amm: Pubkey,
    pub pool_mint: Pubkey,
    pub fee_account: Pubkey,
    pub oracle_main: Pubkey,
    pub oracle_sub: Pubkey,
    pub oracle_pc: Pubkey,
    pub pool_token_usdc: Pubkey, // usdc pool token
    pub pool_token_wsol: Pubkey, // wsol pool token
}

// Lifinity SOL/USDC pool
pub const LIFINITY_SOL_USDC_POOL: LifinityPoolInfo = LifinityPoolInfo {
    authority: pubkey!("82nEEkdjAf2TsVVj189DgRdp7kkQ9Ghs4LqY1gcgbjxn"),
    amm: pubkey!("71GHcjkwmtM7HSWBuqzjEp96prcNNwu1wpUywXiytREU"),
    pool_mint: pubkey!("AtpUocL94CzYR1tZouFpo76QeGsUMH7kSqicaTNy7Lvz"),
    fee_account: pubkey!("AczCqF64dSgTjmREcaCSB7eq561frTvSeJ7uLrW37QWG"),
    oracle_main: pubkey!("EPBJUVCmzvwkGPGcEuwKmXomfGt78Aozy6pj44x9xxDB"),
    oracle_sub: pubkey!("EPBJUVCmzvwkGPGcEuwKmXomfGt78Aozy6pj44x9xxDB"),
    oracle_pc: pubkey!("978Mhamcn7XDkq21kvJWhUDPytJkYtkv8pqnXrUcpUxU"),
    pool_token_usdc: pubkey!("BmKuiSYs91eP8cn8PTD2eue1vVmqfZq2ipg4WQknY23q"), // USDC pool token
    pool_token_wsol: pubkey!("FzMQ1s9vQs4v6wyjVoVTFoDBJX2Qjr5ZsDGi1SA8a8hy"), // WSOL pool token
};

//Byreal CLMM SOL/USDC Pool Data
pub struct ByrealClmmPoolInfo {
    amm_config: &Pubkey,
    pool_state: &Pubkey,
    // input_token_account: &Pubkey,
    // output_token_account: &Pubkey,
    input_vault: &Pubkey,
    output_vault: &Pubkey,
    observation_state: &Pubkey,
    input_vault_mint: &Pubkey,
    output_vault_mint: &Pubkey,
    tickarray_bitmap_extension: &Pubkey,
    tick_arrays: &[&Pubkey], // Variable number of tick arrays
}

// BYREAL CLMM SOL/USDC pool
pub const BYREAL_SOL_USDC_POOL: ByrealClmmPoolInfo = ByrealClmmPoolInfo {
    amm_config: pubkey!("71GHcjkwmtM7HSWBuqzjEp96prcNNwu1wpUywXiytREU"),
    pool_state:pubkey!("AtpUocL94CzYR1tZouFpo76QeGsUMH7kSqicaTNy7Lvz"),
    input_vault: pubkey!("EPBJUVCmzvwkGPGcEuwKmXomfGt78Aozy6pj44x9xxDB"),
    output_vault: pubkey!("EPBJUVCmzvwkGPGcEuwKmXomfGt78Aozy6pj44x9xxDB"),
    observation_state: pubkey!("978Mhamcn7XDkq21kvJWhUDPytJkYtkv8pqnXrUcpUxU"),
    input_vault_mint: pubkey!("978Mhamcn7XDkq21kvJWhUDPytJkYtkv8pqnXrUcpUxU"),
    output_vault_mint: pubkey!("978Mhamcn7XDkq21kvJWhUDPytJkYtkv8pqnXrUcpUxU"),
    tickarray_bitmap_extension: pubkey!("FzMQ1s9vQs4v6wyjVoVTFoDBJX2Qjr5ZsDGi1SA8a8hy"),
    tick_arrays: &[pubkey!()]
};

pub const JUP_AGGV6_PROGRAM: Pubkey = pubkey!("JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4");

//this is how the rest of the accounts will be like (lifinity)


//We can use jupiter to force route through particular dexes to get the relevant accounts for a wsol usdc swap 

// the accounts are surely in order not just scattered lol i can remeber that fo rthe okx each set of accounts had an offset 

//lets even make our own aggregator as a learning tool - use carbon for indexing , postgres for backend , axum for server 

//play around with different routing algorithms djiktra A star, bellman ford , BFS , DFS 

