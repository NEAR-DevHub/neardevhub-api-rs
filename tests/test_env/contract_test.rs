use super::*;
use anyhow::Result;
use devhub_cache_api::{
    db::db_types::{ProposalWithLatestSnapshotView, RfpWithLatestSnapshotView},
    rpc_service::RpcService,
    PaginatedResponse,
};
use near_sdk::{AccountIdRef, NearToken};
use near_workspaces::types::{AccessKey, KeyType, SecretKey};
use rocket::{http::Status, local::asynchronous::Client};
use serde_json::json;

const TEST_SEED: &str = "test_seed_string";
const DEVHUB_CONTRACT_PREFIX: &str = "devhub";
const NEAR_SOCIAL: &AccountIdRef = AccountIdRef::new_or_panic("social.near");

async fn setup_test_env() -> Result<(Worker<Sandbox>, Contract, Account)> {
    let worker = near_workspaces::sandbox().await?;
    let mainnet = near_workspaces::mainnet_archival().await?;

    // You'll need to provide the path to your contract's WASM file
    let wasm_path = Path::new("tests/devhub.wasm"); // Update this path
    let wasm =
        std::fs::read(wasm_path).map_err(|err| anyhow!("Error reading contract WASM: {}", err))?;

    // NEAR social deployment
    let near_social = worker
        .import_contract(&NEAR_SOCIAL.to_owned(), &mainnet)
        .initial_balance(NearToken::from_near(10000))
        .transact()
        .await?;
    near_social.call("new").transact().await?.into_result()?;
    near_social
        .call("set_status")
        .args_json(json!({
            "status": "Live"
        }))
        .transact()
        .await?
        .into_result()?;

    // Create test accounts with specific names
    let sk = SecretKey::from_seed(KeyType::ED25519, TEST_SEED);
    let tla_near = Account::from_secret_key("near".parse()?, sk.clone(), &worker);

    // Set up the account with full access
    worker
        .patch(tla_near.id())
        .access_key(sk.public_key(), AccessKey::full_access())
        .transact()
        .await?;

    let contract_account = tla_near
        .create_subaccount(DEVHUB_CONTRACT_PREFIX)
        .initial_balance(NearToken::from_near(100))
        .transact()
        .await?
        .into_result()?;

    // Deploy the contract
    let contract = contract_account.deploy(&wasm).await?.into_result()?;

    // Initialize the contract
    let outcome = contract.call("new").args_json(json!({})).transact().await?;

    assert!(outcome.is_success());
    assert!(format!("{:?}", outcome).contains("Migrated to version:"));

    Ok((worker, contract, contract_account))
}

// RUST_BACKTRACE=1 cargo test -p devhub-cache-api test_proposal_and_rfp_indexing -- --nocapture
#[tokio::test]
async fn test_proposal_and_rfp_indexing() -> Result<()> {
    let (worker, devhub_contract, contract_account) = setup_test_env().await?;

    // Create a proposal using near-api-rs
    let proposal_result = devhub_contract
        .call("add_proposal")
        .args_json(json!({
            "labels": [],
            "body": {
                "proposal_body_version": "V0",
                "name": "Test Proposal",
                "description": "This is a test proposal to verify the indexing functionality",
                "category": "Marketing",
                "summary": "Test proposal for indexing verification",
                "linked_proposals": [],
                "requested_sponsorship_usd_amount": "1",
                "requested_sponsorship_paid_in_currency": "USDT",
                "receiver_account": "thomasguntenaar.near",
                "supervisor": "theori.near",
                "requested_sponsor": "neardevdao.near",
                "timeline": {
                    "status": "DRAFT"
                }
            },
            "accepted_terms_and_conditions_version": 0
        }))
        .deposit(NearToken::from_near(1))
        .max_gas()
        .transact()
        .await?;

    if !proposal_result.is_success() {
        println!("Proposal creation failed with error: {:?}", proposal_result);
    }

    assert!(proposal_result.is_success());

    // Time travel
    let blocks_to_advance = 100;
    worker.fast_forward(blocks_to_advance).await?;

    // Create an RFP
    let rfp_result = devhub_contract
        .call("add_rfp")
        .args_json(json!({
            "body": {
                "rfp_body_version": "V0",
                "name": "Some RFP",
                "description": "some description",
                "summary": "sum",
                "timeline": {"status": "ACCEPTING_SUBMISSIONS"},
                "submission_deadline": "1707821848175250170"
            },
            "labels": [],
        }))
        .deposit(NearToken::from_near(1))
        .max_gas()
        .transact()
        .await?;

    println!("RFP Result: {:?}", rfp_result);
    assert!(rfp_result.is_success());

    // Get_changelog from the RPC from contract
    let change_log_result: serde_json::Value = devhub_contract
        .call("get_change_log")
        .args_json(json!({}))
        .view()
        .await?
        .json()?;

    println!("Full changelog: {:?}", change_log_result);
    assert!(change_log_result.as_array().unwrap().len() == 2);

    // Get the block_id from the first change
    let first_block_id = change_log_result.as_array().unwrap()[0]["block_id"]
        .as_i64()
        .unwrap();

    // Get_changelog_since from the RPC from contract
    let change_log_since_result: serde_json::Value = devhub_contract
        .call("get_change_log_since")
        .args_json(json!({ "since": first_block_id + 1 }))
        .view()
        .await?
        .json()?;

    println!(
        "Changelog since block {}: {:?}",
        first_block_id + 1,
        change_log_since_result
    );

    // assert change_log_since_result to only have the rfp change
    assert!(change_log_since_result.as_array().unwrap().len() == 1);

    // Wait for indexing
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;

    println!("Contract account ID: {:?}", contract_account.id());
    let rpc_service = RpcService::sandbox(worker.into(), contract_account.id().clone());
    // Index data through the API
    let client = Client::tracked(devhub_cache_api::rocket(Some(rpc_service)))
        .await
        .expect("valid `Rocket`");

    // Indexing
    let indexing_response = client.get("/proposals").dispatch().await;
    assert!(indexing_response.status() == Status::Ok);
    let indexing_result = indexing_response
        .into_json::<PaginatedResponse<ProposalWithLatestSnapshotView>>()
        .await
        .unwrap();
    assert!(!indexing_result.records.is_empty());

    // Check search
    let search_response = client
        .get("/proposals/search/Test Proposal")
        .dispatch()
        .await;
    assert!(search_response.status() == Status::Ok);
    let search_result = search_response
        .into_json::<PaginatedResponse<ProposalWithLatestSnapshotView>>()
        .await
        .unwrap();
    assert!(!search_result.records.is_empty());

    let rfp_search_response = client.get("/rfps/search/Some RFP").dispatch().await;
    assert!(rfp_search_response.status() == Status::Ok);
    let rfp_search_result = rfp_search_response
        .into_json::<PaginatedResponse<RfpWithLatestSnapshotView>>()
        .await
        .unwrap();
    println!("RFP search response: {:?}", rfp_search_result);
    assert!(!rfp_search_result.records.is_empty());

    Ok(())
}
