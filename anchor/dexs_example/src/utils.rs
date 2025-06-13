use crate::prelude::*;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
/// RPC commitment config (optional field in Solana RPCs).
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommitmentConfig {
    pub commitment: String, // e.g., "finalized", "confirmed"
}

/// Token amount in various formats (parsed, raw).
#[derive(Debug, Deserialize)]
pub struct UiTokenAmount {
    #[serde(rename = "uiAmount")]
    pub ui_amount: Option<f64>,
    pub decimals: u8,
    pub amount: String,
    #[serde(rename = "uiAmountString")]
    pub ui_amount_string: String,
}

/// Generic JSON-RPC response wrapper.
#[derive(Debug, Deserialize)]
pub struct RpcResponse<T> {
    pub jsonrpc: String,
    pub result: Option<T>,
    pub id: u32,
}

/// Context for Solana RPC responses.
#[derive(Debug, Deserialize)]
pub struct Context {
    pub slot: u64,
}

/// Response format for token account balance.
#[derive(Debug, Deserialize)]
pub struct ContextValue<T> {
    pub context: Context,
    pub value: T,
}

/// Returns the balance of a token account, given its public key.
pub async fn get_token_account_balance(
    pubkey: &Pubkey,
    commitment: Option<CommitmentConfig>,
) -> Result<Option<UiTokenAmount>> {
    let client = Client::new();

    let request_body = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "getTokenAccountBalance",
        "params": [
            pubkey.to_string(),
            commitment.unwrap_or(CommitmentConfig {
                commitment: "finalized".to_string()
            })
        ]
    });

    let response = client
        .post("http://127.0.0.1:8899")
        .json(&request_body)
        .send()
        .await?;

    let result: RpcResponse<ContextValue<UiTokenAmount>> = response.json().await?;
    println!("Token Account Balance Response: {:#?}", result);

    Ok(result.result.map(|r| r.value))
}