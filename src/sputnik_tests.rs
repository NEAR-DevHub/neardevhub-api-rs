use crate::{db::db_types::SputnikProposalSnapshotRecord, types::PaginatedResponse};

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
}

#[rocket::async_test]
async fn test_get_dao_proposals_filters() {
    use rocket::local::asynchronous::Client;

    let client = Client::tracked(super::rocket())
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
