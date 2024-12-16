use crate::db::db_types::ProposalSnapshotRecord;
use crate::db::DB;
use crate::entrypoints::proposal::proposal_types::{
    FromContractProposal, PartialEditProposalArgs, SetBlockHeightCallbackArgs,
};
use crate::nearblocks_client::types::{Transaction, BLOCK_HEIGHT_OFFSET};
use crate::rpc_service::RpcService;
use devhub_shared::proposal::VersionedProposal;
use near_account_id::AccountId;
use rocket::{http::Status, State};

pub async fn handle_set_block_height_callback(
    transaction: Transaction,
    db: &State<DB>,
    contract: &AccountId,
) -> Result<(), Status> {
    let action = transaction
        .actions
        .as_ref()
        .and_then(|actions| actions.first())
        .ok_or(Status::InternalServerError)?;

    let json_args = action.args.clone();

    let args: SetBlockHeightCallbackArgs =
        serde_json::from_str(&json_args.unwrap_or_default()).unwrap();

    let mut tx = db.begin().await.map_err(|e| {
        eprintln!("Failed to begin transaction: {:?}", e);
        Status::InternalServerError
    })?;
    DB::upsert_proposal(
        &mut tx,
        args.clone().proposal.id,
        args.clone().proposal.author_id.to_string(),
    )
    .await
    .map_err(|e| {
        eprintln!("Failed to upsert proposal {}: {:?}", args.proposal.id, e);
        Status::InternalServerError
    })?;

    let rpc_service = RpcService::new(contract);
    let id = args.clone().proposal.id.try_into().unwrap();

    let versioned_proposal_fallback: VersionedProposal = args.clone().proposal.into();
    let versioned_proposal = match rpc_service.get_proposal(id).await {
        Ok(proposal) => proposal.data,
        Err(e) => {
            eprintln!(
                "Failed to get proposal from RPC, using first snapshot as fallback {:?}",
                e
            );
            versioned_proposal_fallback
        }
    };

    let snapshot = ProposalSnapshotRecord::from_contract_proposal(
        versioned_proposal.clone().into(),
        transaction.block_timestamp.parse::<i64>().unwrap(),
        transaction.block.block_height,
    );

    DB::insert_proposal_snapshot(&mut tx, &snapshot)
        .await
        .map_err(|e| {
            eprintln!(
                "Failed to insert proposal snapshot for proposal {}: {:?}",
                id, e
            );
            Status::InternalServerError
        })?;

    tx.commit().await.map_err(|e| {
        eprintln!("Failed to commit transaction: {:?}", e);
        Status::InternalServerError
    })?;

    Ok(())
}

pub async fn handle_edit_proposal(
    transaction: Transaction,
    db: &State<DB>,
    contract: &AccountId,
) -> Result<(), rocket::http::Status> {
    let rpc_service = RpcService::new(contract);
    let id = get_proposal_id(&transaction).map_err(|e| {
        eprintln!("Failed to get proposal ID: {}", e);
        Status::InternalServerError
    })?;
    println!("Updating proposal {}", id);
    let versioned_proposal = match rpc_service
        .get_proposal_on_block(
            id,
            transaction.receipt_block.block_height + BLOCK_HEIGHT_OFFSET,
        )
        .await
    {
        Ok(proposal) => proposal,
        Err(e) => {
            eprintln!("Failed to get proposal from RPC: {:?}", e);
            return Err(Status::InternalServerError);
        }
    };

    let mut tx = db.begin().await.map_err(|e| {
        eprintln!("Failed to begin transaction: {:?}", e);
        Status::InternalServerError
    })?;

    let snapshot = ProposalSnapshotRecord::from_contract_proposal(
        versioned_proposal.clone().into(),
        transaction.block_timestamp.parse::<i64>().unwrap(),
        transaction.block.block_height,
    );

    DB::insert_proposal_snapshot(&mut tx, &snapshot)
        .await
        .map_err(|e| {
            eprintln!(
                "Failed to insert proposal snapshot for proposal {}: {:?}",
                id, e
            );
            Status::InternalServerError
        })?;

    tx.commit().await.map_err(|e| {
        eprintln!("Failed to commit transaction: {:?}", e);
        Status::InternalServerError
    })?;

    Ok(())
}

fn get_proposal_id(transaction: &Transaction) -> Result<i32, &'static str> {
    let action = transaction
        .actions
        .as_ref()
        .and_then(|actions| actions.first())
        .ok_or("No actions found in transaction")?;

    let args: PartialEditProposalArgs = serde_json::from_str(action.args.as_ref().unwrap())
        .map_err(|e| {
            eprintln!("Failed to parse JSON: {:?}", e);
            "Failed to parse proposal arguments"
        })?;

    Ok(args.id)
}
