use crate::db::db_types::{HandlerError, SputnikProposalSnapshotRecord};
use crate::db::DB;
use crate::entrypoints::sputnik::sputnik_types::{
    Action, Proposal, ProposalKind, ProposalOutput, ProposalStatus, Vote,
};
use crate::nearblocks_client::types::Transaction;
use crate::rpc_service::RpcService;
use near_account_id::AccountId;
use near_api::Contract;
use near_sdk::json_types::U64;
use regex::Regex;
use rocket::State;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AddProposalArgs {
    pub proposal: ProposalInput,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ProposalInput {
    pub description: String,
    pub kind: ProposalKind,
}

#[derive(Serialize, Deserialize, Debug)] // Clone
pub struct ActProposalArgs {
    pub id: i64,
    pub action: Action,
    pub memo: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ProposalDescriptionData {
    pub proposal_action: Option<String>,
    pub is_transfer: bool,
    pub is_asset_exchange: bool,
    pub is_stake_request: bool,
    pub raw_data: HashMap<String, String>,
}

pub fn parse_proposal_description(description: &str) -> ProposalDescriptionData {
    // First try to parse as JSON
    if let Ok(json_value) = serde_json::from_str::<Value>(description) {
        if let Some(obj) = json_value.as_object() {
            let mut result = ProposalDescriptionData::default();

            // Extract raw data
            for (key, value) in obj {
                if let Some(str_value) = value.as_str() {
                    result.raw_data.insert(key.clone(), str_value.to_string());
                } else {
                    result.raw_data.insert(key.clone(), value.to_string());
                }
            }

            // Extract specific fields
            if let Some(action) = obj.get("proposal_action").and_then(|v| v.as_str()) {
                result.proposal_action = Some(action.to_string());
                result.is_transfer = action == "transfer";
                result.is_asset_exchange = action == "asset-exchange";
                result.is_stake_request = matches!(action, "stake" | "unstake" | "withdraw");
            }

            // Check for explicit stake request flag
            if let Some(is_stake) = obj.get("isStakeRequest") {
                result.is_stake_request = is_stake.as_bool().unwrap_or(false)
                    || is_stake.as_str().map(|s| s == "true").unwrap_or(false);
            }

            return result;
        }
    }

    // If JSON parsing fails, try markdown format
    let mut result = ProposalDescriptionData::default();

    for line in description.split("<br>") {
        if let Some(captures) = line.strip_prefix("* ").and_then(|s| {
            let parts: Vec<&str> = s.splitn(2, ": ").collect();
            if parts.len() == 2 {
                Some((parts[0], parts[1]))
            } else {
                None
            }
        }) {
            let (key, value) = captures;

            // Convert "Key Name" to "key_name"
            let normalized_key = key.trim().to_lowercase().replace(" ", "_");
            let value = value.trim().to_string();

            // Store in raw data
            result
                .raw_data
                .insert(normalized_key.clone(), value.clone());

            // Set specific flags
            if normalized_key == "proposal_action" {
                result.proposal_action = Some(value.clone());
                result.is_transfer = value == "transfer";
                result.is_asset_exchange = value == "asset-exchange";
                result.is_stake_request =
                    matches!(value.as_str(), "stake" | "unstake" | "withdraw");
            } else if normalized_key == "isstakerequest" {
                result.is_stake_request = value == "true";
            }
        }
    }

    result
}

fn parse_key_to_readable_format(key: &str) -> String {
    let key = key.replace('_', " ");
    let re = Regex::new(r"([a-z])([A-Z])").unwrap();
    let key = re.replace_all(&key, "$1 $2");

    // Capitalize each word
    key.split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(first_char) => first_char.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}

fn decode_proposal_description(description: &str) -> String {
    // Try to parse as JSON
    if let Ok(parsed_data) = serde_json::from_str::<Value>(description) {
        if let Some(value) = parsed_data.get("proposal_action") {
            return value.to_string();
        }
        if let Some(value) = parsed_data.get("isStakeRequest") {
            return value.to_string();
        }
    }

    // Handle as markdown
    let keys = ["proposal_action", "isStakeRequest"];
    let re = Regex::new(r"^\* (.+): (.+)$").unwrap();
    for key in &keys {
        let markdown_key = parse_key_to_readable_format(key);

        for line in description.split("<br>") {
            if let Some(captures) = re.captures(line) {
                let current_key = captures.get(1).map_or("", |m| m.as_str());
                let value = captures.get(2).map_or("", |m| m.as_str());

                if current_key == markdown_key {
                    return value.trim().to_string();
                }
            }
        }
    }

    "null".to_string()
}

/**
 * TODO we could get the last stored proposal id from the database. if we stored 1,2 and 3 this is 4.
 * FIXME: the RPC returns that some of these blocks have been garbage collected.
 */
pub async fn handle_add_proposal(
    transaction: Transaction,
    db: &State<DB>,
    contract: &AccountId,
    rpc_service: &RpcService,
) -> anyhow::Result<i64> {
    let proposal_id = match rpc_service
        .get_last_proposal_id_on_block(
            Contract(contract.clone()),
            transaction.receipt_block.block_height,
        )
        .await
    {
        Ok(last_proposal_id) => last_proposal_id - 1,
        Err(e) => {
            eprintln!("fatal:Failed to get last dao proposal id on block: {:?}", e);
            let _ = db
                .track_handler_error(HandlerError {
                    transaction_id: transaction.id,
                    error_type: format!(
                        "RPC service failed rpc_service.get_last_proposal_id_on_block({})",
                        transaction.receipt_block.block_height
                    ),
                    message: e.to_string(),
                    block_height: transaction.block.block_height,
                    timestamp: chrono::Utc::now(),
                })
                .await;
            return Err(anyhow::anyhow!(
                "fatal: Failed to get last proposal ID in add_proposal"
            ));
        }
    };

    // Extract the arguments out of the action.
    let add_proposal_action = transaction
        .actions
        .as_ref()
        .and_then(|actions| actions.first())
        .ok_or(anyhow::anyhow!("fatal: Failed to get add proposal action"))?;

    let add_proposal_args: AddProposalArgs = serde_json::from_str(
        add_proposal_action
            .args
            .as_ref()
            .unwrap_or(&String::default()),
    )
    .map_err(|e| {
        eprintln!("Failed to parse AddProposalArgs: {}", e);
        anyhow::anyhow!("fatal: Failed to parse AddProposalArgs")
    })?;

    let proposer = AccountId::from_str(transaction.predecessor_account_id.as_str())
        .map_err(|_| anyhow::anyhow!("fatal: Invalid account ID"))?;

    let proposal_output = ProposalOutput {
        id: proposal_id.try_into().map_err(|e| {
            eprintln!("Failed to convert proposal_id: {}", e);
            anyhow::anyhow!("fatal: Failed to convert proposal_id")
        })?,
        proposal: Proposal {
            proposer,
            description: add_proposal_args.proposal.description,
            kind: add_proposal_args.proposal.kind,
            status: ProposalStatus::Removed,
            vote_counts: HashMap::new(),
            votes: HashMap::new(),
            submission_time: U64::from(transaction.receipt_block.block_height as u64),
        },
    };

    let daop = match rpc_service
        .get_dao_proposal_on_block(
            Contract(contract.clone()),
            proposal_id,
            transaction.receipt_block.block_height,
        )
        .await
    {
        Ok(daop) => daop,
        Err(e) => {
            eprintln!(
                "Failed to get dao proposal on block_height: {} with error: {}",
                transaction.receipt_block.block_height, e
            );
            proposal_output
        }
    };

    let mut tx = db.begin().await.map_err(|e| {
        eprintln!("fatal: Failed to begin transaction: {:?}", e);
        anyhow::anyhow!("fatal: Failed to begin transaction")
    })?;

    let proposal_action = decode_proposal_description(&daop.proposal.description);

    let record = SputnikProposalSnapshotRecord {
        description: daop.proposal.description,
        id: format!("{}_{}", daop.id, contract),
        proposal_id: daop.id as i32,
        kind: daop.proposal.kind.to_json(),
        kind_variant_name: daop.proposal.kind.variant_name(),
        receiver_id: daop.proposal.kind.receiver_id(),
        token_id: daop.proposal.kind.token_id(),
        token_amount: daop.proposal.kind.token_amount(),
        proposer: daop.proposal.proposer.to_string(),
        status: daop.proposal.status.to_string(),
        submission_time: daop.proposal.submission_time.0 as i64,
        vote_counts: to_json_or_null(&daop.proposal.vote_counts),
        votes: to_json_or_null(&daop.proposal.votes),
        total_votes: daop.proposal.votes.len() as i32,
        dao_instance: contract.to_string(),
        proposal_action,
        approvers: None,
        tx_timestamp: transaction.block_timestamp.parse::<i64>().unwrap(),
        hash: transaction.transaction_hash,
        block_height: transaction.block.block_height,
    };

    DB::upsert_dao_proposal_snapshot(&mut tx, record)
        .await
        .map_err(|e| {
            eprintln!(
                "fatal: Failed to insert transactions {}: {:?}",
                transaction.id, e
            );
            anyhow::anyhow!("fatal: Failed to insert transactions")
        })?;

    DB::set_after_block(&mut tx, contract, transaction.block.block_height)
        .await
        .map_err(|e| {
            eprintln!(
                "fatal: Failed to set last updated info for contract: {:?}",
                e
            );
            anyhow::anyhow!("fatal: Failed to set last updated info for contract")
        })?;

    tx.commit().await.map_err(|e| {
        eprintln!("fatal: Failed to commit transaction: {:?}", e);
        anyhow::anyhow!("fatal: Failed to commit transaction")
    })?;

    println!("Inserted proposal snapshot {}", daop.id);

    Ok(transaction.block.block_height)
}

pub fn to_json_or_null<T: Serialize>(value: &T) -> Value {
    serde_json::to_value(value).unwrap_or_else(|e| {
        eprintln!("Failed to serialize: {:?}", e);
        serde_json::Value::Null
    })
}

/**
 * Get's the approvers from the votes to store in a different table to query it for the history dropdown filter.
 */
fn get_approvers(votes: &HashMap<AccountId, Vote>) -> Vec<String> {
    votes
        .iter()
        .map(|(account_id, _)| account_id.to_string())
        .collect()
}

pub async fn handle_act_proposal(
    transaction: Transaction,
    db: &State<DB>,
    contract: &AccountId,
    rpc_service: &RpcService,
) -> anyhow::Result<i64> {
    let action = transaction
        .actions
        .as_ref()
        .and_then(|actions| actions.first())
        .ok_or(anyhow::anyhow!("Failed to get act proposal action"))?;

    let json_args = action.args.clone();

    let args: ActProposalArgs =
        serde_json::from_str(&json_args.unwrap_or_default()).map_err(|e| {
            eprintln!("Failed to parse ActProposalArgs: {}", e);
            anyhow::anyhow!("Failed to parse ActProposalArgs")
        })?;

    let proposal_id = args.id;
    let block_id = transaction.receipt_block.block_height;

    // This will error if the proposal is removed.
    let dao_proposal = match rpc_service
        .get_dao_proposal_on_block(Contract(contract.clone()), proposal_id, block_id)
        .await
    {
        Ok(dao_proposal) => dao_proposal,
        Err(e) => {
            if args.action == Action::VoteRemove || args.action == Action::RemoveProposal {
                println!("Updating proposal status to Removed");
                db.update_proposal_status(format!("{}_{}", proposal_id, contract), "Removed")
                    .await
                    .map_err(|e| {
                        eprintln!("Failed to update proposal status: {:?}", e);
                        anyhow::anyhow!("Failed to update proposal status")
                    })?;
                return Ok(transaction.block.block_height);
            }
            eprintln!(
                "Failed to get proposal in act_proposal with id: {} and block_id: {}, Error: {:?}",
                proposal_id, block_id, e
            );
            let _ = db
                .track_handler_error(HandlerError {
                    transaction_id: transaction.id,
                    error_type: format!(
                        "RPC service failed rpc_service.get_dao_proposal_on_block({}, {})",
                        proposal_id, block_id
                    ),
                    message: e.to_string(),
                    block_height: transaction.block.block_height,
                    timestamp: chrono::Utc::now(),
                })
                .await;
            return Err(anyhow::anyhow!(
                "fatal: Failed to get proposal from RPC in handle_act_proposal"
            ));
        }
    };

    let mut tx = db.begin().await.map_err(|e| {
        eprintln!("Failed to begin transaction: {:?}", e);
        anyhow::anyhow!("Failed to begin transaction")
    })?;

    let approvers = get_approvers(&dao_proposal.proposal.votes);
    let proposal_action = decode_proposal_description(&dao_proposal.proposal.description);

    DB::upsert_dao_approvers(&mut tx, contract.to_string().as_str(), &approvers)
        .await
        .map_err(|e| {
            eprintln!("Failed to upsert dao approvers: {:?}", e);
            anyhow::anyhow!("Failed to upsert dao approvers")
        })?;

    let timestamp = transaction.block_timestamp.parse::<i64>().map_err(|e| {
        eprintln!("Failed to parse block timestamp: {}", e);
        anyhow::anyhow!("Failed to parse block timestamp")
    })?;

    DB::upsert_dao_proposal_snapshot(
        &mut tx,
        SputnikProposalSnapshotRecord {
            description: dao_proposal.proposal.description,
            id: format!("{}_{}", dao_proposal.id, contract),
            proposal_id: dao_proposal.id.try_into().map_err(|e| {
                eprintln!("Failed to convert proposal_id: {}", e);
                anyhow::anyhow!("Failed to convert proposal_id")
            })?,
            kind: dao_proposal.proposal.kind.to_json(),
            kind_variant_name: dao_proposal.proposal.kind.variant_name(),
            receiver_id: dao_proposal.proposal.kind.receiver_id(),
            token_id: dao_proposal.proposal.kind.token_id(),
            token_amount: dao_proposal.proposal.kind.token_amount(),
            proposer: dao_proposal.proposal.proposer.to_string(),
            status: dao_proposal.proposal.status.to_string(),
            submission_time: dao_proposal.proposal.submission_time.0 as i64,
            vote_counts: to_json_or_null(&dao_proposal.proposal.vote_counts),
            votes: to_json_or_null(&dao_proposal.proposal.votes),
            total_votes: dao_proposal.proposal.votes.len() as i32,
            dao_instance: contract.to_string(),
            approvers: Some(approvers),
            proposal_action,
            tx_timestamp: timestamp,
            hash: transaction.transaction_hash,
            block_height: transaction.block.block_height,
        },
    )
    .await
    .map_err(|e| {
        eprintln!("Failed to insert transactions {}: {:?}", transaction.id, e);
        anyhow::anyhow!("Failed to insert transactions")
    })?;

    DB::set_after_block(&mut tx, contract, transaction.block.block_height)
        .await
        .map_err(|e| {
            eprintln!(
                "fatal: Failed to set last updated info for contract: {:?}",
                e
            );
            anyhow::anyhow!("fatal: Failed to set last updated info for contract")
        })?;

    tx.commit().await.map_err(|e| {
        eprintln!("Failed to commit transaction: {:?}", e);
        anyhow::anyhow!("Failed to commit transaction")
    })?;

    Ok(transaction.block.block_height)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_decode_proposal_description() {
        let descriptions = [
            "{\"isStakeRequest\":true,\"warningNotes\":\"Approve to continue staking with this validator\"}",
            "* Proposal Action: withdraw <br>* Show After Proposal Id Approved: 61 <br>* Custom Notes: Following to [#61](/treasury-testing-infinex.near/widget/app?page=stake-delegation&selectedTab=History&highlightProposalId=61) unstake request",
            "{\"title\":\"DevHub Developer Contributor report by Megha for 09/09/2024 - 10/06/2024\",\"summary\":\"Worked on integrating new features to treasury dashboard, like asset exchange using the ref-sdk API, stake delegation, made first version live for devhub, fixed some bugs with devhub and other instances.\",\"notes\":\"Treasury balance insufficient\",\"proposalId\":220}",
            "Change policy",
            "{}",
            "* Proposal Action: stake <br>* Custom Notes: Approve to continue staking with this validator",
            "* Proposal Action: unstake <br>* Notes: Unstake 0.5N",
            "* Proposal Action: stake",
            "* Proposal Action: withdraw <br>* Show After Proposal Id Approved: 58 <br>* Custom Notes: Following to [#58](/treasury-testing-infinex.near/widget/app?page=stake-delegation&selectedTab=History&highlightProposalId=58) unstake request"
        ];

        let expected_results = [
            "true", "withdraw", "null", "null", "null", "stake", "unstake", "stake", "withdraw",
        ];

        for (i, description) in descriptions.iter().enumerate() {
            let result = decode_proposal_description(description);

            assert_eq!(
                result, expected_results[i],
                "expected: {} got: {}",
                expected_results[i], result
            );
        }
    }

    #[test]
    fn test_json_description() {
        let json = r#"{"proposal_action": "transfer", "amount": "100 NEAR"}"#;
        let result = parse_proposal_description(json);

        assert_eq!(result.proposal_action, Some("transfer".to_string()));
        assert!(result.is_transfer);
        assert!(!result.is_asset_exchange);
        assert!(!result.is_stake_request);
        assert_eq!(result.raw_data.get("amount"), Some(&"100 NEAR".to_string()));
    }

    #[test]
    fn test_number_of_descriptions() {
        let descriptions = [
          // Payments: proposal_action is transfer but proposalKind could also be transfer
          "{\"isStakeRequest\":true,\"warningNotes\":\"Approve to continue staking with this validator\"}",
          "* Proposal Action: withdraw <br>* Show After Proposal Id Approved: 61 <br>* Custom Notes: Following to [#61](/treasury-testing-infinex.near/widget/app?page=stake-delegation&selectedTab=History&highlightProposalId=61) unstake request",
          "{\"title\":\"DevHub Developer Contributor report by Megha for 09/09/2024 - 10/06/2024\",\"summary\":\"Worked on integrating new features to treasury dashboard, like asset exchange using the ref-sdk API, stake delegation, made first version live for devhub, fixed some bugs with devhub and other instances.\",\"notes\":\"Treasury balance insufficient\",\"proposalId\":220}",
          "Change policy",
          "{}",
          // Stake delegation: proposal_action is stake or unstake or withdraw and isStakeRequest is true.
          "* Proposal Action: stake <br>* Custom Notes: Approve to continue staking with this validator",
          "* Proposal Action: unstake <br>* Notes: Unstake 0.5N",
          "* Proposal Action: stake",
          "* Proposal Action: withdraw <br>* Show After Proposal Id Approved: 58 <br>* Custom Notes: Following to [#58](/treasury-testing-infinex.near/widget/app?page=stake-delegation&selectedTab=History&highlightProposalId=58) unstake request",
          // Asset exchange
          "* Proposal Action: asset-exchange <br>* Notes: test <br>* Token In: wrap.near <br>* Token Out: 17208628f84f5d6ad33f0da3bbbeb27ffcb398eac501a31bd6ad2011e36133a1 <br>* Amount In: 0.5 <br>* Slippage: 1 <br>* Amount Out: 1.40223",
          "* Proposal Action: asset-exchange <br>* Token In: near <br>* Token Out: wrap.near <br>* Amount In: 1 <br>* Slippage: 1 <br>* Amount Out: 1"
        ];

        let expected_results: Vec<ProposalDescriptionData> = vec![
            ProposalDescriptionData {
                proposal_action: None,
                is_transfer: false,
                is_asset_exchange: false,
                is_stake_request: true,
                raw_data: HashMap::from([("warningNotes".to_string(), "Approve to continue staking with this validator".to_string())]),
            },
            ProposalDescriptionData {
                proposal_action: Some("withdraw".to_string()),
                is_transfer: false,
                is_asset_exchange: false,
                is_stake_request: true,
                raw_data: HashMap::from([("showAfterProposalIdApproved".to_string(), "61".to_string()), ("customNotes".to_string(), "Following to [#61](/treasury-testing-infinex.near/widget/app?page=stake-delegation&selectedTab=History&highlightProposalId=61) unstake request".to_string())]),
            },
            // TODO: ask megha about these examples.
            ProposalDescriptionData {
                proposal_action: Some("withdraw".to_string()),
                is_transfer: true,
                is_asset_exchange: false,
                is_stake_request: false,
                raw_data: HashMap::new(),
            },
            ProposalDescriptionData {
                proposal_action: Some("withdraw".to_string()),
                is_transfer: true,
                is_asset_exchange: false,
                is_stake_request: false,
                raw_data: HashMap::new(),
            },
            ProposalDescriptionData {
                proposal_action: Some("withdraw".to_string()),
                is_transfer: true,
                is_asset_exchange: false,
                is_stake_request: false,
                raw_data: HashMap::new(),
            },
            ProposalDescriptionData {
                proposal_action: Some("withdraw".to_string()),
                is_transfer: true,
                is_asset_exchange: false,
                is_stake_request: false,
                raw_data: HashMap::new(),
            },
            ProposalDescriptionData {
                proposal_action: Some("withdraw".to_string()),
                is_transfer: true,
                is_asset_exchange: false,
                is_stake_request: false,
                raw_data: HashMap::new(),
            },
            ProposalDescriptionData {
                proposal_action: Some("withdraw".to_string()),
                is_transfer: true,
                is_asset_exchange: false,
                is_stake_request: false,
                raw_data: HashMap::new(),
            },
        ];
        for (i, description) in descriptions.iter().enumerate() {
            let result = parse_proposal_description(description);
            assert_eq!(result.proposal_action, expected_results[i].proposal_action);
        }
    }

    #[test]
    fn test_markdown_description() {
        let markdown = "* Proposal Action: stake\n* Amount: 200 NEAR<br>* Duration: 30 days";
        let result = parse_proposal_description(markdown);

        assert_eq!(result.proposal_action, Some("stake".to_string()));
        assert!(!result.is_transfer);
        assert!(!result.is_asset_exchange);
        assert!(result.is_stake_request);
        assert_eq!(result.raw_data.get("amount"), Some(&"200 NEAR".to_string()));
    }

    #[test]
    fn test_explicit_stake_flag() {
        let json = r#"{"isStakeRequest": true, "amount": "300 NEAR"}"#;
        let result = parse_proposal_description(json);

        assert!(result.is_stake_request);
    }
}
