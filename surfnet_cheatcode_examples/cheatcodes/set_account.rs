use crate::prelude::*;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AccountUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    lamports: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<String>,      
    #[serde(skip_serializing_if = "Option::is_none")]   
    owner: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]        
    executable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rent_epoch: Option<u64>,
}

pub async fn set_account(
    rpc_url: &str,
    pubkey: &str,
    account_update: AccountUpdate,
) -> Result<()> {
    let client = Client::new();
    // Convert binary data to base58
    let data_base58 = account_update.data.map(|d| bs58::encode(d).into_string());

    // Construct request struct
    let update = AccountUpdate {
        lamports: account_update.lamports,
        data: data_base58,
        owner: account_update.owner.map(String::from),
        executable:account_update.executable,
        rent_epoch:account_update.rent_epoch,
    };

    let body = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "surfnet_setAccount",
        "params": [pubkey, update]
    });

    let response = client.post(rpc_url).json(&body).send().await?;

    let json: serde_json::Value = response.json().await?;
    println!("RPC response: {:#?}", json);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_set_account() {
        let update = AccountUpdate {
            lamports: Some(500_000),
            data: Some("insert_data_here".into()),
            owner: None,
            executable: None,
            rent_epoch: None,
        };

        let result = set_account("http://localhost:8899", "insert_pubkey_here", update).await;
        assert!(result.is_ok());
    }
}