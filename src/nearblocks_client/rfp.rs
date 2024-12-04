use crate::db::db_types::RfpSnapshotRecord;
use crate::db::DB;
use crate::entrypoints::rfp::rfp_types::*;
use crate::nearblocks_client::types::Transaction;
use crate::rpc_service::RpcService;
use devhub_shared::rfp::VersionedRFP;
use near_account_id::AccountId;
use rocket::{http::Status, State};

pub async fn handle_set_rfp_block_height_callback(
    transaction: Transaction,
    db: &State<DB>,
    contract: &AccountId,
) -> Result<(), Status> {
    if !transaction.receipt_outcome.status {
        eprintln!(
            "RFP receipt outcome status is {:?}",
            transaction.receipt_outcome.status
        );
        eprintln!("On transaction: {:?}", transaction);
        return Ok(());
    }

    let action = transaction
        .actions
        .as_ref()
        .and_then(|actions| actions.first())
        .ok_or(Status::InternalServerError)?;
    let json_args = action.args.clone().unwrap_or_default();

    let args: SetRfpBlockHeightCallbackArgs = serde_json::from_str(&json_args).unwrap();

    let mut tx = db.begin().await.map_err(|_e| Status::InternalServerError)?;
    DB::upsert_rfp(
        &mut tx,
        args.clone().rfp.id,
        args.clone().rfp.author_id.to_string(),
    )
    .await
    .unwrap();

    let rpc_service = RpcService::new(contract);
    let id = args.clone().rfp.id.try_into().unwrap();

    let versioned_rfp_fallback: VersionedRFP = args.clone().rfp.into();
    let versioned_rfp = match rpc_service.get_rfp(id).await {
        Ok(rfp) => rfp.data,
        Err(e) => {
            eprintln!(
                "Failed to get RFP from RPC, using first snapshot as fallback {:?}",
                e
            );
            versioned_rfp_fallback
        }
    };

    let snapshot = RfpSnapshotRecord::from_contract_rfp(
        versioned_rfp.into(),
        transaction.receipt_block.block_timestamp,
        transaction.receipt_block.block_height,
    );

    DB::insert_rfp_snapshot(&mut tx, &snapshot).await.unwrap();

    tx.commit()
        .await
        .map_err(|_e| Status::InternalServerError)?;

    Ok(())
}

fn get_rfp_id(transaction: &Transaction) -> Result<i32, &'static str> {
    let action = transaction
        .actions
        .as_ref()
        .and_then(|actions| actions.first())
        .ok_or("No actions found in transaction")?;

    let args: PartialEditRFPArgs =
        serde_json::from_str(action.args.as_ref().unwrap()).map_err(|e| {
            eprintln!("Failed to parse JSON: {:?}", e);
            "Failed to parse proposal arguments"
        })?;

    Ok(args.id)
}

pub async fn handle_edit_rfp(
    transaction: Transaction,
    db: &State<DB>,
    contract: &AccountId,
) -> Result<(), Status> {
    let rpc_service = RpcService::new(contract);
    let id = get_rfp_id(&transaction).map_err(|e| {
        eprintln!("Failed to get RFP ID: {}", e);
        Status::InternalServerError
    })?;
    println!("Updating rfp {}", id);
    let versioned_rfp = match rpc_service
        .get_rfp_on_block(id, transaction.receipt_block.block_height)
        .await
    {
        Ok(rfp) => rfp,
        Err(e) => {
            eprintln!("Failed to get rfp from RPC: {:?}", e);
            return Err(Status::InternalServerError);
        }
    };

    let mut tx = db.begin().await.map_err(|_e| Status::InternalServerError)?;

    let contract_rfp: ContractRFP = versioned_rfp.clone().into();
    println!(
        "RFP {} timestamp {}",
        contract_rfp.id, transaction.receipt_block.block_timestamp
    );

    let snapshot = RfpSnapshotRecord::from_contract_rfp(
        versioned_rfp.into(),
        transaction.receipt_block.block_timestamp,
        transaction.receipt_block.block_height,
    );

    DB::insert_rfp_snapshot(&mut tx, &snapshot)
        .await
        .map_err(|_e| Status::InternalServerError)?;

    tx.commit()
        .await
        .map_err(|_e| Status::InternalServerError)?;

    Ok(())
}
