use crate::db::db_types::{ProposalSnapshotRecord, RfpSnapshotRecord};
use crate::db::DB;
use crate::entrypoints::proposal::proposal_types::FromContractProposal;
use crate::entrypoints::rfp::rfp_types::FromContractRFP;
// TODO: Remove these
// use crate::nearblocks_client::proposal::{handle_edit_proposal, handle_set_block_height_callback};
// use crate::nearblocks_client::rfp::{handle_edit_rfp, handle_set_rfp_block_height_callback};
use crate::rpc_service::{ChangeLogType, RpcService};
use devhub_shared::proposal::VersionedProposal;
use devhub_shared::rfp::VersionedRFP;
use near_account_id::AccountId;
use rocket::http::Status;

async fn update_nearblocks_data(
    db: &DB,
    contract: &AccountId,
    after_block: Option<i64>,
) -> Result<(), Status> {
    let rpc_service = RpcService::new(contract);
    let result = match rpc_service.get_change_log_since(after_block.unwrap()).await {
        Ok(change_log) => change_log,
        Err(e) => {
            eprintln!("Error fetching change log: {:?}", e);
            return Err(e);
        }
    };

    for change in result {
        // Get the latest proposal
        match change.change_log_type {
            ChangeLogType::Proposal(proposal_id) => {
                let versioned_proposal = match rpc_service.get_proposal(proposal_id as i32).await {
                    Ok(proposal) => proposal.data,
                    Err(e) => {
                        eprintln!("Error fetching proposal: {:?}", e);
                        return Err(Status::InternalServerError);
                    }
                };
                // Add proposal
                let mut tx = db.begin().await.map_err(|e| {
                    eprintln!("Failed to begin transaction: {:?}", e);
                    Status::InternalServerError
                })?;

                let author_id = match versioned_proposal.clone() {
                    VersionedProposal::V0(proposal) => proposal.author_id,
                };

                DB::upsert_proposal(&mut tx, proposal_id, author_id.to_string())
                    .await
                    .map_err(|e| {
                        eprintln!("Failed to upsert proposal {}: {:?}", proposal_id, e);
                        Status::InternalServerError
                    })?;
                let snapshot = ProposalSnapshotRecord::from_contract_proposal(
                    versioned_proposal.into(),
                    change.block_timestamp as i64,
                    change.block_id as i64,
                );
                DB::insert_proposal_snapshot(&mut tx, &snapshot)
                    .await
                    .map_err(|e| {
                        eprintln!(
                            "Failed to insert proposal snapshot for proposal {}: {:?}",
                            proposal_id, e
                        );
                        Status::InternalServerError
                    })?;
                DB::set_last_updated_block_on_tx(&mut tx, change.block_id as i64)
                    .await
                    .map_err(|e| {
                        eprintln!("Failed to set last updated block on tx: {:?}", e);
                        Status::InternalServerError
                    })?;

                tx.commit().await.map_err(|e| {
                    eprintln!("Failed to commit transaction: {:?}", e);
                    Status::InternalServerError
                })?;
            }
            ChangeLogType::RFP(rfp_id) => {
                let versioned_rfp = match rpc_service.get_rfp(rfp_id as i32).await {
                    Ok(rfp) => rfp.data,
                    Err(e) => {
                        eprintln!("Error fetching rfp: {:?}", e);
                        return Err(Status::InternalServerError);
                    }
                };
                // Add rfp
                let mut tx = db.begin().await.map_err(|e| {
                    eprintln!("Failed to begin transaction: {:?}", e);
                    Status::InternalServerError
                })?;
                let author_id = match versioned_rfp.clone() {
                    VersionedRFP::V0(rfp) => rfp.author_id,
                };
                DB::upsert_rfp(&mut tx, rfp_id, author_id.to_string())
                    .await
                    .map_err(|e| {
                        eprintln!("Failed to upsert rfp {}: {:?}", rfp_id, e);
                        Status::InternalServerError
                    })?;
                let snapshot = RfpSnapshotRecord::from_contract_rfp(
                    versioned_rfp.into(),
                    change.block_timestamp as i64,
                    change.block_id as i64,
                );
                DB::insert_rfp_snapshot(&mut tx, &snapshot)
                    .await
                    .map_err(|e| {
                        eprintln!("Failed to insert rfp snapshot for rfp {}: {:?}", rfp_id, e);
                        Status::InternalServerError
                    })?;

                DB::set_last_updated_block_on_tx(&mut tx, change.block_id as i64)
                    .await
                    .map_err(|e| {
                        eprintln!("Failed to set last updated block on tx: {:?}", e);
                        Status::InternalServerError
                    })?;

                tx.commit().await.map_err(|e| {
                    eprintln!("Failed to commit transaction: {:?}", e);
                    Status::InternalServerError
                })?;
            }
        }
    }
    Ok(())
}
