use crate::db::db_types::{LastUpdatedInfo, SputnikProposalSnapshotRecord};
use crate::db::DB;
use crate::nearblocks_client::transactions::update_dao_nearblocks_data;
use crate::rpc_service::RpcService;
use crate::types::PaginatedResponse;
use near_account_id::AccountId;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{get, FromForm, State};
use std::convert::TryInto;
use std::str::FromStr;
use utoipa::ToSchema;
pub mod policy;
pub mod sputnik_types;

#[derive(Clone, Debug, FromForm, ToSchema)]
pub struct GetDaoProposalsFilters {
    pub proposer: Option<String>,
    pub kind: Option<String>,
    pub total_votes: Option<i64>,
    pub status: Option<String>,
    // TODO 157 proposal_action @Megha-Dev-19
    // pub proposal_action: Option<String>,
}

async fn fetch_dao_proposals(
    db: &DB,
    account_id: &str,
    limit: i64,
    order: &str,
    offset: i64,
    filters: Option<GetDaoProposalsFilters>,
) -> (Vec<SputnikProposalSnapshotRecord>, i64) {
    match db
        .get_dao_proposals(account_id, limit, order, offset, filters)
        .await
    {
        Err(e) => {
            eprintln!("Failed to get proposals: {:?}", e);
            (vec![], 0)
        }
        Ok(result) => result,
    }
}

#[utoipa::path(get, path = "/dao/proposals/search/<input>", params(
  ("input"= &str, Path, description ="Url encoded lowercase string to search for in proposal name, description, summary, and category fields."),
))]
#[get("/proposals/search/<input>")]
async fn search(
    input: String,
    db: &State<DB>,
) -> Option<Json<PaginatedResponse<SputnikProposalSnapshotRecord>>> {
    let result = if input.len() == 44 && input.chars().all(|c| c.is_ascii_hexdigit()) {
        match db.get_dao_proposal_by_hash(&input).await {
            Ok(proposal) => Ok((vec![proposal], 1)),
            Err(e) => Err(e),
        }
    } else {
        let search_input = format!("%{}%", input.to_lowercase());
        db.search_dao_proposals(&search_input).await
    };

    match result {
        Ok((proposals, total)) => Some(Json(PaginatedResponse::new(
            proposals.into_iter().collect(),
            1,
            10,
            total.try_into().unwrap(),
            None, // TODO add newly indexed
        ))),
        Err(e) => {
            eprintln!("Error fetching proposals: {:?}", e);
            None
        }
    }
}

#[utoipa::path(get, path = "/dao/proposals/<account_id>?<order>&<limit>&<offset>&<filters>", params(
  ("account_id"= &str, Path, description = "DAO account id"),
  ("order"= &str, Path, description ="default order id_desc"),
  ("limit"= i64, Path, description = "default limit 10"),
  ("offset"= i64, Path, description = "offset"),
  ("filters"= GetDaoProposalsFilters, Path, description = "filters struct that contains stuff like category, labels (vec), author_id, stage, block_timestamp (i64)"),
))]
#[get("/proposals/<account_id>?<order>&<limit>&<offset>&<filters>")]
async fn get_dao_proposals(
    account_id: &str,
    order: Option<&str>,
    limit: Option<i64>,
    offset: Option<i64>,
    filters: Option<GetDaoProposalsFilters>,
    db: &State<DB>,
    rpc_service: &State<RpcService>,
) -> Option<Json<PaginatedResponse<SputnikProposalSnapshotRecord>>> {
    let order = order.unwrap_or("id_desc");
    let limit = limit.unwrap_or(10);
    let offset = offset.unwrap_or(0);

    let contract = match AccountId::from_str(account_id) {
        Ok(contract) => contract,
        Err(_) => {
            eprintln!("Invalid account id: {}", account_id);
            return None;
        }
    };

    let last_updated_info = db
        .get_last_updated_info_for_contract(&contract)
        .await
        .unwrap();

    if let Err(e) = update_dao_nearblocks_data(
        db.inner(),
        &contract,
        rpc_service.inner(),
        Some(last_updated_info.after_block),
    )
    .await
    {
        eprintln!("Failed to update DAO via nearblocks: {e}");
        // https://testing-indexer.fly.dev/
        // return None;
    }

    let (proposals, total) =
        fetch_dao_proposals(db, account_id, limit, order, offset, filters).await;

    Some(Json(PaginatedResponse::new(
        proposals.into_iter().collect(),
        1,
        limit.try_into().unwrap(),
        total.try_into().unwrap(),
        None, // TODO add newly indexed
    )))
}

#[get("/proposals/<account_id>/block/<block>")]
async fn set_block(account_id: &str, block: i64, db: &State<DB>) -> Result<(), Status> {
    match db
        .set_last_updated_info_for_contract(&AccountId::from_str(account_id).unwrap(), 0, block)
        .await
    {
        Ok(()) => Ok(()),
        Err(e) => {
            eprintln!("Error updating block: {:?}", e);
            Err(Status::InternalServerError)
        }
    }
}

#[get("/admin/<account_id>/block")]
async fn get_block_info(account_id: &str, db: &State<DB>) -> Result<Json<LastUpdatedInfo>, Status> {
    let last_updated_info = db
        .get_last_updated_info_for_contract(&AccountId::from_str(account_id).unwrap())
        .await
        .unwrap();
    Ok(Json(last_updated_info))
}

#[get("/admin/<account_id>/reset")]
async fn reset_dao_proposals(account_id: &str, db: &State<DB>) -> Result<(), Status> {
    db.remove_all_dao_proposals(account_id).await.unwrap();
    Ok(())
}

#[get("/admin/<account_id>/resetandtest")]
async fn reset_and_test(
    account_id: &str,
    db: &State<DB>,
    rpc_service: &State<RpcService>,
) -> Json<PaginatedResponse<SputnikProposalSnapshotRecord>> {
    let contract = match AccountId::from_str(account_id) {
        Ok(contract) => contract,
        Err(_) => {
            eprintln!("Invalid account id: {}", account_id);
            AccountId::from_str("testing-astradao.sputnik-dao.near").unwrap()
        }
    };

    db.remove_all_dao_proposals(account_id).await.unwrap();

    let _ = update_dao_nearblocks_data(db.inner(), &contract, rpc_service.inner(), Some(0)).await;

    let (proposals, total) = fetch_dao_proposals(db, account_id, 10, "id_desc", 0, None).await;

    Json(PaginatedResponse::new(
        proposals.into_iter().collect(),
        1,
        10,
        total.try_into().unwrap(),
        None, // TODO add newly indexed
    ))
}

#[utoipa::path(get, path = "/proposals/sync_from_start/<account_id>")]
#[get("/proposals/sync_from_start/<account_id>")]
async fn sync_from_start(
    account_id: &str,
    db: &State<DB>,
    rpc_service: &State<RpcService>,
) -> Result<String, Status> {
    let contract = match AccountId::from_str(account_id) {
        Ok(contract) => contract,
        Err(_) => {
            eprintln!("Invalid account id: {}", account_id);
            return Err(Status::BadRequest);
        }
    };

    let result = update_dao_nearblocks_data(db, &contract, rpc_service, Some(0)).await;

    match result {
        Ok(_) => Ok("Success".to_string()),
        Err(e) => {
            eprintln!("Error syncing from start: {:?}", e);
            Err(Status::InternalServerError)
        }
    }
}

#[utoipa::path(get, path = "/dao/proposals/<account_id>/receivers")]
#[get("/proposals/<account_id>/receivers")]
async fn get_unique_receivers(
    account_id: &str,
    db: &State<DB>,
) -> Result<Json<Vec<String>>, Status> {
    let _ = match AccountId::from_str(account_id) {
        Ok(contract) => contract,
        Err(_) => {
            eprintln!("Invalid account id: {}", account_id);
            return Err(Status::BadRequest);
        }
    };

    match db.get_unique_receiver_ids(account_id).await {
        Ok(receiver_ids) => Ok(Json(receiver_ids)),
        Err(e) => {
            eprintln!("Error fetching unique receiver IDs: {:?}", e);
            Err(Status::InternalServerError)
        }
    }
}

pub fn stage() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("Rfp Stage", |rocket| async {
        println!("Rfp stage on ignite!");

        rocket.mount(
            "/dao/",
            rocket::routes![
                get_block_info,
                set_block,
                get_dao_proposals,
                reset_dao_proposals,
                reset_and_test,
                search,
                sync_from_start,
                get_unique_receivers,
            ],
        )
    })
}
