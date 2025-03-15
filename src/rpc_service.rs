use crate::entrypoints::sputnik::sputnik_types::ProposalOutput;
use devhub_shared::proposal::VersionedProposal;
use devhub_shared::rfp::VersionedRFP;
use near_account_id::AccountId;
use near_api::{types::reference::Reference, types::Data};
use near_api::{Contract, NetworkConfig, RPCEndpoint};
use near_jsonrpc_client::methods::query::RpcQueryRequest;
use rocket::http::Status;
use rocket::serde::json::json;
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
pub struct RpcResponse {
    pub data: String,
}

#[derive(Clone)]
pub struct RpcService {
    network: NetworkConfig,
    contract: Contract,
}

#[derive(Debug, serde::Deserialize)]
pub struct Env {
    fastnear_api_key: String,
}

impl Default for RpcService {
    fn default() -> Self {
        Self {
            network: NetworkConfig::mainnet(),
            contract: Contract("devhub.near".parse::<AccountId>().unwrap()),
        }
    }
}

// Epoch 43,200
// 5 Epochs = 216,000

impl RpcService {
    pub fn new(id: &AccountId) -> Self {
        dotenvy::dotenv().ok();

        let env: Env = envy::from_env::<Env>().expect("Failed to load environment variables");

        let mut network = NetworkConfig::mainnet();

        let custom_endpoint =
            RPCEndpoint::new("https://rpc.mainnet.fastnear.com/".parse().unwrap())
                .with_api_key(env.fastnear_api_key.parse().unwrap())
                .with_retries(3)
                .with_exponential_backoff(true, 2);

        // Use fastnear first before the archival RPC with super low rate limit
        network.rpc_endpoints = vec![custom_endpoint, RPCEndpoint::mainnet().with_retries(3)];

        Self {
            network,
            contract: Contract(id.clone()),
        }
    }

    // devhub contract
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

    // devhub contract
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

    // sputnik contract
    pub async fn get_dao_proposal(
        &self,
        proposal_id: i64,
    ) -> Result<Data<ProposalOutput>, near_api::errors::QueryError<RpcQueryRequest>> {
        println!("get_dao_proposal: {:?}", proposal_id);
        let result: Result<Data<ProposalOutput>, _> = match self
            .contract
            .call_function("get_proposal", json!({ "id": proposal_id }))
            .unwrap()
            .read_only()
            .fetch_from(&self.network)
            .await
        {
            Ok(res) => Ok(res),
            Err(e) => {
                eprintln!("Failed to get dao proposal: {:?}", e);
                Err(e)
            }
        };

        result
    }

    // sputnik contract
    pub async fn get_last_proposal_id(
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

    // devhub contract
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

    // devhub contract
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

    // devhub contract
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

    // sputnik contract
    pub async fn get_dao_proposal_on_block(
        &self,
        proposal_id: i64,
        block_id: i64,
    ) -> anyhow::Result<ProposalOutput> {
        let result: Result<Data<Value>, near_api::errors::QueryError<RpcQueryRequest>> = self
            .contract
            .call_function("get_proposal", json!({ "id": proposal_id }))
            .unwrap()
            .read_only()
            .at(Reference::AtBlock(block_id as u64))
            .fetch_from(&self.network)
            .await;

        match result {
            Ok(raw_json) => match serde_json::from_value::<ProposalOutput>(raw_json.data) {
                Ok(output) => Ok(output),
                Err(e) => {
                    eprintln!("Deserialization error: {:?}", e);
                    Err(anyhow::anyhow!("Deserialization error: {:?}", e))
                }
            },
            Err(on_block_error) => {
                eprintln!(
                    "get_dao_proposal_on_block with id: {:?} on block: {:?}",
                    proposal_id, block_id
                );

                eprintln!(
                    "Failed to get dao proposal on block Error: {:?}",
                    on_block_error
                );
                Err(anyhow::anyhow!(
                    "Failed to get dao proposal on block: {:?}",
                    on_block_error
                ))
            }
        }
    }

    // sputnik contract
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

    // sputnik contract
    pub async fn get_last_proposal_id_on_block(
        &self,
        block_id: i64,
    ) -> Result<Data<i64>, near_api::errors::QueryError<RpcQueryRequest>> {
        println!("Attempting to get last proposal ID at block {}", block_id);
        println!("Using contract: {}", self.contract.0);
        println!("RPC endpoints configured: {:?}", self.network.rpc_endpoints);

        let result: Result<Data<i64>, _> = self
            .contract
            .call_function("get_last_proposal_id", json!({}))
            .unwrap()
            .read_only()
            .at(Reference::AtBlock(block_id as u64))
            .fetch_from(&self.network)
            .await;

        match &result {
            Ok(data) => {
                println!("Successfully retrieved last proposal ID: {}", data.data);
                result
            }
            Err(e) => {
                eprintln!(
                    "Failed to get last proposal ID at block {}: {:?}",
                    block_id, e
                );
                eprintln!("Error details: {:#?}", e);
                result
            }
        }
    }
}
