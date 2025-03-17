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
    /*
      get_last_proposal_id actually returns the number of proposals starting from 0.
      https://github.com/near-daos/sputnik-dao-contract/blob/3d568f9517a8c7a6510786d978bb25b180501841/sputnikdao2/src/proposals.rs#L532
    */

    // println!("Transaction: {:?}", transaction);

    // let nearblocks_client = nearblocks_client::ApiClient::new();
    // let receipt = nearblocks_client
    //     .get_receipt_by_id(&transaction.receipt_id)
    //     .await?;
    // println!("Receipt: {:?}", receipt);

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
            // TODO: this is blocking we could get the last proposal id from the database (but reindexing would need to remove all proposals from db inorder for that to work. Or we get the last proposal id on a block or we get the receipt outcome.)
            return Err(anyhow::anyhow!(
                "non-fatal: Failed to get last proposal ID in add_proposal"
            ));
        }
    };

    // Get the proposal description and kind.
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

    let proposal_action = decode_proposal_description(&daop.proposal.description);

    let mut tx = db.begin().await.map_err(|e| {
        eprintln!("fatal: Failed to begin transaction: {:?}", e);
        anyhow::anyhow!("fatal: Failed to begin transaction")
    })?;

    let receiver_id = get_receiver_id(&daop.proposal.kind);
    let token_id = get_token_id(&daop.proposal.kind);
    let token_amount = get_token_amount(&daop.proposal.kind);

    let kind = serde_json::to_value(daop.proposal.kind).unwrap_or_else(|e| {
        eprintln!("fatal: Failed to serialize proposal kind: {:?}", e);
        serde_json::Value::Null
    });

    let vote_counts = serde_json::to_value(daop.proposal.vote_counts).unwrap_or_else(|e| {
        eprintln!("fatal: Failed to serialize vote counts: {:?}", e);
        serde_json::Value::Null
    });

    let votes = serde_json::to_value(&daop.proposal.votes).unwrap_or_else(|e| {
        eprintln!("fatal: Failed to serialize votes: {:?}", e);
        serde_json::Value::Null
    });

    let record = SputnikProposalSnapshotRecord {
        description: daop.proposal.description,
        id: format!("{}_{}", daop.id, contract),
        proposal_id: daop.id as i32,
        kind,
        receiver_id,
        token_id,
        token_amount,
        proposer: daop.proposal.proposer.to_string(),
        status: daop.proposal.status.to_string(),
        submission_time: daop.proposal.submission_time.0 as i64,
        vote_counts,
        votes,
        total_votes: daop.proposal.votes.len() as i32,
        dao_instance: contract.to_string(),
        proposal_action,
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

/**
 * Get's the receiver id from the proposal kind to store in a different table to query it for the history dropdown filter.
 */
fn get_receiver_id(kind: &ProposalKind) -> Option<String> {
    match kind {
        ProposalKind::FunctionCall { receiver_id, .. } => Some(receiver_id.to_string()),
        ProposalKind::UpgradeRemote { receiver_id, .. } => Some(receiver_id.to_string()),
        ProposalKind::Transfer { receiver_id, .. } => Some(receiver_id.to_string()),
        ProposalKind::BountyDone { receiver_id, .. } => Some(receiver_id.to_string()),
        _ => None,
    }
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

fn get_token_amount(kind: &ProposalKind) -> Option<String> {
    match kind {
        ProposalKind::Transfer { amount, .. } => Some(amount.0.to_string()),
        _ => None,
    }
}

fn get_token_id(kind: &ProposalKind) -> Option<String> {
    match kind {
        ProposalKind::Transfer { token_id, .. } => Some(token_id.to_string()),
        _ => None,
    }
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
            // TODO: this is blocking the PR sometimes we get this error
            //
            return Err(anyhow::anyhow!(
                "non-fatal: Failed to get proposal from RPC in handle_act_proposal"
            ));
        }
    };

    let proposal_action = decode_proposal_description(&dao_proposal.proposal.description);

    let mut tx = db.begin().await.map_err(|e| {
        eprintln!("Failed to begin transaction: {:?}", e);
        anyhow::anyhow!("Failed to begin transaction")
    })?;

    let token_amount = get_token_amount(&dao_proposal.proposal.kind);
    let receiver_id = get_receiver_id(&dao_proposal.proposal.kind);
    let token_id = get_token_id(&dao_proposal.proposal.kind);

    let kind = serde_json::to_value(dao_proposal.proposal.kind).unwrap_or_else(|e| {
        eprintln!("Failed to serialize proposal kind: {:?}", e);
        serde_json::Value::Null
    });

    let vote_counts = serde_json::to_value(dao_proposal.proposal.vote_counts).unwrap_or_else(|e| {
        eprintln!("Failed to serialize vote counts: {:?}", e);
        serde_json::Value::Null
    });

    let approvers = get_approvers(&dao_proposal.proposal.votes);
    DB::upsert_dao_approvers(&mut tx, contract.to_string().as_str(), &approvers)
        .await
        .map_err(|e| {
            eprintln!("Failed to upsert dao approvers: {:?}", e);
            anyhow::anyhow!("Failed to upsert dao approvers")
        })?;

    let votes = serde_json::to_value(&dao_proposal.proposal.votes).unwrap_or_else(|e| {
        eprintln!("Failed to serialize votes: {:?}", e);
        serde_json::Value::Null
    });

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
            kind,
            receiver_id,
            token_id,
            token_amount,
            proposer: dao_proposal.proposal.proposer.to_string(),
            status: dao_proposal.proposal.status.to_string(),
            submission_time: dao_proposal.proposal.submission_time.0 as i64,
            vote_counts,
            votes,
            total_votes: dao_proposal.proposal.votes.len() as i32,
            dao_instance: contract.to_string(),
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
}
