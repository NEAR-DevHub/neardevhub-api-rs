use crate::entrypoints::sputnik::sputnik_types::ProposalOutput;
use devhub_shared::proposal::ProposalId;
use devhub_shared::proposal::VersionedProposal;
use devhub_shared::rfp::RFPId;
use devhub_shared::rfp::VersionedRFP;
use near_account_id::AccountId;
use near_api::{types::reference::Reference, types::Data};
use near_api::{Contract, NetworkConfig, RPCEndpoint};
use near_jsonrpc_client::methods::query::RpcQueryRequest;
use rocket::http::Status;
use rocket::serde::json::json;
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct ChangeLog {
    pub block_id: u64,
    pub block_timestamp: u64,
    pub change_log_type: ChangeLogType,
}

#[derive(Deserialize, Clone)]
pub enum ChangeLogType {
    Proposal(ProposalId),
    RFP(RFPId),
}

#[derive(Deserialize)]
pub struct RpcResponse {
    pub data: String,
}

#[derive(Clone)]
pub struct RpcService {
    pub network: NetworkConfig,
    pub contract: Contract,
}

#[derive(Debug, serde::Deserialize)]
pub struct Env {
    pub contract: String,
    pub nearblocks_api_key: String,
    pub fastnear_api_key: String,
    pub database_url: String,
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
    pub fn new() -> Self {
        dotenvy::dotenv().ok();

        let env: Env = envy::from_env::<Env>().expect("Failed to load environment variables");
        Self::mainnet(env.contract.parse::<AccountId>().unwrap())
    }

    pub fn mainnet(contract: AccountId) -> Self {
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
            contract: Contract(contract),
        }
    }

    pub fn sandbox(network: NetworkConfig, contract: AccountId) -> Self {
        Self {
            network,
            contract: Contract(contract),
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

    // devhub contract
    pub async fn get_change_log(&self) -> Result<Vec<ChangeLog>, Status> {
        let result: Result<Data<Vec<ChangeLog>>, _> = self
            .contract
            .call_function("get_change_log", json!({}))
            .unwrap()
            .read_only()
            .fetch_from(&self.network)
            .await;

        match result {
            Ok(res) => Ok(res.data),
            Err(e) => {
                eprintln!("Failed to get change log: {:?}", e);
                Err(Status::InternalServerError)
            }
        }
    }

    // devhub contract
    pub async fn get_change_log_since(&self, block_id: i64) -> anyhow::Result<Vec<ChangeLog>> {
        match self
            .contract
            .call_function("get_change_log_since", json!({"since": block_id}))
            .unwrap()
            .read_only()
            .fetch_from(&self.network)
            .await
        {
            Ok(res) => Ok(res.data),
            Err(e) => {
                eprintln!(
                    "Failed to get change log since: {:?} error: {:?}",
                    block_id, e
                );
                Err(anyhow::anyhow!("Failed to get change log since: {:?}", e))
            }
        }
    }

    async fn query_contract<T, P>(
        &self,
        contract: Contract,
        method: &str,
        params: P,
        block_id: Option<i64>,
    ) -> anyhow::Result<T>
    where
        P: serde::Serialize,
        T: serde::de::DeserializeOwned + Send + Sync + 'static,
    {
        let mut query = contract.call_function(method, params).unwrap().read_only();

        // Apply block reference if provided
        if let Some(block) = block_id {
            query = query.at(Reference::AtBlock(block as u64));
        }

        // Execute the query
        match query.fetch_from(&self.network).await {
            Ok(data) => Ok(data.data),
            Err(e) => {
                let context = if let Some(block) = block_id {
                    format!("at block {}", block)
                } else {
                    "at current block".to_string()
                };

                eprintln!("Failed to query method '{}' {}: {:?}", method, context, e);
                Err(anyhow::anyhow!(
                    "Failed to query method '{}' {}: {:?}",
                    method,
                    context,
                    e
                ))
            }
        }
    }

    // sputnik contract
    pub async fn get_dao_proposal(
        &self,
        contract: Contract,
        proposal_id: i64,
        block_id: Option<i64>,
    ) -> anyhow::Result<ProposalOutput> {
        println!("get_dao_proposal: {:?}", proposal_id);
        self.query_contract(
            contract,
            "get_proposal",
            json!({ "id": proposal_id }),
            block_id,
        )
        .await
    }

    // sputnik contract
    pub async fn get_dao_proposals(
        &self,
        contract: Contract,
        from_index: i32,
        limit: i32,
        block_id: Option<i64>,
    ) -> anyhow::Result<Vec<ProposalOutput>> {
        self.query_contract(
            contract,
            "get_proposals",
            json!({ "from_index": from_index, "limit": limit }),
            block_id,
        )
        .await
    }

    // sputnik contract
    pub async fn get_last_proposal_id(
        &self,
        contract: Contract,
        block_id: Option<i64>,
    ) -> anyhow::Result<i64> {
        self.query_contract(contract, "get_last_proposal_id", json!({}), block_id)
            .await
    }

    // sputnik contract
    pub async fn get_dao_proposal_on_block(
        &self,
        contract: Contract,
        proposal_id: i64,
        block_id: i64,
    ) -> anyhow::Result<ProposalOutput> {
        self.get_dao_proposal(contract, proposal_id, Some(block_id))
            .await
    }

    // sputnik contract
    pub async fn get_last_proposal_id_on_block(
        &self,
        contract: Contract,
        block_id: i64,
    ) -> anyhow::Result<i64> {
        println!("Attempting to get last proposal ID at block {}", block_id);
        println!("Using contract: {}", self.contract.0);
        self.get_last_proposal_id(contract, Some(block_id)).await
    }
}
