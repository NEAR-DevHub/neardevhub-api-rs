use crate::db::db_types::ProposalSnapshotRecord;
use devhub_shared::proposal::{
    Proposal, ProposalFundingCurrency, ProposalId, VersionedProposalBody,
};
use near_sdk::near;
use rocket::form::FromForm;
use rocket::serde::{Deserialize, Serialize};
use std::collections::HashSet;
use utoipa::ToSchema;
// NOTE should this be VersionedProposal instead of Proposal?
use devhub_shared::proposal::Proposal as ContractProposal;

#[derive(Clone, Debug, FromForm, ToSchema)]
pub struct GetProposalFilters {
    pub category: Option<String>,
    pub labels: Option<Vec<String>>,
    pub input: Option<String>,
    pub author_id: Option<String>,
    pub stage: Option<String>,
    pub block_timestamp: Option<i64>,
}

pub trait ProposalBodyFields {
    fn get_name(&self) -> &String;
    fn get_category(&self) -> &String;
    fn get_summary(&self) -> &String;
    fn get_description(&self) -> &String;
    fn get_linked_proposals(&self) -> &Vec<u32>;
    fn get_requested_sponsorship_usd_amount(&self) -> &u32;
    fn get_requested_sponsorship_paid_in_currency(&self) -> String;
    fn get_requested_sponsor(&self) -> String;
    fn get_receiver_account(&self) -> String;
    fn get_supervisor(&self) -> Option<String>;
    fn get_timeline(&self) -> String;
    fn get_linked_rfp(&self) -> &Option<u32>;
}

pub trait ProposalFundingCurrencyToString {
    fn to_string(&self) -> String;
}

impl ProposalFundingCurrencyToString for ProposalFundingCurrency {
    fn to_string(&self) -> String {
        match self {
            ProposalFundingCurrency::NEAR => "NEAR".to_string(),
            ProposalFundingCurrency::USDT => "USDT".to_string(),
            ProposalFundingCurrency::USDC => "USDC".to_string(),
            ProposalFundingCurrency::OTHER => "OTHER".to_string(),
        }
    }
}

impl ProposalBodyFields for VersionedProposalBody {
    fn get_name(&self) -> &String {
        match self {
            VersionedProposalBody::V0(body) => &body.name,
            VersionedProposalBody::V1(body) => &body.name,
            VersionedProposalBody::V2(body) => &body.name,
        }
    }

    fn get_category(&self) -> &String {
        match self {
            VersionedProposalBody::V0(body) => &body.category,
            VersionedProposalBody::V1(body) => &body.category,
            VersionedProposalBody::V2(body) => &body.category,
        }
    }

    fn get_summary(&self) -> &String {
        match self {
            VersionedProposalBody::V0(body) => &body.summary,
            VersionedProposalBody::V1(body) => &body.summary,
            VersionedProposalBody::V2(body) => &body.summary,
        }
    }

    fn get_description(&self) -> &String {
        match self {
            VersionedProposalBody::V0(body) => &body.description,
            VersionedProposalBody::V1(body) => &body.description,
            VersionedProposalBody::V2(body) => &body.description,
        }
    }

    fn get_linked_proposals(&self) -> &Vec<u32> {
        match self {
            VersionedProposalBody::V0(body) => &body.linked_proposals,
            VersionedProposalBody::V1(body) => &body.linked_proposals,
            VersionedProposalBody::V2(body) => &body.linked_proposals,
        }
    }

    fn get_linked_rfp(&self) -> &Option<u32> {
        match self {
            VersionedProposalBody::V0(_) => &None,
            VersionedProposalBody::V1(_) => &None,
            VersionedProposalBody::V2(body) => &body.linked_rfp,
        }
    }

    fn get_requested_sponsorship_usd_amount(&self) -> &u32 {
        match self {
            VersionedProposalBody::V0(body) => &body.requested_sponsorship_usd_amount,
            VersionedProposalBody::V1(body) => &body.requested_sponsorship_usd_amount,
            VersionedProposalBody::V2(body) => &body.requested_sponsorship_usd_amount,
        }
    }

    fn get_requested_sponsorship_paid_in_currency(&self) -> String {
        match self {
            VersionedProposalBody::V0(body) => {
                body.requested_sponsorship_paid_in_currency.to_string()
            }
            VersionedProposalBody::V1(body) => {
                body.requested_sponsorship_paid_in_currency.to_string()
            }
            VersionedProposalBody::V2(body) => {
                body.requested_sponsorship_paid_in_currency.to_string()
            }
        }
    }

    fn get_requested_sponsor(&self) -> String {
        match self {
            VersionedProposalBody::V0(body) => body.requested_sponsor.to_string(),
            VersionedProposalBody::V1(body) => body.requested_sponsor.to_string(),
            VersionedProposalBody::V2(body) => body.requested_sponsor.to_string(),
        }
    }

    fn get_receiver_account(&self) -> String {
        match self {
            VersionedProposalBody::V0(body) => body.receiver_account.to_string(),
            VersionedProposalBody::V1(body) => body.receiver_account.to_string(),
            VersionedProposalBody::V2(body) => body.receiver_account.to_string(),
        }
    }

    fn get_supervisor(&self) -> Option<String> {
        match self {
            VersionedProposalBody::V0(body) => body.supervisor.as_ref().map(|id| id.to_string()),
            VersionedProposalBody::V1(body) => body.supervisor.as_ref().map(|id| id.to_string()),
            VersionedProposalBody::V2(body) => body.supervisor.as_ref().map(|id| id.to_string()),
        }
    }

    fn get_timeline(&self) -> String {
        match self {
            VersionedProposalBody::V0(body) => {
                serde_json::to_string(&body.timeline).unwrap_or_default()
            }
            VersionedProposalBody::V1(body) => {
                serde_json::to_string(&body.timeline).unwrap_or_default()
            }
            VersionedProposalBody::V2(body) => {
                serde_json::to_string(&body.timeline).unwrap_or_default()
            }
        }
    }
}

pub trait FromContractProposal {
    fn from_contract_proposal(
        proposal: ContractProposal,
        timestamp: i64,
        block_height: i64,
    ) -> Self;
}

impl FromContractProposal for ProposalSnapshotRecord {
    fn from_contract_proposal(
        proposal: ContractProposal,
        timestamp: i64,
        block_height: i64,
    ) -> Self {
        ProposalSnapshotRecord {
            proposal_id: proposal.id as i32,
            block_height,
            ts: timestamp,
            editor_id: proposal.snapshot.editor_id.to_string(),
            social_db_post_block_height: proposal.social_db_post_block_height as i64,
            labels: serde_json::Value::from(Vec::from_iter(
                proposal.snapshot.labels.iter().cloned(),
            )),
            proposal_version: "V0".to_string(),
            proposal_body_version: "V2".to_string(),
            name: Some(proposal.snapshot.body.get_name().clone()),
            category: Some(proposal.snapshot.body.get_category().clone()),
            summary: Some(proposal.snapshot.body.get_summary().clone()),
            description: Some(proposal.snapshot.body.get_description().clone()),
            linked_proposals: Some(serde_json::Value::from(Vec::from_iter(
                proposal.snapshot.body.get_linked_proposals().to_vec(),
            ))),
            linked_rfp: proposal.snapshot.body.get_linked_rfp().map(|x| x as i32),
            requested_sponsorship_usd_amount: Some(
                *proposal
                    .snapshot
                    .body
                    .get_requested_sponsorship_usd_amount() as i32,
            ),
            requested_sponsorship_paid_in_currency: Some(
                proposal
                    .snapshot
                    .body
                    .get_requested_sponsorship_paid_in_currency()
                    .clone(),
            ),
            requested_sponsor: Some(proposal.snapshot.body.get_requested_sponsor().clone()),
            receiver_account: Some(proposal.snapshot.body.get_receiver_account().clone()),
            supervisor: proposal.snapshot.body.get_supervisor(),
            timeline: Some(serde_json::Value::from(
                proposal.snapshot.body.get_timeline().clone(),
            )),
            views: None,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct AddProposalArgs {
    body: VersionedProposalBody,
    labels: HashSet<String>,
    accepted_terms_and_conditions_version: Option<near_sdk::BlockHeight>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SetBlockHeightCallbackArgs {
    pub proposal: Proposal,
}

#[near(serializers=[borsh, json])]
#[derive(Clone)]
pub struct EditProposalArgs {
    pub id: ProposalId,
    pub body: VersionedProposalBody,
    pub labels: HashSet<String>,
}

#[derive(Deserialize, Clone)]
pub struct PartialEditProposalArgs {
    pub id: i32,
}
