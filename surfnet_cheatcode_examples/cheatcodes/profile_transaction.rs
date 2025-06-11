
use crate::prelude::*;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ProfileTransactionRequest {
    transaction_data: String,             // base64-encoded VersionedTransaction
    tag: Option<String>,                  // Optional tag
    encoding: Option<UiAccountEncoding>,  // Optional encoding
}

pub async fn profile_transaction(
    rpc_url: &str,
    transaction_data: Vec<u8>,
    tag: Option<&str>,
    encoding: Option<UiAccountEncoding>,
) -> Result<()> {
    let client = Client::new();

    let tx_base64 = base64::engine::general_purpose::STANDARD.encode(&transaction_data);

    let request_payload = ProfileTransactionRequest {
        transaction_data: tx_base64,
        tag: tag.map(String::from),
        encoding,
    };

    let body = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "surfnet_profileTransaction",
        "params": [
            request_payload.transaction_data,
            request_payload.tag,
            request_payload.encoding
        ]
    });

    let response = client.post(rpc_url).json(&body).send().await?;
    let json: serde_json::Value = response.json().await?;

    println!("RPC response: {:#?}", json);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_account_decoder::UiAccountEncoding;

    #[tokio::test]
    async fn test_profile_transaction() {
        let dummy_tx = vec![0u8; 32]; // Replace with real tx
        let result = profile_transaction(
            "http://localhost:8899",
            dummy_tx,
            Some("test-tag"),
            Some(UiAccountEncoding::Base64),
        )
        .await;

        assert!(result.is_ok());
    }
}

