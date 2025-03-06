use devhub_shared::proposal::{Proposal, VersionedProposal};
use futures::future::join_all;
use near_api::Contract;

use crate::db::db_types::LastUpdatedInfo;
use crate::entrypoints::proposal::proposal_types::ProposalBodyFields;
use crate::nearblocks_client::types::BLOCK_HEIGHT_OFFSET;
use crate::rpc_service::{self, RpcService};
use crate::{db::db_types::ProposalWithLatestSnapshotView, types::PaginatedResponse, Env};
use crate::{separate_number_and_text, timestamp_to_date_string};
use futures::StreamExt;
use near_sdk::AccountId;
use serde_json::{json, Value};

#[rocket::async_test]
async fn test_proposal_ids_continuous_name_status_matches() {
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

    assert_eq!(result.records.len(), 50, "Result records should be 50");

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
    let rpc_service = RpcService::new(&contract_account_id);

    // Create a Vec of futures for all blockchain calls
    let futures = result.records.iter().enumerate().map(|(ndx, record)| {
        let proposal_id = ndx as i32 + offset;
        let rpc_service = rpc_service.clone();
        let record = record.clone();

        async move {
            let proposal =
                Proposal::from(rpc_service.get_proposal(proposal_id).await.unwrap().data);

            // Return tuple of data needed for assertions
            (proposal_id, proposal, record)
        }
    });

    // Execute all futures concurrently
    let results = join_all(futures).await;

    // Perform assertions on results
    for (proposal_id, proposal, record) in results {
        assert_eq!(record.proposal_id, proposal_id);

        let proposal_snapshot_timeline: Value =
            serde_json::from_str(proposal.snapshot.body.get_timeline().as_str()).unwrap();
        eprintln!(
            "proposal {:?}, {:?}, {:?}, {:?}",
            proposal_id,
            record.block_height.unwrap(),
            proposal.snapshot.body.get_name(),
            proposal_snapshot_timeline["status"]
        );

        assert_eq!(
            proposal.snapshot.body.get_name().as_str(),
            record.name.unwrap()
        );

        let timeline: Value =
            serde_json::from_str(record.timeline.unwrap().as_str().unwrap()).unwrap();

        assert_eq!(proposal_snapshot_timeline["status"], timeline["status"]);
    }
}

#[rocket::async_test]
async fn test_if_the_last_ten_will_get_indexed() {
    use rocket::local::asynchronous::Client;

    let client = Client::tracked(super::rocket())
        .await
        .expect("valid `Rocket`");

    let contract_string: String =
        std::env::var("CONTRACT").unwrap_or_else(|_| "devhub.near".to_string());
    let contract_account_id: AccountId = contract_string.parse().unwrap();

    // Get all proposal ids from the RPC service
    let rpc_service = RpcService::new(&contract_account_id);
    let proposal_ids = rpc_service.get_all_proposal_ids().await.unwrap();
    let last_ten_ids: Vec<i32> = proposal_ids.iter().rev().take(10).cloned().collect();

    // Get the last 10 proposals
    let versioned_proposals: Vec<VersionedProposal> = futures::stream::iter(last_ten_ids)
        .then(|id| {
            let rpc_service = rpc_service.clone();
            async move { rpc_service.get_proposal(id).await.unwrap().data }
        })
        .collect()
        .await;

    let proposals: Vec<Proposal> = versioned_proposals
        .iter()
        .map(|vp| Proposal::from((*vp).clone()))
        .collect();

    // Set the block height to a recent block so we won't index from the start
    let block_height = proposals.last().unwrap().social_db_post_block_height;
    eprintln!("block_height: {:?}", block_height);
    let set_block_height = block_height as i64 - BLOCK_HEIGHT_OFFSET;
    let block_height_query = format!("/proposals/info/block/{}", set_block_height);
    let _ = client.get(block_height_query).dispatch().await;

    // Check that the blockheight is set
    let info = client
        .get("/proposals/info/".to_string())
        .dispatch()
        .await
        .into_json::<LastUpdatedInfo>()
        .await
        .unwrap();

    assert_eq!(
        info.after_block, set_block_height,
        "Block height should be set to {:?} but is {:?}",
        set_block_height, info.after_block
    );

    eprintln!("after_block: {:?}", info.after_block);

    // Get the last ten proposals from the API
    let limit = 10;
    let query = format!("/proposals?limit={}", limit);
    let result = client
        .get(query)
        .dispatch()
        .await
        .into_json::<PaginatedResponse<ProposalWithLatestSnapshotView>>()
        .await
        .unwrap();

    assert_eq!(proposals.len(), limit);
    assert_eq!(result.records.len(), limit);

    eprintln!(
        "Proposal IDs RPC: {:?}",
        proposals.iter().map(|p| p.id as i32).collect::<Vec<i32>>()
    );
    eprintln!(
        "Proposal IDs API: {:?}",
        result
            .records
            .iter()
            .map(|r| r.proposal_id)
            .collect::<Vec<i32>>()
    );
    // Compare the last 10 proposals from the API with the RPC
    for (record, proposal) in result.records.iter().zip(proposals.iter()) {
        assert_eq!(
            record.proposal_id, proposal.id as i32,
            "Proposal ID from the API {:?} doesn't match the RPC {:?} on contract {:?}",
            record.proposal_id, proposal.id, contract_string
        );
    }
}

#[rocket::async_test]
async fn test_all_proposals_are_indexed() {
    use near_sdk::AccountId;
    use reqwest;
    use serde_json::Value;
    use std::collections::HashMap;

    let contract_strings = vec![
        "devhub.near",
        "infrastructure-committee.near",
        "events-committee.near",
    ];

    let mut map = HashMap::new();
    map.insert(
        "devhub.near",
        "https://devhub-cache-api-rs.fly.dev/proposals",
    );
    map.insert(
        "infrastructure-committee.near",
        "https://infra-cache-api-rs.fly.dev/proposals",
    );
    map.insert(
        "events-committee.near",
        "https://events-cache-api-rs.fly.dev/proposals",
    );

    for contract_string in contract_strings {
        let account_id = contract_string.parse::<AccountId>().unwrap();
        let url = *map.get(contract_string).expect("Contract string not found");

        // Create a reqwest client
        let client = reqwest::Client::new();

        // Make the HTTP request to the deployed API
        let response = client
            .get(url)
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

        println!("Result for {}: {:?}", contract_string, result);

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

        let last_id_api = last_proposal["proposal_id"]
            .as_i64()
            .expect("Failed to get proposal_id");

        // rpc service
        let rpc_service = RpcService::new(&account_id);

        // Get all proposal ids
        let proposal_ids = rpc_service.get_all_proposal_ids().await;
        let proposal_ids = proposal_ids.unwrap();
        let total_ids_rpc = proposal_ids.len() as i64;
        let last_id_rpc = proposal_ids.last().copied().unwrap();
        assert_eq!(
            last_id_rpc, last_id_api as i32,
            "Last proposal ID from the RPC {:?} doesn't match the API {:?} on contract {:?}",
            last_id_rpc, last_id_api, contract_string
        );

        // Compare the last ID with the total count
        // They should be equal if all proposals are properly indexed
        assert_eq!(
            last_id_api,
            total - 1,
            "Last proposal ID from the API ({}) doesn't match total count ({}) on contract {:?}",
            last_id_api,
            total - 1,
            contract_string
        );

        assert_eq!(
            total_ids_rpc, total,
            "Total count from the RPC {:?} doesn't match the API {:?} on contract {:?}",
            total_ids_rpc, total, contract_string
        );

        eprintln!("Total count: {}, Last ID: {}", total, last_id_api);
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
    let res = client
        .get("/")
        .header(Header::new("Origin", "http://localhost:3000"))
        .dispatch();
    assert_eq!(
        res.status(),
        Status::Ok,
        "Response should be Ok 200 but is {:?}",
        res.status()
    );
    assert!(res
        .headers()
        .get("Access-Control-Allow-Origin")
        .next()
        .is_some());

    // Test disallowed origin
    let response = client
        .get("/")
        .header(Header::new("Origin", "http://disallowed-origin.com"))
        .header(Header::new("Access-Control-Request-Method", "GET"))
        .dispatch();

    assert_eq!(
        response.status(),
        Status::Forbidden,
        "Response should be Forbidden 403 but is {:?}",
        response.status()
    );
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
