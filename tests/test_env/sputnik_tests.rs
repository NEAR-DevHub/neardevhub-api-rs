// use super::*;
use devhub_cache_api::{
    db::db_types::SputnikProposalSnapshotRecord,
    entrypoints::sputnik::sputnik_types::{Proposal, ProposalOutput},
    rpc_service::RpcService,
    types::PaginatedResponse,
};
// use anyhow::Result;
use near_api::Contract;
use near_sdk::AccountId;
#[rocket::async_test]
async fn test_get_dao_proposals() {
    use rocket::local::asynchronous::Client;
    // let rpc_service = RpcService::sandbox(worker.into(), contract_account.id().clone());
    let rpc_service = RpcService::new();
    let client = Client::tracked(devhub_cache_api::rocket(Some(rpc_service)))
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
    // let rpc_service = RpcService::sandbox(worker.into(), contract_account.id().clone());
    let rpc_service = RpcService::new();
    let client = Client::tracked(devhub_cache_api::rocket(Some(rpc_service)))
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
        result.records[0].proposal_id, offset,
        "If offset is {}, the first proposal should have the id {}",
        offset, offset
    );
}

#[rocket::async_test]
async fn test_get_dao_proposals_order() {
    use rocket::local::asynchronous::Client;
    // let rpc_service = RpcService::sandbox(worker.into(), contract_account.id().clone());
    let rpc_service = RpcService::new();
    let client = Client::tracked(devhub_cache_api::rocket(Some(rpc_service)))
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
        result.records[0].proposal_id, 0,
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
        result.records[0].proposal_id, result.total_records as i32 - 1,
        "If order is id_desc, the first proposal with id {:?} should be the same as total records minus 1 {:?}",
        result.records[0].proposal_id, result.total_records - 1
    );
}

#[rocket::async_test]
async fn test_get_dao_proposals_filters() {
    use rocket::local::asynchronous::Client;
    // let rpc_service = RpcService::sandbox(worker.into(), contract_account.id().clone());
    let rpc_service = RpcService::new();
    let client = Client::tracked(devhub_cache_api::rocket(Some(rpc_service)))
        .await
        .expect("valid `Rocket`");

    let total_votes_options = [0, 1, 2];

    for total_votes in total_votes_options {
        // Test Number of Votes
        let response = client
            .get(format!(
                "/dao/proposals/testing-treasury.sputnik-dao.near?filters.total_votes={}",
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
    }

    let status_options = ["InProgress", "Approved", "Failed"];
    for status in status_options {
        // Test Status
        let response = client
            .get(format!(
                "/dao/proposals/testing-treasury.sputnik-dao.near?filters.status={}",
                status
            ))
            .dispatch();

        let result = response
            .await
            .into_json::<PaginatedResponse<SputnikProposalSnapshotRecord>>()
            .await
            .unwrap();

        assert!(
            !result.records.is_empty(),
            "Expected at least 1 record, found {} for status {}",
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
    }

    let kind_options = [
        "ChangePolicy",
        "FunctionCall",
        "ChangePolicyUpdateParameters",
        "AddMemberToRole",
    ];
    for kind in kind_options {
        // Test Kind
        let response = client
            .get(format!(
                "/dao/proposals/testing-treasury.sputnik-dao.near?filters.kind={}",
                kind
            ))
            .dispatch();
        let result = response
            .await
            .into_json::<PaginatedResponse<SputnikProposalSnapshotRecord>>()
            .await
            .unwrap();

        assert!(
            !result.records.is_empty(),
            "Expected at least 1 record, found {} for kind {}",
            result.records.len(),
            kind
        );
        result.records.iter().for_each(|p| {
            assert_eq!(p.kind, kind, "Kind should be {} but is {}", kind, p.kind);
        });
    }

    // Test Proposer
    let proposers = vec!["megha19.near", "thomasguntenaar.near", "freski.near"];
    for proposer in proposers {
        let response = client
            .get(format!(
                "/dao/proposals/testing-treasury.sputnik-dao.near?filters.proposer={}",
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
}

#[rocket::async_test]
async fn test_removed_proposals_have_correct_status() {
    use rocket::local::asynchronous::Client;

    // The following proposal IDs (28, 72, 79) on testing-astradao.sputnik-dao.near
    // were initially created but later voted to be removed. We verify that these proposals
    // are still indexed in the API but their status is correctly set to "Removed"
    // let rpc_service = RpcService::sandbox(worker.into(), contract_account.id().clone());
    let rpc_service = RpcService::new();
    let client = Client::tracked(devhub_cache_api::rocket(Some(rpc_service)))
        .await
        .expect("valid `Rocket`");

    let response = client
        .get("/dao/proposals/testing-treasury.sputnik-dao.near?filters.status=Removed")
        .dispatch();

    let result = response
        .await
        .into_json::<PaginatedResponse<SputnikProposalSnapshotRecord>>()
        .await
        .unwrap();

    eprintln!(
        "result: {:?}",
        result
            .records
            .iter()
            .map(|r| r.id.clone())
            .collect::<Vec<String>>()
    );
    assert!(result.records.len() >= 3);
    assert_eq!(result.records[0].status, "Removed");
    assert_eq!(result.records[1].status, "Removed");
    assert_eq!(result.records[2].status, "Removed");
}

#[rocket::async_test]
async fn test_search_proposals_on_description_or_hash() {
    use rocket::local::asynchronous::Client;
    use urlencoding;
    // let rpc_service = RpcService::sandbox(worker.into(), contract_account.id().clone());
    let rpc_service = RpcService::new();
    let client = Client::tracked(devhub_cache_api::rocket(Some(rpc_service)))
        .await
        .expect("valid `Rocket`");

    let search_term = "Testing multiple approvals with a single rejection";
    let formatted_term = search_term.to_lowercase();
    let encoded_search_term = urlencoding::encode(&formatted_term);

    assert_eq!(
        encoded_search_term,
        "testing%20multiple%20approvals%20with%20a%20single%20rejection"
    );

    let response = client
        .get(format!("/dao/proposals/search/{}", encoded_search_term))
        .dispatch();

    let result = response
        .await
        .into_json::<PaginatedResponse<SputnikProposalSnapshotRecord>>()
        .await
        .unwrap();

    eprintln!("result: {:?}", result);
    assert_eq!(result.records.len(), 1);

    let search_hash = "5YeFkwbL426gv7G6EHYKScW6hYnVA5JgS5Cmde95xHFs";
    let formatted_hash = search_hash.to_lowercase();
    let encoded_search_hash = urlencoding::encode(&formatted_hash);

    let response = client
        .get(format!("/dao/proposals/search/{}", encoded_search_hash))
        .dispatch();

    let result = response
        .await
        .into_json::<PaginatedResponse<SputnikProposalSnapshotRecord>>()
        .await
        .unwrap();

    eprintln!("result: {:?}", result);
    assert_eq!(result.records.len(), 1);
}

#[rocket::async_test]
async fn test_if_the_last_ten_will_get_indexed() {
    use futures::StreamExt;
    use rocket::local::asynchronous::Client;
    // let rpc_service = RpcService::sandbox(worker.into(), contract_account.id().clone());
    let rpc_service = RpcService::new();
    let client = Client::tracked(devhub_cache_api::rocket(Some(rpc_service)))
        .await
        .expect("valid `Rocket`");

    let contract_string: String = std::env::var("SPUTNIK_CONTRACT")
        .unwrap_or_else(|_| "testing-treasury.sputnik-dao.near".to_string());
    let contract_account_id: AccountId = contract_string.parse().unwrap();
    let contract = Contract(contract_account_id.clone());
    println!("contract_account_id: {:?}", contract_account_id);
    // Get all proposal ids from the RPC service
    let rpc_service = RpcService::mainnet(contract_account_id);
    let last_proposal_id = match rpc_service
        .get_last_proposal_id(contract.clone(), None)
        .await
    {
        Ok(response) => response,
        Err(e) => {
            eprintln!("Failed to get the last proposal ID: {:?}", e);
            101
        }
    };

    let last_ten_ids: Vec<i64> = (0..10).map(|i| last_proposal_id - i64::from(i)).collect();

    // Get the last 10 proposals
    let last_ten_proposal_outputs: Vec<ProposalOutput> = futures::stream::iter(last_ten_ids)
        .then(|id| {
            let rpc_service = rpc_service.clone();
            let contract = contract.clone();
            async move {
                rpc_service
                    .get_dao_proposal(contract, id - 1, None)
                    .await
                    .unwrap()
            }
        })
        .collect()
        .await;

    let proposals: Vec<Proposal> = last_ten_proposal_outputs
        .iter()
        .map(|last_ten| last_ten.proposal.clone())
        .collect();

    // Get the last ten proposals from the API
    let limit = 10;
    let query = format!("/dao/proposals/{}/?limit={}", contract_string, limit);
    let result = client
        .get(query)
        .dispatch()
        .await
        .into_json::<PaginatedResponse<SputnikProposalSnapshotRecord>>()
        .await
        .unwrap();

    assert_eq!(proposals.len(), limit);
    assert_eq!(result.records.len(), limit);

    eprintln!(
        "Proposal submission times RPC: {:?}",
        proposals
            .iter()
            .map(|p| p.submission_time.0)
            .collect::<Vec<u64>>()
    );
    eprintln!(
        "Proposal submission_times API: {:?}",
        result
            .records
            .iter()
            .map(|r| r.submission_time)
            .collect::<Vec<i64>>()
    );

    // TODO: 95 is missing from the API

    // Compare the last 10 proposals from the API with the RPC
    for (record, proposal) in result.records.iter().zip(proposals.iter()) {
        assert_eq!(
            record.submission_time, proposal.submission_time.0 as i64,
            "Proposal submission time from the API {:?} doesn't match the RPC {:?} on contract {:?}",
            record.submission_time, proposal.submission_time.0 as i64, contract_string
        );
    }
}
