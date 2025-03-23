-- Add migration script here


CREATE TABLE IF NOT EXISTS
  dao_proposals (
    description VARCHAR NOT NULL,
    id VARCHAR NOT NULL UNIQUE PRIMARY KEY,
    proposal_id INT NOT NULL,
    kind jsonb NOT NULL,
    proposer VARCHAR NOT NULL,
    status VARCHAR NOT NULL,
    submission_time BIGINT NOT NULL,
    vote_counts JSONB NOT NULL,
    votes JSONB NOT NULL,
    total_votes INT NOT NULL,
    dao_instance VARCHAR NOT NULL,
    proposal_action VARCHAR NOT NULL,
    tx_timestamp BIGINT NOT NULL,
    hash VARCHAR NOT NULL -- Link to the transaction that created the proposal?
  );

CREATE UNIQUE INDEX idx_dao_proposals_unique_proposal_instance ON dao_proposals (proposal_id, dao_instance);
CREATE INDEX idx_dao_proposals_description ON dao_proposals (description);
CREATE INDEX idx_dao_proposals_proposal_id ON dao_proposals (proposal_id);
CREATE INDEX idx_dao_proposals_kind ON dao_proposals (kind);
CREATE INDEX idx_dao_proposals_proposer ON dao_proposals (proposer);
CREATE INDEX idx_dao_proposals_status ON dao_proposals (status);
CREATE INDEX idx_dao_proposals_submission_time ON dao_proposals (submission_time);
CREATE INDEX idx_dao_proposals_vote_counts ON dao_proposals (vote_counts);
CREATE INDEX idx_dao_proposals_votes ON dao_proposals (votes);
CREATE INDEX idx_dao_proposals_total_votes ON dao_proposals (total_votes);
CREATE INDEX idx_dao_proposals_dao_instance ON dao_proposals (dao_instance);
CREATE INDEX idx_dao_proposals_proposal_action ON dao_proposals (proposal_action);

CREATE TABLE IF NOT EXISTS
  dao_instances_last_updated_info (
    instance VARCHAR NOT NULL PRIMARY KEY,
    after_date BIGINT NOT NULL,
    after_block BIGINT NOT NULL
  );
