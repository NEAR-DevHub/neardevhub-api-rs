use crate::db::DB;
use crate::nearblocks_client;
use crate::nearblocks_client::proposal::{handle_edit_proposal, handle_set_block_height_callback};
use crate::nearblocks_client::rfp::{handle_edit_rfp, handle_set_rfp_block_height_callback};
use crate::nearblocks_client::types::Transaction;
use near_account_id::AccountId;
use rocket::{http::Status, State};

pub async fn update_nearblocks_data(
    db: &DB,
    contract: &AccountId,
    nearblocks_api_key: &str,
    cursor: String,
) {
    let nearblocks_client = nearblocks_client::ApiClient::new(nearblocks_api_key.to_string());

    let nearblocks_unwrapped = match nearblocks_client
        .get_account_txns_by_pagination(
            contract.clone(),
            cursor.clone(),
            Some(50),
            Some("asc".to_string()),
            Some(1),
        )
        .await
    {
        Ok(response) => response,
        Err(e) => {
            eprintln!("Failed to fetch proposals from nearblocks: {:?}", e);
            return;
        }
    };

    println!(
        "Fetched {} method calls from nearblocks",
        nearblocks_unwrapped.txns.len()
    );

    if nearblocks_unwrapped.cursor.is_none()
        || nearblocks_unwrapped
            .cursor
            .as_ref()
            .expect("Something went wrong with the cursor")
            .parse::<i64>()
            .expect("Cursor is not a number")
            < cursor.parse::<i64>().unwrap_or(0)
    {
        println!("Cursor has wrapped around, no new transactions to process");
        return;
    }

    let _ =
        nearblocks_client::transactions::process(&nearblocks_unwrapped.txns, db.into(), contract)
            .await;

    let new_cursor = nearblocks_unwrapped.cursor.unwrap();
    eprintln!("New cursor is {}", new_cursor.clone());

    if let Some(transaction) = nearblocks_unwrapped.txns.last() {
        let timestamp_nano = transaction.block_timestamp.parse::<i64>().unwrap();
        let _ = db
            .set_last_updated_info(
                timestamp_nano,
                transaction.block.block_height,
                new_cursor.clone(),
            )
            .await;
    }
}

pub async fn process(
    transactions: &[Transaction],
    db: &State<DB>,
    contract: &AccountId,
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
                    handle_set_block_height_callback(transaction.to_owned(), db, contract).await
                }
                "edit_proposal" => handle_edit_proposal(transaction.to_owned(), db, contract).await,
                "edit_proposal_timeline" => {
                    handle_edit_proposal(transaction.to_owned(), db, contract).await
                }
                "edit_proposal_versioned_timeline" => {
                    handle_edit_proposal(transaction.to_owned(), db, contract).await
                }
                "edit_proposal_linked_rfp" => {
                    handle_edit_proposal(transaction.to_owned(), db, contract).await
                }
                "edit_proposal_internal" => {
                    handle_edit_proposal(transaction.to_owned(), db, contract).await
                }
                "edit_rfp_timeline" => {
                    println!("edit_rfp_timeline");
                    handle_edit_rfp(transaction.to_owned(), db, contract).await
                }
                "edit_rfp" => {
                    println!("edit_rfp");
                    handle_edit_rfp(transaction.to_owned(), db, contract).await
                }
                "edit_rfp_internal" => {
                    println!("edit_rfp_internal");
                    handle_edit_rfp(transaction.to_owned(), db, contract).await
                }
                "cancel_rfp" => {
                    println!("cancel_rfp");
                    handle_edit_rfp(transaction.to_owned(), db, contract).await
                }
                "set_rfp_block_height_callback" => {
                    println!("set_rfp_block_height_callback");
                    handle_set_rfp_block_height_callback(transaction.to_owned(), db, contract).await
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
