use crate::db::DB;
use crate::nearblocks_client;
use crate::nearblocks_client::proposal::{handle_edit_proposal, handle_set_block_height_callback};
use crate::nearblocks_client::rfp::{handle_edit_rfp, handle_set_rfp_block_height_callback};
use crate::nearblocks_client::types::Transaction;
use crate::rpc_service::{Env, RpcService};
use near_account_id::AccountId;
use rocket::{http::Status, State};

pub async fn fetch_all_new_transactions(
    nearblocks_client: &nearblocks_client::ApiClient,
    after_block: Option<i64>,
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
) -> anyhow::Result<()> {
    let nearblocks_client = nearblocks_client::ApiClient::new();

    let (all_transactions, current_cursor) =
        match fetch_all_new_transactions(&nearblocks_client, after_block).await {
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
