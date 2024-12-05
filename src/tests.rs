use crate::{
    db::db_types::ProposalWithLatestSnapshotView, rpc_service, types::PaginatedResponse, Env,
};
use near_sdk::AccountId;

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

#[test]
fn test_proposal_ids_are_continuous() {
    use rocket::local::blocking::Client;

    let client = Client::tracked(super::rocket()).expect("valid `Rocket`");

    for _ in 0..50 {
        let response = client
            .get("/proposals?order=id_asc&limit=50&offset=0}`")
            .dispatch();
        let result = response
            .into_json::<PaginatedResponse<ProposalWithLatestSnapshotView>>()
            .unwrap();
        if result.records.len() >= 50 {
            break;
        }
    }
    let response = client
        .get("/proposals?order=id_asc&limit=50&offset=0}`")
        .dispatch();
    let result = response
        .into_json::<PaginatedResponse<ProposalWithLatestSnapshotView>>()
        .unwrap();
    assert_eq!(result.records.len(), 50);
    let env: Env = envy::from_env::<Env>().expect("Failed to load environment variables");
    let account_id: AccountId = env.contract.parse().unwrap();

    for ndx in 0..50 {
        let proposal_id: i32 = ndx as i32;
        assert_eq!(result.records[ndx].proposal_id, proposal_id);
    }
}
