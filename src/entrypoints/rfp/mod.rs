use self::rfp_types::*;
use crate::changelog::fetch_changelog_from_rpc;
use crate::db::db_types::{RfpSnapshotRecord, RfpWithLatestSnapshotView};
use crate::db::DB;
use crate::rpc_service::RpcService;
use crate::separate_number_and_text;
use crate::types::PaginatedResponse;
use devhub_shared::rfp::VersionedRFP;
use rocket::serde::json::Json;
use rocket::{delete, get, http::Status, State};
use std::convert::TryInto;
pub mod rfp_types;

#[utoipa::path(get, path = "/rfps/search/<input>", params(
  ("input"= &str, Path, description ="The string to search for in rfp name, description, summary, and category fields."),
))]
#[get("/search/<input>")]
async fn search(
    input: &str,
    db: &State<DB>,
) -> Option<Json<PaginatedResponse<RfpWithLatestSnapshotView>>> {
    let limit = 10;
    let (number_opt, _) = separate_number_and_text(input);
    let result = if let Some(number) = number_opt {
        match db.get_rfp_with_latest_snapshot_by_id(number).await {
            Ok(rfp) => Ok((vec![rfp], 1)),
            Err(e) => Err(e),
        }
    } else {
        let search_input = format!("%{}%", input.to_lowercase());
        db.search_rfps_with_latest_snapshot(&search_input, limit, 0)
            .await
    };

    match result {
        Ok((rfps, total)) => Some(Json(PaginatedResponse::new(
            rfps.into_iter().collect(),
            1,
            limit.try_into().unwrap(),
            total.try_into().unwrap(),
            None,
        ))),
        Err(e) => {
            eprintln!("Error fetching rfps: {:?}", e);
            None
        }
    }
}

async fn fetch_rfps(
    db: &DB,
    limit: i64,
    order: &str,
    offset: i64,
    filters: Option<GetRfpFilters>,
) -> (Vec<RfpWithLatestSnapshotView>, i64) {
    match db
        .get_rfps_with_latest_snapshot(limit, order, offset, filters)
        .await
    {
        Err(e) => {
            eprintln!("Failed to get rfps: {:?}", e);
            (vec![], 0)
        }
        Ok(result) => result,
    }
}

#[utoipa::path(get, path = "/rfps?<order>&<limit>&<offset>&<filters>", params(
  ("order"= &str, Path, description ="default order id_desc"),
  ("limit"= i64, Path, description = "default limit 10"),
  ("offset"= i64, Path, description = "offset"),
  ("filters"= GetRfpFilters, Path, description = "filters struct that contains stuff like category, labels (vec), author_id, stage, block_timestamp (i64)"),
))]
#[get("/?<order>&<limit>&<offset>&<filters>")]
async fn get_rfps(
    order: Option<&str>,
    limit: Option<i64>,
    offset: Option<i64>,
    filters: Option<GetRfpFilters>,
    db: &State<DB>,
    rpc_service: &State<RpcService>,
) -> Option<Json<PaginatedResponse<RfpWithLatestSnapshotView>>> {
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

    let (rfps, total) = fetch_rfps(db, limit, order, offset, filters).await;

    Some(Json(PaginatedResponse::new(
        rfps.into_iter().collect(),
        1,
        limit.try_into().unwrap(),
        total.try_into().unwrap(),
        Some(change_log_count.unwrap_or(0)),
    )))
}

#[utoipa::path(get, path = "/rfp/{rfp_id}")]
#[get("/<rfp_id>")]
async fn get_rfp(rfp_id: i32) -> Result<Json<VersionedRFP>, Status> {
    match RpcService::new().get_rfp(rfp_id).await {
        Ok(rfp) => Ok(Json(rfp.data)),
        Err(e) => {
            eprintln!("In /rfp/rfp_id; Failed to get rfp from RPC: {:?}", e);
            Err(Status::InternalServerError)
        }
    }
}

#[utoipa::path(get, path = "/rfp/{rfp_id}/snapshots")]
#[get("/<rfp_id>/snapshots")]
async fn get_rfp_with_snapshots(
    rfp_id: i64,
    db: &State<DB>,
    rpc_service: &State<RpcService>,
) -> Option<Json<Vec<RfpSnapshotRecord>>> {
    let last_updated_info = db.get_last_updated_info().await.unwrap();

    let _ = fetch_changelog_from_rpc(
        db.inner(),
        rpc_service.inner(),
        Some(last_updated_info.after_block),
    )
    .await;

    match db.get_rfp_with_all_snapshots(rfp_id).await {
        Err(e) => {
            eprintln!("Failed to get rfps: {:?}", e);
            None
        }
        Ok(result) => Some(Json(result)),
    }
}

// TODO Remove this once we go in production or put it behind authentication or a flag
#[delete("/<rfp_id>/snapshots")]
async fn remove_rfp_snapshots_by_rfp_id(rfp_id: i32, db: &State<DB>) -> Result<(), Status> {
    match db.remove_rfp_snapshots_by_rfp_id(rfp_id).await {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("Failed to remove rfp snapshots: {:?}", e);
            Err(Status::InternalServerError)
        }
    }
}

pub fn stage() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("Rfp Stage", |rocket| async {
        println!("Rfp stage on ignite!");

        rocket
            .mount("/rfps/", rocket::routes![get_rfps, search])
            .mount(
                "/rfp/",
                rocket::routes![
                    get_rfp,
                    get_rfp_with_snapshots,
                    remove_rfp_snapshots_by_rfp_id
                ],
            )
    })
}
