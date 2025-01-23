-- Add migration script here

CREATE TABLE handler_errors (
    id BIGSERIAL PRIMARY KEY,
    transaction_id TEXT NOT NULL,
    error_type TEXT NOT NULL,
    message TEXT NOT NULL,
    block_height BIGINT NOT NULL,
    timestamp TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create an index on transaction_id for faster lookups
CREATE INDEX handler_errors_transaction_id_idx ON handler_errors(transaction_id);

-- Create an index on block_height for range queries
CREATE INDEX handler_errors_block_height_idx ON handler_errors(block_height);

-- Create an index on timestamp for time-based queries
CREATE INDEX handler_errors_timestamp_idx ON handler_errors(timestamp);

