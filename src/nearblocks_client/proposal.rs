use crate::db::db_types::ProposalSnapshotRecord;
use crate::db::DB;
use crate::entrypoints::proposal::proposal_types::{
    FromContractProposal, PartialEditProposalArgs, SetBlockHeightCallbackArgs,
};
use crate::nearblocks_client::types::{Transaction, BLOCK_HEIGHT_OFFSET};
use crate::rpc_service::RpcService;
use devhub_shared::proposal::VersionedProposal;
use rocket::State;

pub async fn handle_set_block_height_callback(
    transaction: Transaction,
    db: &State<DB>,
    rpc_service: &State<RpcService>,
) -> anyhow::Result<()> {
    let action = transaction
        .actions
        .as_ref()
        .and_then(|actions| actions.first())
        .ok_or(anyhow::anyhow!("No actions found in transaction"))?;

    let json_args = action.args.clone();

    let args: SetBlockHeightCallbackArgs =
        serde_json::from_str(&json_args.unwrap_or_default()).unwrap();

    let mut tx = db.begin().await.map_err(|e| {
        eprintln!("Failed to begin transaction: {:?}", e);
        anyhow::anyhow!("Failed to begin transaction")
    })?;
    DB::upsert_proposal(
        &mut tx,
        args.clone().proposal.id,
        args.clone().proposal.author_id.to_string(),
    )
    .await
    .map_err(|e| {
        eprintln!("Failed to upsert proposal {}: {:?}", args.proposal.id, e);
        anyhow::anyhow!("Failed to upsert proposal")
    })?;

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
            anyhow::anyhow!("Failed to insert proposal snapshot")
        })?;

    DB::set_last_updated_block_on_tx(&mut tx, transaction.block.block_height)
        .await
        .map_err(|e| {
            eprintln!("Failed to set last updated block: {:?}", e);
            anyhow::anyhow!("Failed to set last updated block")
        })?;

    tx.commit().await.map_err(|e| {
        eprintln!("Failed to commit transaction: {:?}", e);
        anyhow::anyhow!("Failed to commit transaction")
    })?;

    Ok(())
}

pub async fn handle_edit_proposal(
    transaction: Transaction,
    db: &State<DB>,
    rpc_service: &State<RpcService>,
) -> anyhow::Result<()> {
    let id = get_proposal_id(&transaction).map_err(|e| {
        eprintln!("Failed to get proposal ID: {}", e);
        anyhow::anyhow!("Failed to get proposal ID")
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
            return Err(anyhow::anyhow!("Failed to get proposal from RPC"));
        }
    };

    let mut tx = db.begin().await.map_err(|e| {
        eprintln!("Failed to begin transaction: {:?}", e);
        anyhow::anyhow!("Failed to begin transaction")
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
            anyhow::anyhow!("Failed to insert proposal snapshot")
        })?;

    DB::set_last_updated_block_on_tx(&mut tx, transaction.block.block_height)
        .await
        .map_err(|e| {
            eprintln!("Failed to set last updated block: {:?}", e);
            anyhow::anyhow!("Failed to set last updated block")
        })?;

    tx.commit().await.map_err(|e| {
        eprintln!("Failed to commit transaction: {:?}", e);
        anyhow::anyhow!("Failed to commit transaction")
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
