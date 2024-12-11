use futures::future::join_all;
use near_api::prelude::Contract;

use crate::{db::db_types::ProposalWithLatestSnapshotView, types::PaginatedResponse, Env};
use crate::{separate_number_and_text, timestamp_to_date_string};
use near_sdk::AccountId;
use serde_json::{json, Value};

/**
 * Test Nearblocks mocked transactions
 */

/**
 * Search Proposals
 * Get Proposals
 * Get Proposal Snapshots
 * Search RFPs
 * Get RFPs
 * Get RFP Snapshots
 */

#[rocket::async_test]
async fn test_proposal_ids_are_continuous_and_name_and_status_matches() {
    use rocket::local::asynchronous::Client;

    let client = Client::tracked(super::rocket())
        .await
        .expect("valid `Rocket`");
    let offset = 100;
    let limit = 50;
    let query = format!("/proposals?order=id_asc&limit={}&offset={}", limit, offset);
    // First page
    let response = client.get(query).dispatch();
    let result = response
        .await
        .into_json::<PaginatedResponse<ProposalWithLatestSnapshotView>>()
        .await
        .unwrap();

    let env = std::env::var("ENV").unwrap_or_else(|_| "LOCAL".to_string());
    let result = if env == "GH_ACTION" {
        let file = std::fs::File::open("test/result.json").expect("Unable to open file");
        serde_json::from_reader(file).expect("Unable to parse JSON")
    } else {
        result
    };

    assert_eq!(result.records.len(), 50);

    eprintln!(
        "Results {:?}",
        result
            .records
            .clone()
            .into_iter()
            .map(|r| r.proposal_id)
            .collect::<Vec<i32>>()
    );

    let env: Env = envy::from_env::<Env>().expect("Failed to load environment variables");
    let contract_account_id: AccountId = env.contract.parse().unwrap();
    let contract = Contract(contract_account_id);

    // Create a Vec of futures for all blockchain calls
    let futures = result.records.iter().enumerate().map(|(ndx, record)| {
        let proposal_id = ndx as i32 + offset;
        let contract = contract.clone();
        let record = record.clone();

        async move {
            let call = contract
                .call_function("get_proposal", json!({"proposal_id": proposal_id}))
                .unwrap();
            let proposal: Value = call.read_only().fetch_from_mainnet().await.unwrap().data;

            // Return tuple of data needed for assertions
            (proposal_id, proposal, record)
        }
    });

    // Execute all futures concurrently
    let results = join_all(futures).await;

    // Perform assertions on results
    for (proposal_id, proposal, record) in results {
        assert_eq!(record.proposal_id, proposal_id);

        eprintln!(
            "proposal {:?}, {:?}, {:?}, {:?}",
            proposal_id,
            record.block_height.unwrap(),
            proposal["snapshot"]["name"],
            proposal["snapshot"]["timeline"]["status"]
        );

        assert_eq!(
            proposal["snapshot"]["name"].as_str().unwrap(),
            record.name.unwrap()
        );

        let timeline: Value =
            serde_json::from_str(record.timeline.unwrap().as_str().unwrap()).unwrap();

        assert_eq!(
            proposal["snapshot"]["timeline"]["status"],
            timeline["status"]
        );
    }
}

#[test]
fn test_index() {
    use rocket::local::blocking::Client;

    // Construct a client to use for dispatching requests.
    let client = Client::tracked(super::rocket()).expect("valid `Rocket`");

    // Dispatch a request to 'GET /' and validate the response.
    let response = client.get("/").dispatch();
    assert_eq!(response.into_string().unwrap(), "Welcome from fly.io!!!!!");
}

#[test]
fn test_timestamp_to_date_string() {
    // Test regular date
    assert_eq!(timestamp_to_date_string(1704067200000000000), "2024-01-01");

    // Test edge cases
    assert_eq!(timestamp_to_date_string(0), "1970-01-01");

    // Test negative timestamp
    assert_eq!(timestamp_to_date_string(-86400000000000), "1969-12-31");
}

#[test]
fn test_separate_number_and_text() {
    // Test normal case
    assert_eq!(
        separate_number_and_text("123 test"),
        (Some(123), "test".to_string())
    );

    // Test no number
    assert_eq!(separate_number_and_text("test"), (None, "test".to_string()));

    // Test only number
    assert_eq!(separate_number_and_text("123"), (Some(123), "".to_string()));

    // Test number at end
    assert_eq!(
        separate_number_and_text("test 123"),
        (Some(123), "test".to_string())
    );
}
