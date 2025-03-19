-- Add migration script here

-- Add receiver_id column to dao_proposals table
ALTER TABLE dao_proposals
ADD COLUMN receiver_id VARCHAR NULL;

-- Create an index for the new column
CREATE INDEX idx_dao_proposals_receiver_id ON dao_proposals (receiver_id);
