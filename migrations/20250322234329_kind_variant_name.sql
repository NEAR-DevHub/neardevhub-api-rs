-- Add migration script here

-- Drop last index
DROP INDEX idx_dao_proposals_kind_prefix;

-- Add kind_variant_name column
ALTER TABLE dao_proposals ADD COLUMN kind_variant_name VARCHAR NOT NULL DEFAULT 'Unknown';

-- Create index on the new column
CREATE INDEX idx_dao_proposals_kind_variant_name ON dao_proposals(kind_variant_name);

-- Handle different possible structures of the kind field
UPDATE dao_proposals
SET kind_variant_name = CASE
    -- If kind is a string that begins with a recognizable variant name, extract it
    WHEN (kind#>>'{}') LIKE '%ChangeConfig%' THEN 'ChangeConfig'
    WHEN (kind#>>'{}') LIKE '%ChangePolicy%' THEN 'ChangePolicy'
    WHEN (kind#>>'{}') LIKE '%AddMemberToRole%' THEN 'AddMemberToRole'
    WHEN (kind#>>'{}') LIKE '%RemoveMemberFromRole%' THEN 'RemoveMemberFromRole'
    WHEN (kind#>>'{}') LIKE '%FunctionCall%' THEN 'FunctionCall'
    WHEN (kind#>>'{}') LIKE '%UpgradeSelf%' THEN 'UpgradeSelf'
    WHEN (kind#>>'{}') LIKE '%UpgradeRemote%' THEN 'UpgradeRemote'
    WHEN (kind#>>'{}') LIKE '%Transfer%' THEN 'Transfer'
    WHEN (kind#>>'{}') LIKE '%SetStakingContract%' THEN 'SetStakingContract'
    WHEN (kind#>>'{}') LIKE '%AddBounty%' THEN 'AddBounty'
    WHEN (kind#>>'{}') LIKE '%BountyDone%' THEN 'BountyDone'
    WHEN (kind#>>'{}') LIKE '%Vote%' THEN 'Vote'
    WHEN (kind#>>'{}') LIKE '%FactoryInfoUpdate%' THEN 'FactoryInfoUpdate'
    WHEN (kind#>>'{}') LIKE '%ChangePolicyAddOrUpdateRole%' THEN 'ChangePolicyAddOrUpdateRole'
    WHEN (kind#>>'{}') LIKE '%ChangePolicyRemoveRole%' THEN 'ChangePolicyRemoveRole'
    WHEN (kind#>>'{}') LIKE '%ChangePolicyUpdateDefaultVotePolicy%' THEN 'ChangePolicyUpdateDefaultVotePolicy'
    WHEN (kind#>>'{}') LIKE '%ChangePolicyUpdateParameters%' THEN 'ChangePolicyUpdateParameters'
    -- Default to the first 30 chars if no match is found
    ELSE SUBSTRING((kind#>>'{}'), 1, 30)
END;