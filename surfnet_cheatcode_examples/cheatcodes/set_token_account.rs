use crate::prelude::*;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TokenAccountUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    amount: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delegated_amount: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    delegate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    close_authority: Option<String>
}

pub async fn set_token_account( 
    owner: String,
    mint: String,
    account_update: TokenAccountUpdate
) -> Result<()> {
    let client = Client::new();

      let update = TokenAccountUpdate {
        amount: Some(account_update.amount.unwrap_or(1_000_000)),
        delegated_amount:account_update.delegated_amount, // defaults to None
        state: Some(account_update.state.unwrap_or_else(|| "initialized".to_string())),
        delegate:account_update.delegate,
        close_authority:account_update.close_authority,
    };

    let request_body = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "surfnet_setTokenAccount",
        "params": [owner, mint, update]
    });

    let response = client
        .post("http://127.0.0.1:8899")
        .json(&request_body)
        .send()
        .await?;

    let result: serde_json::Value = response.json().await?;
    println!("Response: {:#?}", result); 

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_set_token_account() {
        let update = TokenAccountUpdate {
            amount: Some(500_000),
            delegated_amount: None,
            state: Some("initialized".to_string()),
            delegate: None,
            close_authority: None,
        };

        let result = set_token_account(
            "insert_owner_here".to_string(),
            "insert_mint_here".to_string(),
            update,
        )
        .await;

        assert!(result.is_ok());
    }
}