use crate::nearblocks_client;
use crate::nearblocks_client::types::Transaction;
use near_account_id::AccountId;

pub async fn fetch_all_new_transactions(
    nearblocks_client: &nearblocks_client::ApiClient,
    contract: &AccountId,
    after_block: Option<i64>,
) -> (Vec<Transaction>, String) {
    let mut all_transactions = Vec::new();
    let mut current_cursor = "".to_string();

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
    }

    (all_transactions, current_cursor)
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
