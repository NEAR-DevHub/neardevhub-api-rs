use devhub_shared::proposal::VersionedProposal;
use devhub_shared::rfp::VersionedRFP;
use near_account_id::AccountId;
use near_api::{types::reference::Reference, types::Data};
use near_api::{Chain, Contract, NetworkConfig, RPCEndpoint};
use near_jsonrpc_client::methods::query::RpcQueryRequest;
use rocket::http::Status;
use rocket::serde::json::json;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RpcResponse {
    pub data: String,
}

#[derive(Clone)]
pub struct RpcService {
    network: NetworkConfig,
    contract: Contract,
}

#[derive(Deserialize)]
struct QueryResponse {
    // jsonrpc: String,
    result: QueryResponseResult,
    // id: String,
}

#[derive(Deserialize)]
struct QueryResponseResult {
    // result is an array of bytes, to be specific it is an ASCII code of the string
    result: Vec<i32>,
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
        let mut network = NetworkConfig::mainnet();

        // FASTNEAR RPC provider as back up when our tests hit a toomanyrequests error.
        network.rpc_endpoints.push(RPCEndpoint::new(
            "https://free.rpc.fastnear.com".parse().unwrap(),
        ));

        Self {
            network,
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

    pub async fn get_all_rfp_ids(&self) -> Result<Vec<i32>, Status> {
        let result: Result<Data<Vec<i32>>, _> = self
            .contract
            .call_function("get_all_rfp_ids", ())
            .unwrap()
            .read_only()
            .fetch_from(&self.network)
            .await;

        match result {
            Ok(res) => Ok(res.data),
            Err(e) => {
                eprintln!("Failed to get all rfp ids: {:?}", e);
                Err(Status::InternalServerError)
            }
        }
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

    pub async fn block_timestamp(&self, block_id: u64) -> Result<u64, Status> {
        let block = Chain::block()
            .at(Reference::AtBlock(block_id))
            .fetch_from(&self.network)
            .await;

        match block {
            Ok(block) => Ok(block.header.timestamp),
            Err(e) => {
                eprintln!("Failed to get block timestamp: {:?}", e);
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

    pub async fn query(
        &self,
        method_name: String,
        block_id: i64,
        args_base64: String,
    ) -> Result<String, Status> {
        let args = json!({
          "request_type": "call_function",
          "account_id": self.contract.0.to_string(),
          "block_id": block_id,
          "method_name": method_name,
          "args_base64": args_base64
        });

        println!("Querying args: {:?}", args);

        let result: Result<Data<QueryResponse>, _> = self
            .contract
            .call_function("query", args)
            .unwrap()
            .read_only()
            .fetch_from(&self.network)
            .await;

        match result {
            Ok(res) => {
                // From ascii code to string
                let decoded = res
                    .data
                    .result
                    .result
                    .iter()
                    .map(|c| *c as u8 as char)
                    .collect();
                // Should return JSON object?
                Ok(decoded)
            }
            Err(e) => {
                eprintln!("Failed to query: {:?}", e);
                Err(Status::InternalServerError)
            }
        }
    }
}
