-- Add migration script here

ALTER TABLE dao_proposals
ADD COLUMN block_height BIGINT NOT NULL DEFAULT 0;

-- Remove the default constraint after adding the column
ALTER TABLE dao_proposals
ALTER COLUMN block_height DROP DEFAULT;

-- Add an index for the new column
CREATE INDEX idx_dao_proposals_block_height ON dao_proposals (block_height);
