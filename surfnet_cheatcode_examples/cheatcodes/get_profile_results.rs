
use crate::prelude::*;


#[derive(Serialize)]
struct GetProfileResults {
    tag: String,
}

pub async fn get_profile_results(
    rpc_url: &str,
    tag: &str,
) -> Result<()> {
    let client = Client::new();

    let request_payload = GetProfileResults {
        tag: tag.to_string(),
    };

    let body = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "surfnet_getProfileResults",
        "params": [request_payload.tag]
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
    async fn test_get_profile_results() {
        let result = get_profile_results("http://localhost:8899", "test-tag").await;
        assert!(result.is_ok());
    }
}
