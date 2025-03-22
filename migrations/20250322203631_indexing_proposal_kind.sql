-- Add migration script here
-- index row size 5696 exceeds btree version 4 maximum 2704 for index "idx_dao_proposals_kind"

-- Drop the existing index
DROP INDEX idx_dao_proposals_kind;
   
-- Create an index on just the first N characters
CREATE INDEX idx_dao_proposals_kind_prefix ON dao_proposals (substring(kind::text, 1, 40));