use near_api::prelude::{Contract, Reference};

use crate::{
    db::db_types::ProposalWithLatestSnapshotView, rpc_service, types::PaginatedResponse, Env,
};
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

#[test]
fn test_index() {
    use rocket::local::blocking::Client;

    // Construct a client to use for dispatching requests.
    let client = Client::tracked(super::rocket()).expect("valid `Rocket`");

    // Dispatch a request to 'GET /' and validate the response.
    let response = client.get("/").dispatch();
    assert_eq!(response.into_string().unwrap(), "Welcome from fly.io!!!!!");
}

#[rocket::async_test]
async fn test_proposal_ids_are_continuous_and_name_and_status_matches() {
    use rocket::local::asynchronous::Client;

    let client = Client::tracked(super::rocket())
        .await
        .expect("valid `Rocket`");

    let response = client
        .get("/proposals?order=id_asc&limit=50&offset=0}`")
        .dispatch();
    let result = response
        .await
        .into_json::<PaginatedResponse<ProposalWithLatestSnapshotView>>()
        .await
        .unwrap();
    assert_eq!(result.records.len(), 50);

    let env: Env = envy::from_env::<Env>().expect("Failed to load environment variables");
    let contract_account_id: AccountId = env.contract.parse().unwrap();
    let contract = Contract(contract_account_id);

    for ndx in 0..result.records.len() {
        let proposal_id: i32 = ndx as i32;
        assert_eq!(result.records[ndx].proposal_id, proposal_id);

        let call = contract
            .call_function("get_proposal", json!({"proposal_id": proposal_id}))
            .unwrap();
        let proposal: Value = call
            .read_only()
            .fetch_from_mainnet()
            .await
            .unwrap()
            .data;
        eprintln!(
            "proposal {:?}, {:?}, {:?}, {:?}",
            proposal_id,
            result.records[ndx].block_height.unwrap(),
            proposal["snapshot"]["name"],
            proposal["snapshot"]["timeline"]["status"]
        );
        assert_eq!(
            proposal["snapshot"]["name"].as_str().unwrap(),
            result.records[ndx].clone().name.unwrap()
        );

        let timeline: Value = serde_json::from_str(
            result.records[ndx]
                .clone()
                .timeline
                .unwrap()
                .as_str()
                .unwrap(),
        )
        .unwrap();
        eprint!("timeline {:?}", timeline);
        assert_eq!(
            proposal["snapshot"]["timeline"]["status"],
            timeline["status"]
        );
    }
}
