-- Add migration script here

-- Add approvers column to dao_proposals table
ALTER TABLE dao_proposals
ADD COLUMN approvers TEXT[] NULL;

-- Create an index for the new column
CREATE INDEX idx_dao_proposals_approvers ON dao_proposals USING GIN (approvers);

UPDATE dao_proposals
SET approvers = (
    SELECT ARRAY(
        SELECT jsonb_object_keys(votes::jsonb)
    )
);