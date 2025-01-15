use super::sputnik_types::{Balance, RolePermission, VotePolicy};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::{U128, U64};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::AccountId;

pub struct UserInfo {
    pub account_id: AccountId,
    pub amount: Balance,
}
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug, PartialEq))]
#[serde(crate = "near_sdk::serde")]
pub struct Policy {
    pub roles: Vec<RolePermission>,
    pub default_vote_policy: VotePolicy,
    pub proposal_bond: U128,
    pub proposal_period: U64,
    pub bounty_bond: U128,
    pub bounty_forgiveness_period: U64,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug, PartialEq))]
#[serde(crate = "near_sdk::serde", untagged)]
pub enum VersionedPolicy {
    Default(Vec<AccountId>),
    Current(Policy),
}
