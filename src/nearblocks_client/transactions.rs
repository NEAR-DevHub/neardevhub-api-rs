use crate::db::DB;
use crate::nearblocks_client;
use crate::nearblocks_client::proposal::{handle_edit_proposal, handle_set_block_height_callback};
use crate::nearblocks_client::rfp::{handle_edit_rfp, handle_set_rfp_block_height_callback};
use crate::nearblocks_client::sputnik::{handle_act_proposal, handle_add_proposal};
use crate::nearblocks_client::types::Transaction;
use crate::rpc_service::{Env, RpcService};
use near_account_id::AccountId;
use rocket::{http::Status, State};

pub async fn fetch_all_new_transactions(
    nearblocks_client: &nearblocks_client::ApiClient,
    after_block: Option<i64>,
    passed_contract: Option<AccountId>,
) -> anyhow::Result<(Vec<Transaction>, String)> {
    let mut all_transactions = Vec::new();
    let mut current_cursor = "".to_string();

    dotenvy::dotenv().ok();
    let env: Env = envy::from_env::<Env>().expect("Failed to load environment variables");
    let env_contract: AccountId = env.contract.parse().expect("Failed to parse contract");

    let contract = match passed_contract {
        Some(c) => c,
        None => env_contract,
    };

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
) -> anyhow::Result<()> {
    let nearblocks_client = nearblocks_client::ApiClient::new();

    let (all_transactions, current_cursor) =
        match fetch_all_new_transactions(&nearblocks_client, after_block, None).await {
            Ok(result) => result,
            Err(e) => {
                eprintln!("Error fetching transactions: {:?}", e);
                return Err(anyhow::anyhow!("Error fetching transactions: {:?}", e));
            }
        };

    println!("Total transactions fetched: {}", all_transactions.len());

    if let Err(e) =
        nearblocks_client::transactions::process(&all_transactions, db, rpc_service).await
    {
        eprintln!("Error processing transactions: {:?}", e);
        return Err(anyhow::anyhow!("Error processing transactions: {:?}", e));
    }

    if let Some(transaction) = all_transactions.last() {
        let timestamp_nano = transaction.block_timestamp.parse::<i64>().unwrap();
        let _ = db
            .set_last_updated_info(
                timestamp_nano,
                transaction.block.block_height,
                current_cursor,
            )
            .await;
    }

    Ok(())
}

pub async fn update_dao_nearblocks_data(
    db: &DB,
    contract: &AccountId,
    rpc_service: &RpcService,
    after_block: Option<i64>,
) -> anyhow::Result<()> {
    let nearblocks_client = nearblocks_client::ApiClient::new();
    println!(
        "Fetching all new transactions for contract: {} starting from block: {}",
        contract,
        after_block.unwrap_or(0)
    );
    let (all_transactions, _) =
        match fetch_all_new_transactions(&nearblocks_client, after_block, Some(contract.clone()))
            .await
        {
            Ok(result) => result,
            Err(e) => {
                eprintln!("Error fetching transactions: {:?}", e);
                return Err(anyhow::anyhow!("Error fetching transactions: {:?}", e));
            }
        };

    println!("Total transactions fetched: {}", all_transactions.len());

    // Process transactions and get the last successful block height
    let last_successful_block =
        process_dao_transactions(&all_transactions, db.into(), contract, rpc_service).await?;

    // Only update the after_block if all transactions were processed successfully
    println!(
        "Setting last updated info for contract: {} with block_height: {}",
        contract, last_successful_block
    );

    Ok(())
}

pub async fn process_dao_transactions(
    transactions: &[Transaction],
    db: &State<DB>,
    contract: &AccountId,
    rpc_service: &RpcService,
) -> anyhow::Result<i64> {
    // Return the last successful block height
    let mut last_successful_block = None;

    for transaction in transactions.iter() {
        if let Some(action) = transaction
            .actions
            .as_ref()
            .and_then(|actions| actions.first())
        {
            if !transaction.receipt_outcome.status {
                eprintln!(
                    "Proposal receipt outcome status is {:?} with block hash: {}",
                    transaction.receipt_outcome.status, transaction.receipt_block.block_hash
                );
                continue;
            }

            // Process the transaction and propagate any errors
            match action.method.as_deref().unwrap_or("") {
                "add_proposal" => {
                    println!("add_proposal");
                    let block_height =
                        handle_add_proposal(transaction.to_owned(), db, contract, rpc_service)
                            .await?;
                    last_successful_block = Some(block_height);
                }
                "act_proposal" => {
                    println!("act_proposal");
                    let block_height =
                        handle_act_proposal(transaction.to_owned(), db, contract, rpc_service)
                            .await?;
                    last_successful_block = Some(block_height);
                }
                _ => {
                    if action.action == "FUNCTION_CALL" {
                        println!("Unhandled method: {:?}", action.method.as_ref().unwrap());
                    } else {
                        println!("Unhandled action: {:?}", action.action);
                    }
                    continue;
                }
            };
        }
    }

    last_successful_block
        .ok_or_else(|| anyhow::anyhow!("No transactions were processed successfully"))
}

pub async fn process(
    transactions: &[Transaction],
    db: &State<DB>,
    rpc_service: &State<RpcService>,
) -> Result<(), Status> {
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
                // eprintln!("On transaction: {:?}", transaction);
                continue;
            }
            let result = match action.method.as_deref().unwrap_or("") {
                "set_block_height_callback" => {
                    handle_set_block_height_callback(transaction.to_owned(), db, rpc_service).await
                }
                "edit_proposal" => {
                    handle_edit_proposal(transaction.to_owned(), db, rpc_service).await
                }
                "edit_proposal_timeline" => {
                    handle_edit_proposal(transaction.to_owned(), db, rpc_service).await
                }
                "edit_proposal_versioned_timeline" => {
                    handle_edit_proposal(transaction.to_owned(), db, rpc_service).await
                }
                "edit_proposal_linked_rfp" => {
                    handle_edit_proposal(transaction.to_owned(), db, rpc_service).await
                }
                "edit_proposal_internal" => {
                    handle_edit_proposal(transaction.to_owned(), db, rpc_service).await
                }
                "edit_rfp_timeline" => {
                    println!("edit_rfp_timeline");
                    handle_edit_rfp(transaction.to_owned(), db, rpc_service).await
                }
                "edit_rfp" => {
                    println!("edit_rfp");
                    handle_edit_rfp(transaction.to_owned(), db, rpc_service).await
                }
                "edit_rfp_internal" => {
                    println!("edit_rfp_internal");
                    handle_edit_rfp(transaction.to_owned(), db, rpc_service).await
                }
                "cancel_rfp" => {
                    println!("cancel_rfp");
                    handle_edit_rfp(transaction.to_owned(), db, rpc_service).await
                }
                "set_rfp_block_height_callback" => {
                    println!("set_rfp_block_height_callback");
                    handle_set_rfp_block_height_callback(transaction.to_owned(), db, rpc_service)
                        .await
                }
                _ => {
                    if action.action == "FUNCTION_CALL" {
                        // println!("Unhandled method: {:?}", action.method.as_ref().unwrap());
                    } else {
                        // println!("Unhandled action: {:?}", action.action);
                    }
                    continue;
                }
            };
            result?;
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
        let (transactions, current_cursor) = fetch_all_new_transactions(&client, Some(0), None)
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

        // Remove strict cursor assertion since it can vary
        println!("Total transactions found: {}", transactions.len());
        println!("Final cursor: {}", current_cursor);
    }
}
