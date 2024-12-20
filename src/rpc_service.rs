use std::{collections::HashMap, str::FromStr};

use devhub_shared::proposal::VersionedProposal;
use devhub_shared::rfp::VersionedRFP;
use near_account_id::AccountId;
use near_api::{prelude::*, types::reference::Reference, types::Data};
use near_jsonrpc_client::methods::query::RpcQueryRequest;
use near_sdk::json_types::U64;
use rocket::http::Status;
use rocket::serde::json::json;
use serde::Deserialize;

use crate::entrypoints::sputnik::sputnik_types::{
    Proposal, ProposalKind, ProposalOutput, ProposalStatus,
};

#[derive(Deserialize)]
pub struct RpcResponse {
    pub data: String,
}

#[derive(Clone)]
pub struct RpcService {
    network: NetworkConfig,
    contract: Contract,
}

impl Default for RpcService {
    fn default() -> Self {
        Self {
            network: NetworkConfig::mainnet(),
            contract: Contract("devhub.near".parse::<AccountId>().unwrap()),
        }
    }
}

impl RpcService {
    pub fn new(id: &AccountId) -> Self {
        Self {
            network: NetworkConfig::mainnet(),
            contract: Contract(id.clone()),
        }
    }

    pub async fn get_proposal(
        &self,
        proposal_id: i32,
    ) -> Result<Data<VersionedProposal>, near_api::errors::QueryError<RpcQueryRequest>> {
        let result: Result<Data<VersionedProposal>, _> = self
            .contract
            .call_function("get_proposal", json!({ "proposal_id": proposal_id }))
            .unwrap()
            .read_only()
            .fetch_from(&self.network)
            .await;

        result
    }

    pub async fn get_rfp(
        &self,
        rfp_id: i32,
    ) -> Result<Data<VersionedRFP>, near_api::errors::QueryError<RpcQueryRequest>> {
        let result: Result<Data<VersionedRFP>, _> = self
            .contract
            .call_function("get_rfp", json!({ "rfp_id": rfp_id }))
            .unwrap()
            .read_only()
            .fetch_from(&self.network)
            .await;

        result
    }

    pub async fn get_dao_proposal(
        &self,
        proposal_id: i64,
    ) -> Result<Data<ProposalOutput>, near_api::errors::QueryError<RpcQueryRequest>> {
        println!("get_dao_proposal: {:?}", proposal_id);
        let result: Result<Data<ProposalOutput>, _> = self
            .contract
            .call_function("get_proposal", json!({ "id": proposal_id }))
            .unwrap()
            .read_only()
            .fetch_from(&self.network)
            .await;

        result
    }

    pub async fn get_last_dao_proposal_id(
        &self,
    ) -> Result<Data<i64>, near_api::errors::QueryError<RpcQueryRequest>> {
        let result: Result<Data<i64>, _> = self
            .contract
            .call_function("get_last_proposal_id", json!({}))
            .unwrap()
            .read_only()
            .fetch_from(&self.network)
            .await;

        result
    }

    pub async fn get_dao_proposals(
        &self,
        from_index: i32,
        limit: i32,
    ) -> Result<Data<Vec<ProposalOutput>>, near_api::errors::QueryError<RpcQueryRequest>> {
        let result: Result<Data<Vec<ProposalOutput>>, _> = self
            .contract
            .call_function(
                "get_proposals",
                json!({ "from_index": from_index, "limit": limit }),
            )
            .unwrap()
            .read_only()
            .fetch_from(&self.network)
            .await;

        result
    }

    pub async fn get_all_proposal_ids(&self) -> Result<Vec<i32>, Status> {
        let result: Result<Data<Vec<i32>>, _> = self
            .contract
            .call_function("get_all_proposal_ids", ())
            .unwrap()
            .read_only()
            .fetch_from(&self.network)
            .await;

        match result {
            Ok(res) => Ok(res.data),
            Err(e) => {
                eprintln!("Failed to get all proposal ids: {:?}", e);
                Err(Status::InternalServerError)
            }
        }
    }

    pub async fn get_proposal_on_block(
        &self,
        proposal_id: i32,
        block_id: i64,
    ) -> Result<VersionedProposal, Status> {
        let result: Result<Data<VersionedProposal>, near_api::errors::QueryError<RpcQueryRequest>> =
            self.contract
                .call_function("get_proposal", json!({ "proposal_id": proposal_id }))
                .unwrap()
                .read_only()
                .at(Reference::AtBlock(block_id as u64))
                .fetch_from(&self.network)
                .await;

        match result {
            Ok(res) => Ok(res.data),
            Err(on_block_error) => match self.get_proposal(proposal_id).await {
                Ok(proposal) => Ok(proposal.data),
                Err(rpc_error) => {
                    eprintln!(
                        "Failed to get proposal from RPC on block height {} and id {}",
                        block_id, proposal_id,
                    );
                    eprintln!("{:?}", on_block_error);
                    eprintln!("{:?}", rpc_error);
                    Err(Status::InternalServerError)
                }
            },
        }
    }

    pub async fn get_rfp_on_block(
        &self,
        rfp_id: i32,
        block_id: i64,
    ) -> Result<VersionedRFP, Status> {
        let result: Result<Data<VersionedRFP>, near_api::errors::QueryError<RpcQueryRequest>> =
            self.contract
                .call_function("get_rfp", json!({ "rfp_id": rfp_id }))
                .unwrap()
                .read_only()
                .at(Reference::AtBlock(block_id as u64))
                .fetch_from(&self.network)
                .await;

        match result {
            Ok(res) => Ok(res.data),
            Err(e) => {
                eprintln!("Failed to get rfp on block: {:?}", e);
                Err(Status::InternalServerError)
            }
        }
    }

    pub async fn get_dao_proposal_on_block(
        &self,
        proposal_id: i64,
        block_id: i64,
    ) -> Result<ProposalOutput, Status> {
        let result: Result<Data<ProposalOutput>, near_api::errors::QueryError<RpcQueryRequest>> =
            self.contract
                .call_function("get_proposal", json!({ "id": proposal_id }))
                .unwrap()
                .read_only()
                .at(Reference::AtBlock(block_id as u64))
                .fetch_from(&self.network)
                .await;

        match result {
            Ok(res) => Ok(res.data),
            Err(on_block_error) => match self.get_dao_proposal(proposal_id).await {
                Ok(proposal) => Ok(proposal.data),
                Err(rpc_error) => {
                    eprintln!(
                        "Failed get_dao_proposal_on_block from RPC on block height {} and id {}",
                        block_id, proposal_id,
                    );
                    eprintln!("{:?}", on_block_error);
                    eprintln!("{:?}", rpc_error);
                    // Err(Status::InternalServerError)
                    // TODO issue 1018
                    Ok(ProposalOutput {
                        id: proposal_id.try_into().unwrap(),
                        proposal: Proposal {
                            proposer: AccountId::from_str("deleted.near").unwrap(),
                            description: "".to_string(),
                            kind: ProposalKind::FunctionCall {
                                receiver_id: AccountId::from_str("deleted.near").unwrap(),
                                actions: vec![],
                            },
                            status: ProposalStatus::Removed,
                            vote_counts: HashMap::new(),
                            votes: HashMap::new(),
                            submission_time: U64::from(0),
                        },
                    })
                }
            },
        }
    }

    pub async fn get_dao_proposals_on_block(
        &self,
        from_index: i32,
        limit: i32,
        block_id: i64,
    ) -> Result<Vec<ProposalOutput>, Status> {
        let result: Result<
            Data<Vec<ProposalOutput>>,
            near_api::errors::QueryError<RpcQueryRequest>,
        > = self
            .contract
            .call_function(
                "get_proposals",
                json!({ "from_index": from_index, "limit": limit }),
            )
            .unwrap()
            .read_only()
            .at(Reference::AtBlock(block_id as u64))
            .fetch_from(&self.network)
            .await;

        match result {
            Ok(res) => Ok(res.data),
            Err(on_block_error) => match self.get_dao_proposals(from_index, limit).await {
                Ok(proposals) => Ok(proposals.data),
                Err(rpc_error) => {
                    eprintln!("Failed to get dao proposals from RPC on block height {} from_index {} limit {}", block_id, from_index, limit);
                    eprintln!("{:?}", on_block_error);
                    eprintln!("{:?}", rpc_error);
                    Err(Status::InternalServerError)
                }
            },
        }
    }

    pub async fn get_last_dao_proposal_id_on_block(
        &self,
        block_id: i64,
    ) -> Result<Data<i64>, near_api::errors::QueryError<RpcQueryRequest>> {
        let result: Result<Data<i64>, _> = self
            .contract
            .call_function("get_last_proposal_id", json!({}))
            .unwrap()
            .read_only()
            .at(Reference::AtBlock(block_id as u64))
            .fetch_from(&self.network)
            .await;

        result
    }
}
