use crate::db::db_types::SputnikProposalSnapshotRecord;
use crate::db::DB;
use crate::nearblocks_client::transactions::update_dao_via_nearblocks;
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
    nearblocks_api_key: &State<String>,
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

    update_dao_via_nearblocks(
        db.inner(),
        &contract,
        nearblocks_api_key.inner(),
        Some(last_updated_info.after_block),
    )
    .await;

    let (proposals, total) =
        fetch_dao_proposals(db, account_id, limit, order, offset, filters).await;

    Some(Json(PaginatedResponse::new(
        proposals.into_iter().map(Into::into).collect(),
        1,
        limit.try_into().unwrap(),
        total.try_into().unwrap(),
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

#[get("/proposals/<account_id>/reset")]
async fn reset_dao_proposals(account_id: &str, db: &State<DB>) -> Result<(), Status> {
    db.remove_all_dao_proposals(account_id).await.unwrap();
    Ok(())
}

#[get("/proposals/<account_id>/resetandtest")]
async fn reset_and_test(
    account_id: &str,
    db: &State<DB>,
    nearblocks_api_key: &State<String>,
) -> Json<PaginatedResponse<SputnikProposalSnapshotRecord>> {
    let contract = match AccountId::from_str(account_id) {
        Ok(contract) => contract,
        Err(_) => {
            eprintln!("Invalid account id: {}", account_id);
            AccountId::from_str("testing-astradao.sputnik-dao.near").unwrap()
        }
    };

    db.remove_all_dao_proposals(account_id).await.unwrap();

    update_dao_via_nearblocks(db.inner(), &contract, nearblocks_api_key.inner(), Some(0)).await;

    let (proposals, total) = fetch_dao_proposals(db, account_id, 10, "id_desc", 0, None).await;

    Json(PaginatedResponse::new(
        proposals.into_iter().map(Into::into).collect(),
        1,
        10,
        total.try_into().unwrap(),
    ))
}

pub fn stage() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("Rfp Stage", |rocket| async {
        println!("Rfp stage on ignite!");

        rocket.mount(
            "/dao/",
            rocket::routes![
                set_block,
                get_dao_proposals,
                reset_dao_proposals,
                reset_and_test
            ],
        )
    })
}
