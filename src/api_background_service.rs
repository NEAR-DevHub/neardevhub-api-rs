/**
 * TODO
 * - Update the cache in the background instead of blocking responses
 */
use crate::nearblocks_client::ApiClient;
use crate::types::Contract;
use near_account_id::AccountId;
use tokio::{
    signal,
    task::JoinHandle,
    time::{self, Duration},
};

pub struct ApiBackgroundService {
    api_client: ApiClient,
    handle: Option<JoinHandle<()>>,
    contract: Contract,
}

impl ApiBackgroundService {
    pub fn new(api_client: ApiClient, contract: Contract) -> Self {
        Self {
            api_client,
            handle: None,
            contract,
        }
    }

    // Spawns the background task and starts it
    pub fn start(&mut self) {
        let api_client = self.api_client.clone();
        let contract = self.contract.clone();
        let handle = tokio::spawn(async move {
            let mut interval = time::interval(Duration::from_secs(60));
            loop {
                interval.tick().await;

                match api_client
                    .get_account_txns_by_pagination(
                        contract.parse::<AccountId>().unwrap(),
                        "".to_string(),
                        Some(25),
                        Some("asc".to_string()),
                        Some(1),
                        None,
                    )
                    .await
                {
                    Ok(response) => {
                        println!("Received data: {:?}", response);
                    }
                    Err(e) => {
                        eprintln!("Error fetching data: {:?}", e);
                    }
                }
            }
        });

        self.handle = Some(handle);
    }

    // Handles graceful shutdown of the background task
    pub async fn shutdown(self) {
        if let Some(handle) = self.handle {
            handle.abort(); // Abort the background task
            println!("Background task aborted");
        }
    }

    // Wait for a shutdown signal (e.g., Ctrl+C)
    pub async fn wait_for_shutdown(self) {
        signal::ctrl_c().await.expect("Failed to listen for Ctrl+C");
        self.shutdown().await;
    }
}
