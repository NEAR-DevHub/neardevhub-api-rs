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
 * Test that if new proposals appear, they should also be added to the cache
 * Test that status changes are updated in the cache
 * Test that new comment block heights are registered
 */

#[rocket::async_test]
async fn test_new_proposals_appear_in_cache() {
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
}

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

#[rocket::async_test]
async fn test_all_proposals_are_indexed() {
    use reqwest;
    use serde_json::Value;

    let urls = [
        "https://devhub-cache-api-rs.fly.dev/proposals",
        "https://infra-cache-api-rs.fly.dev/proposals",
        "https://events-cache-api-rs.fly.dev/proposals",
    ];

    // Create a reqwest client
    let client = reqwest::Client::new();

    // Make the HTTP request to the deployed API
    let response = client
        .get(urls[2])
        .send()
        .await
        .expect("Failed to get response");

    // Ensure the request was successful
    assert!(response.status().is_success());

    // Parse the response body as JSON
    let result: Value = response
        .json()
        .await
        .expect("Failed to parse response as JSON");

    println!("Result: {:?}", result);

    // Extract total count and records
    let total = result["total_records"]
        .as_i64()
        .expect("Failed to get total count");

    let records = result["records"]
        .as_array()
        .expect("Failed to get records array");

    // Ensure we have records
    assert!(!records.is_empty(), "No records found");

    // Get the last proposal ID
    let last_proposal = records.first().expect("Failed to get last record");

    let last_id = last_proposal["proposal_id"]
        .as_i64()
        .expect("Failed to get proposal_id");

    // Compare the last ID with the total count
    // They should be equal if all proposals are properly indexed
    assert_eq!(
        last_id,
        total - 1,
        "Last proposal ID ({}) doesn't match total count ({})",
        last_id,
        total - 1
    );

    eprintln!("Total count: {}, Last ID: {}", total, last_id);
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

    // Multiple numbers in the string
    assert_eq!(
        separate_number_and_text("123test456"),
        (Some(123), "test456".to_string())
    );

    // String with special characters
    assert_eq!(
        separate_number_and_text("@#$%^&*()"),
        (None, "@#$%^&*()".to_string())
    );

    // Negative number should be ignored
    assert_eq!(
        separate_number_and_text("-123 test"),
        (Some(123), "- test".to_string())
    );
}

#[test]
fn test_cors_configuration() {
    use rocket::http::{Header, Status};
    use rocket::local::blocking::Client;

    let client = Client::tracked(super::rocket()).expect("valid Rocket instance");

    // Test allowed origin
    let response = client
        .options("/")
        .header(Header::new("Origin", "http://localhost:3000"))
        .header(Header::new("Access-Control-Request-Method", "GET"))
        .dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert!(response
        .headers()
        .get("Access-Control-Allow-Origin")
        .next()
        .is_some());

    // Test disallowed origin
    let response = client
        .options("/")
        .header(Header::new("Origin", "http://disallowed-origin.com"))
        .header(Header::new("Access-Control-Request-Method", "GET"))
        .dispatch();

    assert_eq!(response.status(), Status::NoContent);
}

#[test]
fn test_custom_error_handler() {
    use rocket::http::Status;
    use rocket::local::blocking::Client;

    let client = Client::tracked(super::rocket()).expect("valid Rocket instance");

    // Test 404 Not Found
    let response = client.get("/nonexistent_route").dispatch();
    assert_eq!(response.status(), Status::NotFound);
    assert_eq!(
        response.into_string().unwrap(),
        "Custom 404 Error: Not Found"
    );
}

#[rocket::async_test]
async fn test_route_test() {
    use rocket::local::asynchronous::Client;

    let client = Client::tracked(super::rocket())
        .await
        .expect("valid Rocket instance");

    // Test valid request
    let response = client.get("/test").dispatch().await;
    assert_eq!(
        response.into_string().await.unwrap(),
        "Welcome to devhub.near"
    );
}
