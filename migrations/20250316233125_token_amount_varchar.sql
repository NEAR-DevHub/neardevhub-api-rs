-- Add migration script here

-- Change token_amount column from NUMERIC to VARCHAR
ALTER TABLE dao_proposals 
ALTER COLUMN token_amount TYPE VARCHAR USING token_amount::VARCHAR;

-- Recreate the index for token_amount
DROP INDEX IF EXISTS idx_dao_proposals_token_amount;
CREATE INDEX idx_dao_proposals_token_amount ON dao_proposals(token_amount);

-- Add a functional index for numeric range queries
CREATE INDEX idx_dao_proposals_token_amount_numeric ON dao_proposals((token_amount::NUMERIC(39,0))) 
WHERE token_amount IS NOT NULL AND token_amount ~ '^[0-9]+$';
