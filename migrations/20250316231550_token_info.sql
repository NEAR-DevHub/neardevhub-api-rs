-- Add migration script here

-- Add token_id and token_amount columns to dao_proposals table
ALTER TABLE dao_proposals
ADD COLUMN token_id VARCHAR NULL,
ADD COLUMN token_amount NUMERIC(39, 0) NULL; -- Using NUMERIC(20, 0) to store u64 values safely

-- Create indexes for better query performance
CREATE INDEX idx_dao_proposals_token_id ON dao_proposals(token_id);

-- Add a B-tree index specifically for range queries on token_amount
CREATE INDEX idx_dao_proposals_token_amount ON dao_proposals(token_amount);