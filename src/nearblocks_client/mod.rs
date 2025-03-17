use near_sdk::AccountId;
use reqwest::Client;
use serde::{Deserialize, Serialize};
pub mod types;
use types::Transaction;
pub mod proposal;
pub mod rfp;
pub mod sputnik;
pub mod transactions;
use crate::rpc_service::Env;
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiResponse {
    #[serde(default)]
    pub txns: Vec<Transaction>,
    pub cursor: Option<String>,
}

#[derive(Clone)]
pub struct ApiClient {
    base_url: String,
    client: Client,
    api_key: String,
}

impl Default for ApiClient {
    fn default() -> Self {
        Self::new()
    }
}

impl ApiClient {
    pub fn new() -> Self {
        dotenvy::dotenv().ok();
        let env: Env = envy::from_env::<Env>().expect("Failed to load environment variables");

        Self {
            base_url: "https://api.nearblocks.io/".to_string(),
            client: Client::new(),
            api_key: env.nearblocks_api_key,
        }
    }

    pub async fn get_account_txns_by_pagination(
        &self,
        account_id: AccountId,
        cursor: String,
        limit: Option<i32>,
        order: Option<String>,
        page: Option<i32>,
        after_block: Option<i64>,
    ) -> anyhow::Result<ApiResponse> {
        let base_params = self.build_pagination_params(limit, order, page);
        let query_params = self.add_cursor_param(base_params, cursor, after_block);
        let endpoint = format!("v1/account/{}/txns", account_id);
        let url = self.base_url.clone() + &endpoint + &query_params;

        println!("Fetching transactions from {}", url);

        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;

        // Add debug information about the response
        println!("Response status: {}", response.status());

        if !response.status().is_success() {
            let error_text = response.text().await?;
            eprintln!("API error response: {}", error_text);
            return Err(anyhow::anyhow!("API error response: {}", error_text));
        }

        let response_text = response.text().await?;

        match serde_json::from_str::<ApiResponse>(&response_text) {
            Ok(api_response) => {
                println!(
                    "Successfully fetched {} transactions",
                    api_response.txns.len()
                );
                Ok(api_response)
            }
            Err(e) => {
                eprintln!("Failed to parse API response: {}", e);
                eprintln!("Raw response: {}", response_text);
                Err(anyhow::anyhow!("Failed to parse API response: {}", e))
            }
        }
    }

    // 692tHFoeCUDWs9PSdvrb3FiZEHvso6pzrjgy8VK8RVVG
    // curl 'https://api.nearblocks.io/v1/search/receipts?keyword=692tHFoeCUDWs9PSdvrb3FiZEHvso6pzrjgy8VK8RVVG'
    pub async fn get_receipt_by_id(&self, receipt_id: &str) -> anyhow::Result<ApiResponse> {
        let endpoint = format!("v1/search/receipts?keyword={}", receipt_id);
        let url = self.base_url.clone() + &endpoint;

        println!("Fetching receipt from {}", url);
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;

        let response_text = response.text().await?;

        match serde_json::from_str::<ApiResponse>(&response_text) {
            Ok(api_response) => {
                println!(
                    "Successfully fetched {} transactions",
                    api_response.txns.len()
                );
                Ok(api_response)
            }
            Err(e) => {
                eprintln!("Failed to parse API response: {}", e);
                eprintln!("Raw response: {}", response_text);
                Err(anyhow::anyhow!("Failed to parse API response: {}", e))
            }
        }
    }

    fn build_pagination_params(
        &self,
        limit: Option<i32>,
        order: Option<String>,
        page: Option<i32>,
    ) -> String {
        format!(
            "?per_page={}&order={}&page={}",
            limit.unwrap_or(25),
            order.unwrap_or_else(|| "asc".to_string()),
            page.unwrap_or(1),
        )
    }

    fn add_cursor_param(
        &self,
        base_params: String,
        cursor: String,
        after_block: Option<i64>,
    ) -> String {
        if cursor.is_empty() {
            format!("{}&after_block={}", base_params, after_block.unwrap_or(0))
        } else {
            format!("{}&cursor={}", base_params, cursor)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_cursor_param() {
        let client = ApiClient::default();
        let base_params = "?per_page=50&order=asc&page=1".to_string();

        // Test case 1: Empty cursor with default after_block
        let result = client.add_cursor_param(base_params.clone(), "".to_string(), None);
        assert_eq!(result, "?per_page=50&order=asc&page=1&after_block=0");

        // Test case 2: Empty cursor with specific after_block
        let result = client.add_cursor_param(base_params.clone(), "".to_string(), Some(12345));
        assert_eq!(result, "?per_page=50&order=asc&page=1&after_block=12345");

        // Test case 3: Non-empty cursor (after_block should be ignored)
        let result =
            client.add_cursor_param(base_params.clone(), "abc123".to_string(), Some(12345));
        assert_eq!(result, "?per_page=50&order=asc&page=1&cursor=abc123");
    }

    #[test]
    fn test_build_pagination_params() {
        let client = ApiClient::default();

        // Test case 1: All parameters are None (default values)
        let result = client.build_pagination_params(None, None, None);
        assert_eq!(result, "?per_page=25&order=asc&page=1");

        // Test case 2: Custom limit, default others
        let result = client.build_pagination_params(Some(50), None, None);
        assert_eq!(result, "?per_page=50&order=asc&page=1");

        // Test case 3: Custom order, default others
        let result = client.build_pagination_params(None, Some("desc".to_string()), None);
        assert_eq!(result, "?per_page=25&order=desc&page=1");

        // Test case 4: Custom page, default others
        let result = client.build_pagination_params(None, None, Some(3));
        assert_eq!(result, "?per_page=25&order=asc&page=3");

        // Test case 5: All parameters custom
        let result = client.build_pagination_params(Some(100), Some("desc".to_string()), Some(5));
        assert_eq!(result, "?per_page=100&order=desc&page=5");
    }
}
