use crate::db::db_types::SputnikProposalSnapshotRecord;
use crate::db::DB;
use crate::entrypoints::sputnik::sputnik_types::{
    Action, Proposal, ProposalKind, ProposalOutput, ProposalStatus,
};
use crate::nearblocks_client::types::Transaction;
use crate::rpc_service::RpcService;
use near_account_id::AccountId;
use near_sdk::json_types::U64;
use regex::Regex;
use rocket::{http::Status, State};
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
    for key in &keys {
        let markdown_key = parse_key_to_readable_format(key);
        let re = Regex::new(r"^\* (.+): (.+)$").unwrap();

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

pub async fn handle_add_proposal(
    transaction: Transaction,
    db: &State<DB>,
    contract: &AccountId,
) -> Result<(), Status> {
    let rpc_service = RpcService::new(contract);
    /*
      get_last_proposal_id actually returns the number of proposals starting from 0.
      https://github.com/near-daos/sputnik-dao-contract/blob/3d568f9517a8c7a6510786d978bb25b180501841/sputnikdao2/src/proposals.rs#L532
    */
    let proposal_id = match rpc_service
        .get_last_proposal_id_on_block(transaction.receipt_block.block_height)
        .await
    {
        Ok(last_proposal_id) => last_proposal_id.data - 1,
        Err(e) => {
            eprintln!("Failed to get last dao proposal id on block: {:?}", e);
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            println!("Trying again");
            rpc_service
                .get_last_proposal_id_on_block(transaction.receipt_block.block_height)
                .await
                .unwrap()
                .data
                - 1
        }
    };

    let add_proposal_action = transaction
        .actions
        .as_ref()
        .and_then(|actions| actions.first())
        .ok_or(Status::InternalServerError)?;
    let add_proposal_args: AddProposalArgs = serde_json::from_str(
        add_proposal_action
            .args
            .as_ref()
            .unwrap_or(&String::default()),
    )
    .unwrap();

    let proposal_output = ProposalOutput {
        id: proposal_id.try_into().unwrap(),
        proposal: Proposal {
            proposer: AccountId::from_str(transaction.predecessor_account_id.as_str()).unwrap(),
            description: add_proposal_args.proposal.description,
            kind: add_proposal_args.proposal.kind,
            status: ProposalStatus::Removed,
            vote_counts: HashMap::new(),
            votes: HashMap::new(),
            submission_time: U64::from(transaction.receipt_block.block_height as u64),
        },
    };

    let daop = match rpc_service
        .get_dao_proposal_on_block(proposal_id, transaction.receipt_block.block_height)
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
        eprintln!("Failed to begin transaction: {:?}", e);
        Status::InternalServerError
    })?;

    let kind = serde_json::to_value(daop.proposal.kind).unwrap_or_else(|e| {
        eprintln!("Failed to serialize proposal kind: {:?}", e);
        serde_json::Value::Null
    });

    let vote_counts = serde_json::to_value(daop.proposal.vote_counts).unwrap_or_else(|e| {
        eprintln!("Failed to serialize vote counts: {:?}", e);
        serde_json::Value::Null
    });

    let votes = serde_json::to_value(&daop.proposal.votes).unwrap_or_else(|e| {
        eprintln!("Failed to serialize votes: {:?}", e);
        serde_json::Value::Null
    });

    let record = SputnikProposalSnapshotRecord {
        description: daop.proposal.description,
        id: format!("{}_{}", daop.id, contract),
        proposal_id: daop.id as i32,
        kind,
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
    };

    DB::upsert_dao_proposal_snapshot(&mut tx, record)
        .await
        .map_err(|e| {
            eprintln!("Failed to insert transactions {}: {:?}", transaction.id, e);
            Status::InternalServerError
        })?;

    tx.commit().await.map_err(|e| {
        eprintln!("Failed to commit transaction: {:?}", e);
        Status::InternalServerError
    })?;

    println!("Inserted proposal snapshot {}", daop.id);

    Ok(())
}

pub async fn handle_act_proposal(
    transaction: Transaction,
    db: &State<DB>,
    contract: &AccountId,
) -> Result<(), rocket::http::Status> {
    let action = transaction
        .actions
        .as_ref()
        .and_then(|actions| actions.first())
        .ok_or(Status::InternalServerError)?;

    let json_args = action.args.clone();

    let args: ActProposalArgs = serde_json::from_str(&json_args.unwrap_or_default()).unwrap();

    let proposal_id = args.id;

    let rpc_service = RpcService::new(contract);

    let block_id = transaction.receipt_block.block_height;

    // This will error if the proposal is removed.
    let dao_proposal = match rpc_service
        .get_dao_proposal_on_block(proposal_id, block_id)
        .await
    {
        Ok(dao_proposal) => dao_proposal,
        Err(e) => {
            if args.action == Action::VoteRemove || args.action == Action::RemoveProposal {
                println!("Updating proposal status to Removed");
                db.update_proposal_status(format!("{}_{}", proposal_id, contract), "Removed")
                    .await?;
                return Ok(());
            }
            eprintln!(
                "Failed to get proposal in act_proposal with id: {} and block_id: {}, Error: {:?}",
                proposal_id, block_id, e
            );
            return Err(Status::InternalServerError);
        }
    };

    let proposal_action = decode_proposal_description(&dao_proposal.proposal.description);

    let mut tx = db.begin().await.map_err(|e| {
        eprintln!("Failed to begin transaction: {:?}", e);
        Status::InternalServerError
    })?;

    let kind = serde_json::to_value(dao_proposal.proposal.kind).unwrap_or_else(|e| {
        eprintln!("Failed to serialize proposal kind: {:?}", e);
        serde_json::Value::Null
    });

    let vote_counts = serde_json::to_value(dao_proposal.proposal.vote_counts).unwrap_or_else(|e| {
        eprintln!("Failed to serialize vote counts: {:?}", e);
        serde_json::Value::Null
    });

    let votes = serde_json::to_value(&dao_proposal.proposal.votes).unwrap_or_else(|e| {
        eprintln!("Failed to serialize votes: {:?}", e);
        serde_json::Value::Null
    });
    DB::upsert_dao_proposal_snapshot(
        &mut tx,
        SputnikProposalSnapshotRecord {
            description: dao_proposal.proposal.description,
            id: format!("{}_{}", dao_proposal.id, contract),
            proposal_id: dao_proposal.id.try_into().unwrap(),
            kind,
            proposer: dao_proposal.proposal.proposer.to_string(),
            status: dao_proposal.proposal.status.to_string(),
            submission_time: dao_proposal.proposal.submission_time.0 as i64,
            vote_counts,
            votes,
            total_votes: dao_proposal.proposal.votes.len() as i32,
            dao_instance: contract.to_string(),
            proposal_action,
            tx_timestamp: transaction.block_timestamp.parse::<i64>().unwrap(),
            hash: transaction.transaction_hash,
        },
    )
    .await
    .map_err(|e| {
        eprintln!("Failed to insert transactions {}: {:?}", transaction.id, e);
        Status::InternalServerError
    })?;

    tx.commit().await.map_err(|e| {
        eprintln!("Failed to commit transaction: {:?}", e);
        Status::InternalServerError
    })?;

    Ok(())
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
