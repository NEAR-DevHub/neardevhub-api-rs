use crate::db::db_types::{ProposalSnapshotRecord, RfpSnapshotRecord};
use crate::db::DB;
use crate::entrypoints::proposal::proposal_types::FromContractProposal;
use crate::entrypoints::rfp::rfp_types::FromContractRFP;
use crate::rpc_service::RpcService;
use near_account_id::AccountId;
use rocket::{
    fairing::{self, AdHoc, Fairing},
    Build, Rocket,
};
use rocket_db_pools::Database;
use std::time::Duration;
use tokio::time::interval;

pub struct PeriodicUpdater {
    proposal_sync_interval: u64,
    rfp_sync_interval: u64,
}

impl PeriodicUpdater {
    // Helper function to sync proposals
    pub async fn sync_proposals(db: &DB, contract: &AccountId) -> Result<Vec<i32>, String> {
        let rpc_service = RpcService::new(contract);
        let proposal_ids = match rpc_service.get_all_proposal_ids().await {
            Ok(ids) => ids,
            Err(e) => {
                eprintln!("Failed to get proposal ids: {:?}", e);
                return Err("Failed to get proposal ids".to_string());
            }
        };

        let last_ten_proposal_ids = proposal_ids
            .iter()
            .rev()
            .take(20)
            .copied()
            .collect::<Vec<_>>();

        let mut tx = db.begin().await.map_err(|e| {
            eprintln!("Failed to begin transaction: {:?}", e);
            e.to_string()
        })?;

        for proposal_id in last_ten_proposal_ids.clone() {
            println!("syncing proposal_id: {}", proposal_id);
            let proposal = rpc_service.get_proposal(proposal_id).await.unwrap();
            let block_timestamp = rpc_service
                .block_timestamp(proposal.block_height)
                .await
                .unwrap();

            let snapshot = ProposalSnapshotRecord::from_contract_proposal(
                proposal.data.into(),
                block_timestamp as i64,
                proposal.block_height as i64,
            );

            DB::upsert_proposal(
                &mut tx,
                snapshot.proposal_id as u32,
                snapshot.editor_id.to_string(),
            )
            .await
            .map_err(|e| {
                eprintln!(
                    "Failed to upsert proposal {}: {:?}",
                    snapshot.proposal_id, e
                );
                e.to_string()
            })?;
            let _ = DB::insert_proposal_snapshot(&mut tx, &snapshot).await;
        }

        tx.commit().await.map_err(|e| {
            eprintln!("Failed to commit transaction: {:?}", e);
            e.to_string()
        })?;

        Ok(last_ten_proposal_ids)
    }

    // Helper function to sync RFPs
    pub async fn sync_rfps(db: &DB, contract: &AccountId) -> Result<Vec<i32>, String> {
        let rpc_service = RpcService::new(contract);
        let rfp_ids = match rpc_service.get_all_rfp_ids().await {
            Ok(ids) => ids,
            Err(e) => {
                eprintln!("Failed to get rfp ids: {:?}", e);
                return Err("Failed to get rfp ids".to_string());
            }
        };

        let last_ten_rfp_ids = rfp_ids.iter().rev().take(10).copied().collect::<Vec<_>>();

        let mut tx = db.begin().await.map_err(|e| {
            eprintln!("Failed to begin transaction: {:?}", e);
            e.to_string()
        })?;

        for rfp_id in last_ten_rfp_ids.clone() {
            println!("syncing rfp_id: {}", rfp_id);
            let rfp = rpc_service.get_rfp(rfp_id).await.unwrap();
            let block_timestamp = rpc_service.block_timestamp(rfp.block_height).await.unwrap();

            let snapshot = RfpSnapshotRecord::from_contract_rfp(
                rfp.data.into(),
                block_timestamp as i64,
                rfp.block_height as i64,
            );

            DB::upsert_rfp(
                &mut tx,
                snapshot.rfp_id as u32,
                snapshot.editor_id.to_string(),
            )
            .await
            .map_err(|e| {
                eprintln!("Failed to upsert rfp {}: {:?}", snapshot.rfp_id, e);
                e.to_string()
            })?;
            let _ = DB::insert_rfp_snapshot(&mut tx, &snapshot).await;
        }

        tx.commit().await.map_err(|e| {
            eprintln!("Failed to commit transaction: {:?}", e);
            e.to_string()
        })?;

        Ok(last_ten_rfp_ids)
    }
}

#[rocket::async_trait]
impl Fairing for PeriodicUpdater {
    fn info(&self) -> rocket::fairing::Info {
        rocket::fairing::Info {
            name: "Periodic Updater",
            kind: rocket::fairing::Kind::Ignite,
        }
    }

    async fn on_ignite(&self, rocket: Rocket<Build>) -> fairing::Result {
        let db = match DB::fetch(&rocket) {
            Some(db) => db,
            None => return Err(rocket),
        };

        let contract = match rocket.state::<AccountId>() {
            Some(contract) => contract.clone(),
            None => return Err(rocket),
        };

        // Only spawn if proposal_sync_interval is greater than 0
        if self.proposal_sync_interval > 0 {
            let interval_duration = self.proposal_sync_interval;
            let db_clone = db.clone();
            let contract_clone = contract.clone();

            tokio::spawn(async move {
                let mut interval = interval(Duration::from_secs(interval_duration));
                loop {
                    interval.tick().await;
                    println!("Running periodic update for Proposals...");
                    if let Err(e) = Self::sync_proposals(&db_clone, &contract_clone).await {
                        eprintln!("Error during proposal sync: {}", e);
                    }
                }
            });
        }

        // Only spawn if rfp_sync_interval is greater than 0
        if self.rfp_sync_interval > 0 {
            let interval_duration = self.rfp_sync_interval;
            let db_clone = db.clone();
            let contract_clone = contract;

            tokio::spawn(async move {
                let mut interval = interval(Duration::from_secs(interval_duration));
                loop {
                    interval.tick().await;
                    println!("Running periodic update for RFPs...");
                    if let Err(e) = Self::sync_rfps(&db_clone, &contract_clone).await {
                        eprintln!("Error during RFP sync: {}", e);
                    }
                }
            });
        }

        Ok(rocket)
    }
}

pub fn stage(proposal_sync_interval: u64, rfp_sync_interval: u64) -> AdHoc {
    AdHoc::on_ignite("Periodic Updater Stage", move |rocket| async move {
        rocket.attach(PeriodicUpdater {
            proposal_sync_interval,
            rfp_sync_interval,
        })
    })
}
