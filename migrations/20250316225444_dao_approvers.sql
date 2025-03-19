-- Add migration script here
CREATE TABLE IF NOT EXISTS dao_approvers (
    id SERIAL PRIMARY KEY,
    dao_instance VARCHAR NOT NULL,
    approver VARCHAR NOT NULL,
    UNIQUE(dao_instance, approver)
);

CREATE INDEX idx_dao_approvers_dao_instance ON dao_approvers(dao_instance);