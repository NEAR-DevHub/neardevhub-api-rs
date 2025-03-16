use near_sdk::borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::json_types::Base58CryptoHash;
use near_sdk::json_types::{U128, U64};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::AccountId;
use std::collections::{HashMap, HashSet};
pub type OldAccountId = String;
use near_sdk::json_types::Base64VecU8;
use std::fmt;

use super::policy::VersionedPolicy;

pub type Balance = u128;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct ProposalOutput {
    pub id: u64,
    #[serde(flatten)]
    pub proposal: Proposal,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug))]
#[serde(crate = "near_sdk::serde")]
pub struct Proposal {
    pub proposer: AccountId,
    pub description: String,
    pub kind: ProposalKind,
    pub status: ProposalStatus,
    pub vote_counts: HashMap<String, [u64; 3]>,
    pub votes: HashMap<AccountId, Vote>,
    pub submission_time: U64,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Config {
    pub name: String,
    pub purpose: String,
    pub metadata: Base64VecU8,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Clone, Debug))]
#[serde(crate = "near_sdk::serde")]
pub enum ProposalKind {
    ChangeConfig {
        config: Config,
    },
    ChangePolicy {
        policy: VersionedPolicy,
    },
    AddMemberToRole {
        member_id: AccountId,
        role: String,
    },
    RemoveMemberFromRole {
        member_id: AccountId,
        role: String,
    },
    FunctionCall {
        receiver_id: AccountId,
        actions: Vec<ActionCall>,
    },
    UpgradeSelf {
        hash: Base58CryptoHash,
    },
    UpgradeRemote {
        receiver_id: AccountId,
        method_name: String,
        hash: Base58CryptoHash,
    },
    Transfer {
        token_id: OldAccountId,
        receiver_id: AccountId,
        amount: U128,
        msg: Option<String>,
    },
    SetStakingContract {
        staking_id: AccountId,
    },
    AddBounty {
        bounty: Bounty,
    },
    BountyDone {
        bounty_id: u64,
        receiver_id: AccountId,
    },
    Vote,
    FactoryInfoUpdate {
        factory_info: FactoryInfo,
    },
    ChangePolicyAddOrUpdateRole {
        role: RolePermission,
    },
    ChangePolicyRemoveRole {
        role: String,
    },
    ChangePolicyUpdateDefaultVotePolicy {
        vote_policy: VotePolicy,
    },
    ChangePolicyUpdateParameters {
        parameters: PolicyParameters,
    },
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Clone, Debug))]
#[serde(crate = "near_sdk::serde")]
pub struct ActionCall {
    method_name: String,
    args: Base64VecU8,
    deposit: U128,
    gas: U64,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug))]
#[serde(crate = "near_sdk::serde")]
pub struct Bounty {
    pub description: String,
    pub token: OldAccountId,
    pub amount: U128,
    pub times: u32,
    pub max_deadline: U64,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug, PartialEq))]
#[serde(crate = "near_sdk::serde")]
pub struct VotePolicy {
    pub weight_kind: WeightKind,
    pub quorum: U128,
    pub threshold: WeightOrRatio,
}

impl Default for VotePolicy {
    fn default() -> Self {
        VotePolicy {
            weight_kind: WeightKind::RoleWeight,
            quorum: U128(0),
            threshold: WeightOrRatio::Ratio(1, 2),
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug, PartialEq))]
#[serde(crate = "near_sdk::serde")]
#[serde(untagged)]
pub enum WeightOrRatio {
    Weight(U128),
    Ratio(u64, u64),
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug))]
#[serde(crate = "near_sdk::serde")]
pub enum WeightKind {
    TokenWeight,
    RoleWeight,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Clone, Debug))]
#[serde(crate = "near_sdk::serde")]
pub struct FactoryInfo {
    pub factory_id: AccountId,
    pub auto_update: bool,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug, PartialEq))]
#[serde(crate = "near_sdk::serde")]
pub struct RolePermission {
    pub name: String,
    pub kind: RoleKind,
    pub permissions: HashSet<String>,
    pub vote_policy: HashMap<String, VotePolicy>,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug, PartialEq))]
#[serde(crate = "near_sdk::serde")]
pub enum RoleKind {
    Everyone,
    Member(U128),
    Group(HashSet<AccountId>),
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(crate = "near_sdk::serde")]
pub enum ProposalStatus {
    InProgress,
    Approved,
    Rejected,
    Removed,
    Expired,
    Moved,
    Failed,
}

impl fmt::Display for ProposalStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProposalStatus::InProgress => write!(f, "InProgress"),
            ProposalStatus::Approved => write!(f, "Approved"),
            ProposalStatus::Rejected => write!(f, "Rejected"),
            ProposalStatus::Removed => write!(f, "Removed"),
            ProposalStatus::Expired => write!(f, "Expired"),
            ProposalStatus::Moved => write!(f, "Moved"),
            ProposalStatus::Failed => write!(f, "Failed"),
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
#[borsh(use_discriminant = true)]
pub enum Vote {
    Approve = 0x0,
    Reject = 0x1,
    Remove = 0x2,
}

impl From<Action> for Vote {
    fn from(action: Action) -> Self {
        match action {
            Action::VoteApprove => Vote::Approve,
            Action::VoteReject => Vote::Reject,
            Action::VoteRemove => Vote::Remove,
            _ => unreachable!(),
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub enum Action {
    AddProposal,
    RemoveProposal,
    VoteApprove,
    VoteReject,
    VoteRemove,
    Finalize,
    MoveToHub,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Clone, Debug))]
#[serde(crate = "near_sdk::serde")]
pub struct PolicyParameters {
    pub proposal_bond: Option<U128>,
    pub proposal_period: Option<U64>,
    pub bounty_bond: Option<U128>,
    pub bounty_forgiveness_period: Option<U64>,
}
