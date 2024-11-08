use self::rfp_types::*;
use crate::db::db_types::RfpWithLatestSnapshotView;
use crate::db::DB;
use crate::rpc_service::RpcService;
use crate::types::{Contract, PaginatedResponse};
use crate::{nearblocks_client, timestamp_to_date_string};
use devhub_shared::rfp::VersionedRFP;
use near_account_id::AccountId;
use rocket::serde::json::Json;
use rocket::{get, http::Status, State};
use std::convert::TryInto;
pub mod rfp_types;

// #[utoipa::path(get, path = "/rfps/search?<input>", params(
//   ("input"= &str, Path, description ="The string to search for in rfp name, description, summary, and category fields."),
// ))]
// #[get("/search/<input>")]
// fn search(input: String) -> Option<Json<PaginatedResponse<RfpWithLatestSnapshotView>>> {
//     None
// }

#[utoipa::path(get, path = "/rfps?<order>&<limit>&<offset>&<filters>", params(
  ("order"= &str, Path, description ="order"),
  ("limit"= i64, Path, description = "limit"),
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
    contract: &State<Contract>,
) -> Option<Json<PaginatedResponse<RfpWithLatestSnapshotView>>> {
    let current_timestamp_nano = chrono::Utc::now().timestamp_nanos_opt().unwrap();
    let last_updated_timestamp = db.get_last_updated_timestamp().await.unwrap();

    let order = order.unwrap_or("desc");
    let limit = limit.unwrap_or(25);
    let offset = offset.unwrap_or(0);

    // If we called nearblocks in the last 60 milliseconds return the database values
    if current_timestamp_nano - last_updated_timestamp
        < chrono::Duration::seconds(60).num_nanoseconds().unwrap()
    {
        println!("Returning cached proposals");
        let (rfps, total) = match db
            .get_rfps_with_latest_snapshot(limit, order, offset, filters)
            .await
        {
            Err(e) => {
                println!("Failed to get proposals: {:?}", e);
                (vec![], 0)
            }
            Ok(result) => result,
        };

        return Some(Json(PaginatedResponse::new(
            rfps.into_iter().map(Into::into).collect(),
            1,
            limit.try_into().unwrap(),
            total.try_into().unwrap(),
        )));
    }
    println!("Fetching not yet indexed method calls from nearblocks");

    let nearblocks_client = nearblocks_client::ApiClient::default();

    let nearblocks_unwrapped = match nearblocks_client
        .get_account_txns_by_pagination(
            contract.parse::<AccountId>().unwrap(),
            // Instead of just set_block_height_callback we should get all method calls
            // and handle them accordingly.
            // TODO Change to None
            Some("set_rfp_block_height_callback".to_string()),
            Some(timestamp_to_date_string(last_updated_timestamp)),
            // if this limit hits 10 we might need to do it in a loop let's say there are 100 changes since the last call to nearblocks.
            Some(25),
            Some("asc".to_string()),
        )
        .await
    {
        Ok(nearblocks_unwrapped) => {
            // If the response was successful, print the count of method calls
            // println!("Response: {:?}", nearblocks_unwrapped);
            nearblocks_unwrapped
        }
        Err(e) => {
            // If there was an error, print it or handle it as needed
            eprintln!("Failed to fetch data from nearblocks: {:?}", e);
            nearblocks_client::ApiResponse { txns: vec![] }
        }
    };
    let _ = nearblocks_client::transactions::process(&nearblocks_unwrapped.txns, db).await;

    let (rfps, total) = match db
        .get_rfps_with_latest_snapshot(limit, order, offset, filters)
        .await
    {
        Err(e) => {
            println!("Failed to get proposals: {:?}", e);
            (vec![], 0)
        }
        Ok(result) => result,
    };

    Some(Json(PaginatedResponse::new(
        rfps.into_iter().map(Into::into).collect(),
        1,
        limit.try_into().unwrap(),
        total.try_into().unwrap(),
    )))
}

#[utoipa::path(get, path = "/rfps/{rfp_id}")]
#[get("/<rfp_id>")]
async fn get_rfp(rfp_id: i32) -> Result<Json<VersionedRFP>, Status> {
    // TODO Get cached rfp
    match RpcService::default().get_rfp(rfp_id).await {
        Ok(rfp) => Ok(Json(rfp.data)),
        Err(e) => {
            eprintln!("Failed to get rfp from RPC: {:?}", e);
            Err(Status::InternalServerError)
        }
    }
}

pub fn stage() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("Rfp Stage", |rocket| async {
        println!("Rfp stage on ignite!");

        rocket.mount("/rfps/", rocket::routes![get_rfps, get_rfp])
    })
}
