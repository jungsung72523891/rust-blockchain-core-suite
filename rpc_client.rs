use reqwest::Client;
use serde_json::{json, Value};
use std::collections::HashMap;

pub struct RpcClient {
    client: Client,
    endpoint: String,
    request_id: u64,
}

impl RpcClient {
    pub fn new(endpoint: String) -> Self {
        Self {
            client: Client::new(),
            endpoint,
            request_id: 1,
        }
    }

    fn next_id(&mut self) -> u64 {
        let id = self.request_id;
        self.request_id += 1;
        id
    }

    pub async fn call_method(
        &mut self,
        method: &str,
        params: HashMap<String, String>,
    ) -> Result<Value, String> {
        let payload = json!({
            "jsonrpc": "2.0",
            "method": method,
            "params": params,
            "id": self.next_id()
        });

        let response = self.client
            .post(&self.endpoint)
            .json(&payload)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        let json: Value = response
            .json()
            .await
            .map_err(|e| format!("Invalid JSON: {}", e))?;

        if let Some(err) = json.get("error") {
            return Err(format!("RPC Error: {}", err));
        }

        json.get("result")
            .cloned()
            .ok_or_else(|| "No result field".to_string())
    }

    pub async fn get_block_height(&mut self) -> Result<u64, String> {
        let mut params = HashMap::new();
        let res = self.call_method("get_block_height", params).await?;
        res.as_u64().ok_or_else(|| "Invalid height".to_string())
    }
}
