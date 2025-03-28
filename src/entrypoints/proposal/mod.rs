use self::proposal_types::*;
use crate::changelog::fetch_changelog_from_rpc;
use crate::db::db_types::{
    LastUpdatedInfo, ProposalSnapshotRecord, ProposalWithLatestSnapshotView,
};
use crate::db::DB;
use crate::nearblocks_client::transactions::update_nearblocks_data;
use crate::rpc_service::RpcService;
use crate::separate_number_and_text;
use crate::types::PaginatedResponse;
use devhub_shared::proposal::VersionedProposal;
use rocket::delete;
use rocket::serde::json::Json;
use rocket::{get, http::Status, State};
use std::convert::TryInto;
pub mod proposal_types;

#[utoipa::path(get, path = "/proposals/search?<input>", params(
  ("input"= &str, Path, description ="The string to search for in proposal name, description, summary, and category fields."),
))]
#[get("/search/<input>")]
async fn search(
    input: &str,
    db: &State<DB>,
) -> Option<Json<PaginatedResponse<ProposalWithLatestSnapshotView>>> {
    let limit = 10;
    let (number, _) = separate_number_and_text(input);

    let result = if let Some(number) = number {
        match db.get_proposal_with_latest_snapshot_by_id(number).await {
            Ok(proposal) => Ok((vec![proposal], 1)),
            Err(e) => Err(e),
        }
    } else {
        let search_input = format!("%{}%", input.to_lowercase());
        db.search_proposals_with_latest_snapshot(&search_input, limit, 0)
            .await
    };

    match result {
        Ok((proposals, total)) => Some(Json(PaginatedResponse::new(
            proposals.clone().into_iter().collect(),
            1,
            limit.try_into().unwrap(),
            total.try_into().unwrap(),
            None,
        ))),
        Err(e) => {
            eprintln!("Error fetching proposals: {:?}", e);
            None
        }
    }
}

async fn fetch_proposals(
    db: &DB,
    limit: i64,
    order: &str,
    offset: i64,
    filters: Option<GetProposalFilters>,
) -> (Vec<ProposalWithLatestSnapshotView>, i64) {
    match db
        .get_proposals_with_latest_snapshot(limit, order, offset, filters)
        .await
    {
        Err(e) => {
            eprintln!("Failed to get proposals: {:?}", e);
            (vec![], 0)
        }
        Ok(result) => result,
    }
}

#[utoipa::path(get, path = "/proposals?<order>&<limit>&<offset>&<filters>", params(
  ("order"= &str, Path, description ="default order id_desc (ts_asc)"),
  ("limit"= i64, Path, description = "default limit 10"),
  ("offset"= i64, Path, description = "offset"),
  ("filters"= GetProposalFilters, Path, description = "filters struct that contains stuff like category, labels (vec), author_id, stage, block_timestamp (i64)"),
))]
#[get("/?<order>&<limit>&<offset>&<filters>")]
async fn get_proposals(
    order: Option<&str>,
    limit: Option<i64>,
    offset: Option<i64>,
    filters: Option<GetProposalFilters>,
    db: &State<DB>,
    rpc_service: &State<RpcService>,
) -> Option<Json<PaginatedResponse<ProposalWithLatestSnapshotView>>> {
    let order = order.unwrap_or("id_desc");
    let limit = limit.unwrap_or(10);
    let offset = offset.unwrap_or(0);

    let last_updated_info = db.get_last_updated_info().await.unwrap();

    let change_log_count = fetch_changelog_from_rpc(
        db.inner(),
        rpc_service.inner(),
        Some(last_updated_info.after_block),
    )
    .await;

    let (proposals, total) = fetch_proposals(db.inner(), limit, order, offset, filters).await;

    Some(Json(PaginatedResponse::new(
        proposals.into_iter().collect(),
        1,
        limit.try_into().unwrap(),
        total.try_into().unwrap(),
        Some(change_log_count.unwrap_or(0)),
    )))
}

#[utoipa::path(get, path = "/proposal/{proposal_id}/snapshots")]
#[get("/<proposal_id>/snapshots")]
async fn get_proposal_with_all_snapshots(
    proposal_id: i32,
    db: &State<DB>,
    rpc_service: &State<RpcService>,
) -> Option<Json<Vec<ProposalSnapshotRecord>>> {
    let last_updated_info = db.get_last_updated_info().await.unwrap();

    let _ = fetch_changelog_from_rpc(
        db.inner(),
        rpc_service.inner(),
        Some(last_updated_info.after_block),
    )
    .await;

    match db.get_proposal_with_all_snapshots(proposal_id).await {
        Err(e) => {
            eprintln!("Failed to get proposal snapshots: {:?}", e);
            // Ok(Json(vec![]))
            None
        }
        Ok(result) => Some(Json(result)),
    }
}

#[get("/info/cursor/<cursor>")]
async fn set_cursor(cursor: &str, db: &State<DB>) -> Result<(), Status> {
    match db.set_last_updated_cursor(cursor.to_string()).await {
        Ok(()) => Ok(()),
        Err(e) => {
            eprintln!("Error updating timestamp: {:?}", e);
            Err(Status::InternalServerError)
        }
    }
}

#[get("/info/timestamp/<timestamp>")]
async fn set_timestamp(timestamp: i64, db: &State<DB>) -> Result<(), Status> {
    match db.set_last_updated_timestamp(timestamp).await {
        Ok(()) => Ok(()),
        Err(e) => {
            eprintln!("Error updating timestamp: {:?}", e);
            Err(Status::InternalServerError)
        }
    }
}

#[get("/info/block/<block>")]
async fn set_block(block: i64, db: &State<DB>) -> Result<(), Status> {
    match db.set_last_updated_block(block).await {
        Ok(()) => Ok(()),
        Err(e) => {
            eprintln!("Error updating block: {:?}", e);
            Err(Status::InternalServerError)
        }
    }
}

#[get("/info/reset")]
async fn reset(db: &State<DB>) -> Result<(), Status> {
    match db.set_last_updated_info(0, 0, "".to_string()).await {
        Ok(()) => Ok(()),
        Err(e) => {
            eprintln!("Error updating timestamp: {:?}", e);
            Err(Status::InternalServerError)
        }
    }
}

#[utoipa::path(get, path = "/proposals/sync_from_start")]
#[get("/sync_from_start/<max_transactions>")]
async fn sync_from_start(
    max_transactions: Option<usize>,
    db: &State<DB>,
    rpc_service: &State<RpcService>,
) -> Result<String, Status> {
    let result = update_nearblocks_data(db, rpc_service, Some(0), max_transactions).await;

    match result {
        Ok(_) => Ok("Success".to_string()),
        Err(e) => {
            eprintln!("Error syncing from start: {:?}", e);
            Err(Status::InternalServerError)
        }
    }
}

#[utoipa::path(get, path = "/proposals/continue_sync")]
#[get("/continue_sync/<max_transactions>")]
async fn continue_sync(
    max_transactions: Option<usize>,
    db: &State<DB>,
    rpc_service: &State<RpcService>,
) -> Result<String, Status> {
    let last_updated_info = db.get_last_updated_info().await.unwrap();
    let result = update_nearblocks_data(
        db,
        rpc_service,
        Some(last_updated_info.after_block),
        max_transactions,
    )
    .await;

    match result {
        Ok(_) => Ok("Success".to_string()),
        Err(e) => {
            eprintln!("Error syncing from start: {:?}", e);
            Err(Status::InternalServerError)
        }
    }
}

// TODO Remove this once we go in production or put it behind authentication or a flag
#[get("/info/clean")]
async fn clean(db: &State<DB>) -> Result<(), Status> {
    let _ = match db.remove_all_snapshots().await {
        Ok(()) => Ok(()),
        Err(e) => {
            eprintln!("Error cleaning snapshots: {:?}", e);
            Err(Status::InternalServerError)
        }
    };

    match db.remove_all_data().await {
        Ok(()) => Ok(()),
        Err(e) => {
            eprintln!("Error cleaning data: {:?}", e);
            Err(Status::InternalServerError)
        }
    }
}

#[get("/info")]
async fn get_timestamp(db: &State<DB>) -> Result<Json<LastUpdatedInfo>, Status> {
    let info = db.get_last_updated_info().await.unwrap();
    Ok(Json(info))
}

#[utoipa::path(get, path = "/proposal/{proposal_id}")]
#[get("/<proposal_id>")]
async fn get_proposal(
    proposal_id: i32,
    rpc_service: &State<RpcService>,
) -> Result<Json<VersionedProposal>, rocket::http::Status> {
    // We should also add rate limiting to this endpoint
    match rpc_service.get_proposal(proposal_id).await {
        Ok(proposal) => Ok(Json(proposal.data)),
        Err(e) => {
            eprintln!("Failed to get proposal from RPC: {:?}", e);
            Err(Status::InternalServerError)
        }
    }
}

// TODO Remove this once we go in production or put it behind authentication or a flag
#[delete("/<proposal_id>/snapshots")]
async fn remove_proposal_snapshots_by_id(proposal_id: i32, db: &State<DB>) -> Result<(), Status> {
    match db.remove_proposal_snapshots_by_id(proposal_id).await {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("Failed to remove proposal snapshots: {:?}", e);
            Err(Status::InternalServerError)
        }
    }
}

pub fn stage() -> rocket::fairing::AdHoc {
    // rocket
    rocket::fairing::AdHoc::on_ignite("Proposal Stage", |rocket| async {
        println!("Proposal stage on ignite!");

        rocket
            .mount(
                "/proposals/",
                rocket::routes![
                    get_proposals,
                    set_timestamp,
                    get_timestamp,
                    search,
                    clean,
                    reset,
                    set_cursor,
                    set_block,
                    sync_from_start,
                    continue_sync,
                ],
            )
            .mount(
                "/proposal/",
                rocket::routes![
                    get_proposal,
                    get_proposal_with_all_snapshots,
                    remove_proposal_snapshots_by_id,
                ],
            )
    })
}
