use crate::db::DB;
use crate::nearblocks_client;
use crate::nearblocks_client::proposal::{handle_edit_proposal, handle_set_block_height_callback};
use crate::nearblocks_client::rfp::{handle_edit_rfp, handle_set_rfp_block_height_callback};
use crate::nearblocks_client::types::Transaction;
use crate::rpc_service::{Env, RpcService};
use near_account_id::AccountId;
use rocket::State;

pub async fn fetch_all_new_transactions(
    nearblocks_client: &nearblocks_client::ApiClient,
    after_block: Option<i64>,
    max_transactions: Option<usize>,
) -> anyhow::Result<(Vec<Transaction>, String)> {
    let mut all_transactions = Vec::new();
    let mut current_cursor = "".to_string();

    dotenvy::dotenv().ok();
    let env: Env = envy::from_env::<Env>().expect("Failed to load environment variables");
    let contract: AccountId = env.contract.parse().expect("Failed to parse contract");

    loop {
        let response = match nearblocks_client
            .get_account_txns_by_pagination(
                contract.clone(),
                current_cursor.clone(),
                Some(25),
                Some("asc".to_string()),
                Some(1),
                after_block,
            )
            .await
        {
            Ok(response) => response,
            Err(e) => {
                eprintln!("Failed to fetch transactions from nearblocks: {:?}", e);
                break;
            }
        };

        println!(
            "Fetched {} transactions from nearblocks",
            response.txns.len()
        );

        if let Some(max_transactions) = max_transactions {
            if all_transactions.len() >= max_transactions {
                println!("Reached max transactions, finished fetching transactions");
                break;
            }
        }

        // Check if we've wrapped around or reached the end
        if response.cursor.is_none() {
            println!("Cursor has wrapped around, finished fetching transactions");
            all_transactions.extend(response.txns);
            current_cursor = "None".to_string();
            break;
        }

        // Add transactions to our collection
        all_transactions.extend(response.txns);

        // Update cursor for next iteration
        current_cursor = response.cursor.unwrap();

        // We only use the nearblocks api for the first sync. It has 150 calls / minute limit
        // So we sleep for 500ms to avoid hitting the limit. We don't expect to run
        // this more than once or twice after initialization
        tokio::time::sleep(std::time::Duration::from_millis(750)).await;
    }

    Ok((all_transactions, current_cursor))
}

pub async fn update_nearblocks_data(
    db: &State<DB>,
    rpc_service: &State<RpcService>,
    after_block: Option<i64>,
    max_transactions: Option<usize>,
) -> anyhow::Result<()> {
    let nearblocks_client = nearblocks_client::ApiClient::new();

    let (all_transactions, _current_cursor) =
        match fetch_all_new_transactions(&nearblocks_client, after_block, max_transactions).await {
            Ok(result) => result,
            Err(e) => {
                eprintln!("Error fetching transactions: {:?}", e);
                return Err(anyhow::anyhow!("Error fetching transactions: {:?}", e));
            }
        };

    println!("Total transactions fetched: {}", all_transactions.len());

    nearblocks_client::transactions::process(&all_transactions, db, rpc_service).await?;

    Ok(())
}

fn is_fatal_error(error: &anyhow::Error) -> bool {
    let error_msg = error.to_string();

    let non_fatal_errors = ["non-fatal"];

    !non_fatal_errors.iter().any(|&msg| error_msg.contains(msg))
}

pub async fn process(
    transactions: &[Transaction],
    db: &State<DB>,
    rpc_service: &State<RpcService>,
) -> anyhow::Result<()> {
    for transaction in transactions.iter() {
        if let Some(action) = transaction
            .actions
            .as_ref()
            .and_then(|actions| actions.first())
        {
            if !transaction.receipt_outcome.status {
                eprintln!(
                    "Proposal receipt outcome status is {:?}",
                    transaction.receipt_outcome.status
                );
                continue;
            }
            let result = match action.method.as_deref().unwrap_or("") {
                "set_block_height_callback" => {
                    handle_set_block_height_callback(transaction.to_owned(), db, rpc_service).await
                }
                "edit_proposal"
                | "edit_proposal_timeline"
                | "edit_proposal_versioned_timeline"
                | "edit_proposal_linked_rfp"
                | "edit_proposal_internal" => {
                    handle_edit_proposal(transaction.to_owned(), db, rpc_service).await
                }
                "edit_rfp_timeline"
                | "edit_rfp"
                | "edit_rfp_internal"
                | "edit_rfp_linked_proposal"
                | "edit_rfp_internal_linked_proposal"
                | "edit_rfp_internal_linked_proposal_timeline"
                | "cancel_rfp" => handle_edit_rfp(transaction.to_owned(), db, rpc_service).await,
                "set_rfp_block_height_callback" => {
                    println!("set_rfp_block_height_callback");
                    handle_set_rfp_block_height_callback(transaction.to_owned(), db, rpc_service)
                        .await
                }
                _ => {
                    continue;
                }
            };

            if let Err(e) = result {
                if is_fatal_error(&e) {
                    eprintln!("Fatal error, stopping: {:?}", e);
                    return Ok(());
                } else {
                    eprintln!("Non-fatal error, continuing: {:?}", e);
                    continue;
                }
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[tokio::test]
    #[ignore]
    async fn test_fetch_all_transactions() {
        let client = nearblocks_client::ApiClient::new();
        let (transactions, current_cursor) =
            fetch_all_new_transactions(&client, Some(0), Some(601))
                .await
                .expect("Error fetching transactions");

        // Check total count
        assert!(
            transactions.len() >= 600,
            "Expected at least 600 transactions, but got {}",
            transactions.len()
        );

        assert!(
            current_cursor == "None",
            "Current cursor should be None but is >{}<",
            current_cursor
        );

        // Check for duplicates
        let mut seen_transactions = HashSet::new();
        let mut duplicates = Vec::new();

        for tx in &transactions {
            if !seen_transactions.insert(&tx.id) {
                duplicates.push(tx.id.clone());
            }
        }

        assert!(
            duplicates.is_empty(),
            "Found {} duplicate transactions:\n{}",
            duplicates.len(),
            duplicates.join("\n")
        );

        println!("Total transactions found: {}", transactions.len());
    }
}
