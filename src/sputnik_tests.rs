use futures::future::join_all;
use near_api::prelude::Contract;

use crate::{db::db_types::SputnikProposalSnapshotRecord, types::PaginatedResponse, Env};
use crate::{separate_number_and_text, timestamp_to_date_string};
use near_sdk::AccountId;
use serde_json::{json, Value};

#[rocket::async_test]
async fn test_get_dao_proposals() {
    use rocket::local::asynchronous::Client;

    let client = Client::tracked(super::rocket())
        .await
        .expect("valid `Rocket`");

    let response = client
        .get("/dao/proposals/testing-treasury.sputnik-dao.near")
        .dispatch();
    let result = response
        .await
        .into_json::<PaginatedResponse<SputnikProposalSnapshotRecord>>()
        .await
        .unwrap();

    assert_eq!(result.records.len(), 50);
}

#[rocket::async_test]
async fn test_get_dao_proposals_limit_and_offset() {
    use rocket::local::asynchronous::Client;

    let client = Client::tracked(super::rocket())
        .await
        .expect("valid `Rocket`");

    let offset = 10;
    let response = client
        .get(format!(
            "/dao/proposals/testing-treasury.sputnik-dao.near?limit=10&offset={}&order=id_asc",
            offset
        ))
        .dispatch();
    let result = response
        .await
        .into_json::<PaginatedResponse<SputnikProposalSnapshotRecord>>()
        .await
        .unwrap();

    assert_eq!(result.records.len(), 10);

    assert_eq!(
        result.records[0].id, offset,
        "If offset is {}, the first proposal should have the id {}",
        offset, offset
    );
}

#[rocket::async_test]
async fn test_get_dao_proposals_order() {
    use rocket::local::asynchronous::Client;

    let client = Client::tracked(super::rocket())
        .await
        .expect("valid `Rocket`");

    let response = client
        .get("/dao/proposals/testing-treasury.sputnik-dao.near?order=id_asc")
        .dispatch();
    let result = response
        .await
        .into_json::<PaginatedResponse<SputnikProposalSnapshotRecord>>()
        .await
        .unwrap();

    assert_eq!(
        result.records[0].id, 0,
        " If order is id_asc, the first proposal should have the lowest id which is 0"
    );

    assert_eq!(
        result.records.len(),
        10,
        "If the limit is not set, the default limit should be 10"
    );

    let response = client
        .get("/dao/proposals/testing-treasury.sputnik-dao.near?order=id_desc")
        .dispatch();
    let result = response
        .await
        .into_json::<PaginatedResponse<SputnikProposalSnapshotRecord>>()
        .await
        .unwrap();

    assert_eq!(
        result.records[0].id , result.total_records as i32 - 1,
        "If order is id_desc, the first proposal with id {:?} should be the same as total records minus 1 {:?}",
        result.records[0].id, result.total_records - 1
    );

    // From RPC get all proposal ids
    // let account_id = "testing-treasury.sputnik-dao.near".parse().unwrap();
    // let contract = Contract(account_id);
    // let call_function_builder = contract.call_function("get_proposals", ()).unwrap();
    // let proposal_ids: Value = call_function_builder
    //     .read_only()
    //     .fetch_from_mainnet()
    //     .await
    //     .unwrap()
    //     .data;
    // let proposal_ids: Vec<i32> = proposal_ids
    //     .as_array()
    //     .unwrap()
    //     .iter()
    //     .map(|p| p.as_i64().unwrap() as i32)
    //     .collect();

    // assert_eq!(
    //     proposal_ids,
    //     result.records.iter().map(|p| p.id).collect::<Vec<i32>>()
    // );
}

#[rocket::async_test]
async fn test_get_dao_proposals_filters() {
    use rocket::local::asynchronous::Client;

    let client = Client::tracked(super::rocket())
        .await
        .expect("valid `Rocket`");

    // test filters
    let kind = "ChangePolicy";
    let status = "Pending";
    let total_votes = 2;
    let proposer = "testing-treasury.sputnik-dao.near";

    // Test Number of Votes
    let response = client
        .get(format!(
            "/dao/proposals/testing-treasury.sputnik-dao.near?total_votes={}",
            total_votes
        ))
        .dispatch();

    let result = response
        .await
        .into_json::<PaginatedResponse<SputnikProposalSnapshotRecord>>()
        .await
        .unwrap();

    assert!(
        result.records.len() > 1,
        "Expected more than 1 record, found {} for total votes {}",
        result.records.len(),
        total_votes
    );

    result.records.iter().for_each(|p| {
        assert_eq!(
            p.total_votes, total_votes,
            "Total votes should be {} but is {}",
            total_votes, p.total_votes
        );
    });

    // Test Status
    let response = client
        .get(format!(
            "/dao/proposals/testing-treasury.sputnik-dao.near?status={}",
            status
        ))
        .dispatch();

    let result = response
        .await
        .into_json::<PaginatedResponse<SputnikProposalSnapshotRecord>>()
        .await
        .unwrap();

    assert!(
        result.records.len() > 1,
        "Expected more than 1 record, found {} for status {}",
        result.records.len(),
        status
    );
    result.records.iter().for_each(|p| {
        assert_eq!(
            p.status, status,
            "Status should be {} but is {}",
            status, p.status
        );
    });

    // Test Kind
    let response = client
        .get(format!(
            "/dao/proposals/testing-treasury.sputnik-dao.near?kind={}",
            kind
        ))
        .dispatch();
    let result = response
        .await
        .into_json::<PaginatedResponse<SputnikProposalSnapshotRecord>>()
        .await
        .unwrap();

    assert!(
        result.records.len() > 1,
        "Expected more than 1 record, found {} for kind {}",
        result.records.len(),
        kind
    );
    result.records.iter().for_each(|p| {
        assert_eq!(p.kind, kind, "Kind should be {} but is {}", kind, p.kind);
    });

    // Test Proposer
    let response = client
        .get(format!(
            "/dao/proposals/testing-treasury.sputnik-dao.near?proposer={}",
            proposer
        ))
        .dispatch();
    let result = response
        .await
        .into_json::<PaginatedResponse<SputnikProposalSnapshotRecord>>()
        .await
        .unwrap();

    assert!(
        result.records.len() > 1,
        "Expected more than 1 record, found {} for proposer {}",
        result.records.len(),
        proposer
    );
    result.records.iter().for_each(|p| {
        assert_eq!(
            p.proposer, proposer,
            "Proposer should be {} but is {}",
            proposer, p.proposer
        );
    });
}
