use crate::db::db_types::{ProposalSnapshotRecord, RfpSnapshotRecord};
use crate::db::DB;
use crate::entrypoints::proposal::proposal_types::FromContractProposal;
use crate::entrypoints::rfp::rfp_types::FromContractRFP;
use crate::rpc_service::{ChangeLog, ChangeLogType, RpcService};
use devhub_shared::proposal::VersionedProposal;
use devhub_shared::rfp::VersionedRFP;
pub async fn fetch_changelog_from_rpc(
    db: &DB,
    rpc_service: &RpcService,
    after_block: Option<i64>,
) -> anyhow::Result<usize> {
    let result = match rpc_service.get_change_log_since(after_block.unwrap()).await {
        Ok(change_log) => change_log,
        Err(e) => {
            eprintln!("Error fetching change log: {:?}", e);
            return Err(anyhow::anyhow!("Error fetching change log from contract"));
        }
    };

    for change in result.clone() {
        // Get the latest proposal
        match change.change_log_type {
            ChangeLogType::Proposal(proposal_id) => {
                handle_proposal_change(db, rpc_service, proposal_id, &change).await?;
            }
            ChangeLogType::RFP(rfp_id) => {
                handle_rfp_change(db, rpc_service, rfp_id, &change).await?;
            }
        }
    }
    Ok(result.len())
}

async fn handle_proposal_change(
    db: &DB,
    rpc_service: &RpcService,
    proposal_id: u32,
    change: &ChangeLog,
) -> anyhow::Result<()> {
    let versioned_proposal = match rpc_service.get_proposal(proposal_id as i32).await {
        Ok(proposal) => proposal.data,
        Err(e) => {
            eprintln!("Error fetching proposal: {:?}", e);
            return Err(anyhow::anyhow!("Error fetching proposal"));
        }
    };
    // Add proposal
    let mut tx = db.begin().await.map_err(|e| {
        eprintln!("Failed to begin transaction: {:?}", e);
        anyhow::anyhow!("Failed to begin transaction")
    })?;

    let author_id = match versioned_proposal.clone() {
        VersionedProposal::V0(proposal) => proposal.author_id,
    };

    DB::upsert_proposal(&mut tx, proposal_id, author_id.to_string())
        .await
        .map_err(|e| {
            eprintln!("Failed to upsert proposal {}: {:?}", proposal_id, e);
            anyhow::anyhow!("Failed to upsert proposal")
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
            anyhow::anyhow!("Failed to insert proposal snapshot")
        })?;
    DB::set_last_updated_block_on_tx(&mut tx, change.block_id as i64)
        .await
        .map_err(|e| {
            eprintln!("Failed to set last updated block on tx: {:?}", e);
            anyhow::anyhow!("Failed to set last updated block on tx")
        })?;

    tx.commit().await.map_err(|e| {
        eprintln!("Failed to commit transaction: {:?}", e);
        anyhow::anyhow!("Failed to commit transaction")
    })?;
    Ok(())
}

async fn handle_rfp_change(
    db: &DB,
    rpc_service: &RpcService,
    rfp_id: u32,
    change: &ChangeLog,
) -> anyhow::Result<()> {
    let versioned_rfp = match rpc_service.get_rfp(rfp_id as i32).await {
        Ok(rfp) => rfp.data,
        Err(e) => {
            eprintln!("Error fetching rfp: {:?}", e);
            return Err(anyhow::anyhow!("Error fetching rfp"));
        }
    };
    // Add rfp
    let mut tx = db.begin().await.map_err(|e| {
        eprintln!("Failed to begin transaction: {:?}", e);
        anyhow::anyhow!("Failed to begin transaction")
    })?;
    let author_id = match versioned_rfp.clone() {
        VersionedRFP::V0(rfp) => rfp.author_id,
    };

    DB::upsert_rfp(&mut tx, rfp_id, author_id.to_string())
        .await
        .map_err(|e| {
            eprintln!("Failed to upsert rfp {}: {:?}", rfp_id, e);
            anyhow::anyhow!("Failed to upsert rfp")
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
            anyhow::anyhow!("Failed to insert rfp snapshot")
        })?;

    DB::set_last_updated_block_on_tx(&mut tx, change.block_id as i64)
        .await
        .map_err(|e| {
            eprintln!("Failed to set last updated block on tx: {:?}", e);
            anyhow::anyhow!("Failed to set last updated block on tx")
        })?;

    tx.commit().await.map_err(|e| {
        eprintln!("Failed to commit transaction: {:?}", e);
        anyhow::anyhow!("Failed to commit transaction")
    })?;
    Ok(())
}
