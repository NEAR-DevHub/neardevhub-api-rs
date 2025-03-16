use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

// u64 is not supported by sqlx postgres
// use near_sdk::{BlockHeight, Timestamp};
pub type BlockHeight = i64;
pub type Timestamp = i64;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct ProposalRecord {
    pub id: i32,
    pub author_id: String,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct LastUpdatedInfo {
    pub after_date: i64,
    pub after_block: i64,
    pub cursor: String,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct ProposalSnapshotRecord {
    pub proposal_id: i32,
    pub block_height: BlockHeight,
    pub ts: Timestamp,
    pub editor_id: String,
    pub social_db_post_block_height: BlockHeight,
    pub labels: serde_json::Value,
    pub proposal_version: String,
    pub proposal_body_version: String,
    pub name: Option<String>,
    pub category: Option<String>,
    pub summary: Option<String>,
    pub description: Option<String>,
    pub linked_proposals: Option<serde_json::Value>,
    pub linked_rfp: Option<i32>,
    pub requested_sponsorship_usd_amount: Option<i32>,
    pub requested_sponsorship_paid_in_currency: Option<String>,
    pub requested_sponsor: Option<String>,
    pub receiver_account: Option<String>,
    pub supervisor: Option<String>,
    pub timeline: Option<serde_json::Value>,
    pub views: Option<i32>,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct DumpRecord {
    pub receipt_id: String,
    pub method_name: String,
    pub block_height: BlockHeight,
    pub block_timestamp: i32,
    pub args: String,
    pub author: String,
    pub proposal_id: i32,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct ProposalWithLatestSnapshotView {
    pub proposal_id: i32,
    pub author_id: String,
    pub block_height: Option<BlockHeight>,
    pub ts: Option<Timestamp>,
    pub editor_id: Option<String>,
    pub social_db_post_block_height: Option<BlockHeight>,
    pub labels: Option<serde_json::Value>,
    pub proposal_version: Option<String>,
    pub proposal_body_version: Option<String>,
    pub name: Option<String>,
    pub category: Option<String>,
    pub summary: Option<String>,
    pub description: Option<String>,
    pub linked_proposals: Option<serde_json::Value>,
    pub linked_rfp: Option<i32>,
    pub requested_sponsorship_usd_amount: Option<i32>,
    pub requested_sponsorship_paid_in_currency: Option<String>,
    pub requested_sponsor: Option<String>,
    pub receiver_account: Option<String>,
    pub supervisor: Option<String>,
    pub timeline: Option<serde_json::Value>,
    pub views: Option<i32>,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct RfpRecord {
    pub id: i32,
    pub author_id: String,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct RfpSnapshotRecord {
    pub rfp_id: i32,
    pub block_height: BlockHeight,
    pub ts: Timestamp,
    pub editor_id: String,
    pub social_db_post_block_height: BlockHeight,
    pub labels: serde_json::Value,
    pub linked_proposals: Option<serde_json::Value>,
    pub rfp_version: String,
    pub rfp_body_version: String,
    pub name: Option<String>,
    pub category: Option<String>,
    pub summary: Option<String>,
    pub description: Option<String>,
    pub timeline: Option<serde_json::Value>,
    pub submission_deadline: Timestamp,
    pub views: Option<i32>,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct RfpWithLatestSnapshotView {
    pub rfp_id: i32,
    pub author_id: String,
    pub block_height: BlockHeight,
    pub ts: Timestamp,
    pub editor_id: String,
    pub social_db_post_block_height: BlockHeight,
    pub labels: serde_json::Value,
    pub linked_proposals: Option<serde_json::Value>,
    pub rfp_version: String,
    pub rfp_body_version: String,
    pub name: Option<String>,
    pub category: Option<String>,
    pub summary: Option<String>,
    pub description: Option<String>,
    pub timeline: Option<serde_json::Value>,
    pub views: Option<i32>,
    pub submission_deadline: Timestamp,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct RfpDumpRecord {
    pub receipt_id: String,
    pub method_name: String,
    pub block_height: BlockHeight,
    pub block_timestamp: i32,
    pub args: String,
    pub author: String,
    pub rfp_id: i32,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct SputnikProposalSnapshotRecord {
    pub description: String,
    pub id: String,
    pub proposal_id: i32,
    pub kind: serde_json::Value,
    pub receiver_id: Option<String>,
    pub proposer: String,
    pub status: String,
    pub submission_time: i64,
    pub vote_counts: serde_json::Value,
    pub votes: serde_json::Value,
    pub total_votes: i32,
    pub dao_instance: String,
    pub proposal_action: String,
    pub tx_timestamp: i64,
    pub hash: String,
    pub block_height: i64,
}

#[derive(Debug, Clone)]
pub struct HandlerError {
    pub transaction_id: String,
    pub error_type: String,
    pub message: String,
    pub block_height: i64,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}
