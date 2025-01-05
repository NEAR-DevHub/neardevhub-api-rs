use crate::db::db_types::SputnikProposalSnapshotRecord;
use crate::db::DB;
use crate::entrypoints::sputnik::sputnik_types::{Action, ProposalKind};
use crate::nearblocks_client::types::Transaction;
use crate::rpc_service::RpcService; // BLOCK_HEIGHT_OFFSET
use near_account_id::AccountId;
use regex::Regex;
use rocket::{http::Status, State};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::types::BLOCK_HEIGHT_OFFSET;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AddProposalArgs {
    pub proposal: ProposalInput,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ProposalInput {
    pub description: String,
    pub kind: ProposalKind,
}

#[derive(Serialize, Deserialize)] // Clone
pub struct ActProposalArgs {
    pub id: i64,
    pub action: Action,
    pub memo: Option<String>,
}

fn parse_key_to_readable_format(key: &str) -> String {
    // Replace underscores with spaces
    let key = key.replace('_', " ");

    // Add spaces between camelCase or PascalCase words
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

fn decode_proposal_description(key: &str, description: &str) -> String {
    // Try to parse as JSON
    if let Ok(parsed_data) = serde_json::from_str::<Value>(description) {
        if let Some(value) = parsed_data.get(key) {
            return value.as_str().unwrap_or("parsed error 1.").to_string();
        }
    }

    // Handle as markdown
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

    "key not found".to_string() // Return None if key not found
}

pub async fn handle_add_proposal(
    transaction: Transaction,
    db: &State<DB>,
    contract: &AccountId,
) -> Result<(), Status> {
    let rpc_service = RpcService::new(contract);

    // get last proposal id
    let last_proposal_id = match rpc_service
        .get_last_dao_proposal_id_on_block(
            transaction.receipt_block.block_height + BLOCK_HEIGHT_OFFSET,
        )
        .await
    {
        Ok(last_proposal_id) => last_proposal_id.data - 1, // TODO TEST
        Err(e) => {
            eprintln!("Failed to get last dao proposal id on block: {:?}", e);
            return Err(Status::InternalServerError);
        }
    };

    println!("Last proposal id: {}", last_proposal_id);

    // TODO either check get the last dao proposal id on block or just add 1 to the id.
    // on block => BLOCK_HEIGHT_OFFSET ?
    // let proposal_id = last_proposal_id + 1;

    let daop = match rpc_service
        .get_dao_proposal_on_block(
            last_proposal_id,
            transaction.receipt_block.block_height + BLOCK_HEIGHT_OFFSET,
        )
        .await
    {
        Ok(daop) => daop,
        Err(e) => {
            eprintln!(
                "Failed to get dao proposal on block: {:?}, block_height: {}",
                e, transaction.receipt_block.block_height
            );
            println!(
                "Skipping proposal, probably deleted, id:{}",
                last_proposal_id
            );
            return Ok(());
        }
    };

    println!("Proposal: {:?}", daop.id);
    println!("Proposal description: {:?}", daop.proposal.description);

    let proposal_action = decode_proposal_description("isStakeRequest", &daop.proposal.description); // TODO check v1 isStakeRequest as well

    println!("Proposal action: {}", proposal_action);
    let mut tx = db.begin().await.map_err(|e| {
        eprintln!("Failed to begin transaction: {:?}", e);
        Status::InternalServerError
    })?;

    let record = SputnikProposalSnapshotRecord {
        description: daop.proposal.description,
        id: daop.id.try_into().unwrap(),
        kind: serde_json::to_value(daop.proposal.kind).unwrap(),
        proposer: daop.proposal.proposer.to_string(),
        status: daop.proposal.status.to_string(),
        submission_time: daop.proposal.submission_time.0 as i64,
        vote_counts: serde_json::to_value(daop.proposal.vote_counts).unwrap(),
        votes: serde_json::to_value(&daop.proposal.votes).unwrap(),
        total_votes: daop.proposal.votes.len() as i32,
        dao_instance: contract.to_string(),
        proposal_action,
        tx_timestamp: transaction.block_timestamp.parse::<i64>().unwrap(),
        hash: transaction.transaction_hash,
    };

    println!("Inserting proposal snapshot {:?}", record);
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

// TODO: instead of arguments parsing get the updated version of the proposal from the contract via the RPC
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
    // let action = args.action;

    let rpc_service = RpcService::new(contract);

    let dao_proposal = match rpc_service
        .get_dao_proposal_on_block(
            proposal_id,
            transaction.receipt_block.block_height + BLOCK_HEIGHT_OFFSET,
        )
        .await
    {
        Ok(dao_proposal) => dao_proposal,
        Err(e) => {
            eprintln!("Failed to get dao proposal: {:?}", e);
            // TODO
            println!("Skipping proposal {:?}", proposal_id);
            return Ok(());
        }
    };

    let proposal_action =
        decode_proposal_description("proposal_action", &dao_proposal.proposal.description); // TODO check v1 isStakeRequest as well

    let mut tx = db.begin().await.map_err(|e| {
        eprintln!("Failed to begin transaction: {:?}", e);
        Status::InternalServerError
    })?;

    DB::upsert_dao_proposal_snapshot(
        &mut tx,
        SputnikProposalSnapshotRecord {
            description: dao_proposal.proposal.description,
            id: dao_proposal.id.try_into().unwrap(),
            kind: serde_json::to_value(dao_proposal.proposal.kind).unwrap(),
            proposer: dao_proposal.proposal.proposer.to_string(),
            status: dao_proposal.proposal.status.to_string(),
            submission_time: dao_proposal.proposal.submission_time.0 as i64,
            vote_counts: serde_json::to_value(dao_proposal.proposal.vote_counts).unwrap(),
            votes: serde_json::to_value(&dao_proposal.proposal.votes).unwrap(),
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

    Ok(())
}
