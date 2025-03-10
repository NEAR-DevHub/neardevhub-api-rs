use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::db::db_types::ProposalWithLatestSnapshotView;

pub type Contract = String;

#[derive(Clone, Debug, Serialize, Deserialize, Default, ToSchema)]
#[aliases(PaginatedProposalResponse = PaginatedResponse<ProposalWithLatestSnapshotView>)]
pub struct PaginatedResponse<T: Serialize> {
    pub records: Vec<T>,
    pub page: u64,
    pub total_pages: u64,
    pub limit: u64,
    pub total_records: u64,
    pub newly_indexed: usize,
}

impl<T: Serialize> PaginatedResponse<T> {
    pub fn new(
        records: Vec<T>,
        page: u64,
        limit: u64,
        total_records: u64,
        newly_indexed: Option<usize>,
    ) -> Self {
        let extra_page = if total_records % limit == 0 { 0 } else { 1 };
        let total_pages = (total_records / limit) + extra_page;
        Self {
            records,
            page,
            total_pages,
            limit,
            total_records,
            newly_indexed: newly_indexed.unwrap_or(0),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct ProposalResponse {
    pub id: i32,
    pub author_id: String,
    // Latest Snapshot
    pub proposal_id: i32,
    pub block_height: i64,
    pub ts: i32,
    pub editor_id: String,
    pub social_db_post_block_height: i64,
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
