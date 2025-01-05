use crate::db::DB;
use crate::nearblocks_client;
use crate::nearblocks_client::proposal::{handle_edit_proposal, handle_set_block_height_callback};
use crate::nearblocks_client::rfp::{handle_edit_rfp, handle_set_rfp_block_height_callback};
use crate::nearblocks_client::sputnik::{handle_act_proposal, handle_add_proposal};
use crate::nearblocks_client::types::Transaction;
use near_account_id::AccountId;
use rocket::{http::Status, State};

async fn fetch_all_new_transactions(
    nearblocks_client: &nearblocks_client::ApiClient,
    contract: &AccountId,
    after_block: Option<i64>,
) -> (Vec<Transaction>, String) {
    let mut all_transactions = Vec::new();
    let mut current_cursor = "".to_string();

    // TODO remove
    // if contract == "testing-astradao.sputnik-dao.near" {
    //     // Read from file
    //     let file_path = format!("transactions_{}.json", contract.to_string());
    //     let json_data = std::fs::read_to_string(file_path).unwrap();
    //     let transactions: Vec<Transaction> = serde_json::from_str(&json_data).unwrap();
    //     return (transactions.into_iter().rev().collect(), "None".to_string());
    // }

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
        if response.cursor.is_none() || contract == "testing-astradao.sputnik-dao.near" {
            println!("Cursor has wrapped around, finished fetching transactions");
            all_transactions.extend(response.txns);
            current_cursor = "None".to_string();
            break;
        }

        // Add transactions to our collection
        all_transactions.extend(response.txns);

        // For testing purposes write to a json file
        // let file_path = format!("transactions_{}.json", contract.to_string());
        // let json_data = serde_json::to_string(&all_transactions).unwrap();
        // std::fs::write(file_path, json_data).unwrap();

        // Update cursor for next iteration
        current_cursor = response.cursor.unwrap();
    }

    (all_transactions, current_cursor)
}

pub async fn update_nearblocks_data(
    db: &DB,
    contract: &AccountId,
    nearblocks_api_key: &str,
    after_block: Option<i64>,
) {
    let nearblocks_client = nearblocks_client::ApiClient::new(nearblocks_api_key.to_string());

    let (all_transactions, current_cursor) =
        fetch_all_new_transactions(&nearblocks_client, contract, after_block).await;

    println!("Total transactions fetched: {}", all_transactions.len());

    let _ = nearblocks_client::transactions::process(&all_transactions, db.into(), contract).await;

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
}

pub async fn update_dao_via_nearblocks(
    db: &DB,
    contract: &AccountId,
    nearblocks_api_key: &str,
    after_block: Option<i64>,
) {
    let nearblocks_client = nearblocks_client::ApiClient::new(nearblocks_api_key.to_string());

    let (all_transactions, _) =
        fetch_all_new_transactions(&nearblocks_client, contract, after_block).await;

    println!("Total transactions fetched: {}", all_transactions.len());

    let _ = nearblocks_client::transactions::process_dao_transactions(
        &all_transactions,
        db.into(),
        contract,
    )
    .await;

    if let Some(transaction) = all_transactions.last() {
        let timestamp_nano = transaction.block_timestamp.parse::<i64>().unwrap();
        let _ = db
            .set_last_updated_info_for_contract(
                contract,
                timestamp_nano,
                transaction.block.block_height,
            )
            .await;
    }
}

pub async fn process_dao_transactions(
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
                // TODO can't reuse this because the other contract has a function with the same name
                "add_proposal" => {
                    println!("add_proposal");
                    handle_add_proposal(transaction.to_owned(), db, contract).await
                }
                // TODO: Uncomment this
                // "act_proposal" => {
                //     println!("act_proposal");
                //     handle_act_proposal(transaction.to_owned(), db, contract).await
                // }
                _ => {
                    if action.action == "FUNCTION_CALL" {
                        println!("Unhandled method: {:?}", action.method.as_ref().unwrap());
                    } else {
                        println!("Unhandled action: {:?}", action.action);
                    }
                    continue;
                }
            };
            result?;
        }
    }

    Ok(())
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[tokio::test]
    async fn test_fetch_all_transactions() {
        let api_key = std::env::var("NEARBLOCKS_API_KEY")
            .expect("NEARBLOCKS_API_KEY environment variable not set");
        let client = nearblocks_client::ApiClient::new(api_key);
        let contract: AccountId = "infrastructure-committee.near"
            .parse()
            .expect("Invalid account ID");

        let (transactions, current_cursor) =
            fetch_all_new_transactions(&client, &contract, Some(0)).await;

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
