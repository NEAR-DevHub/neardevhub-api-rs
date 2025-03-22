use super::db_types::{BlockHeight, LastUpdatedInfo, SputnikProposalSnapshotRecord};
use super::DB;
use crate::entrypoints::sputnik::GetDaoProposalsFilters;
use near_sdk::AccountId;
use rocket::http::Status;
use sqlx::{query, Error, Postgres, Transaction};

impl DB {
    pub async fn set_last_updated_info_for_contract(
        &self,
        contract: &AccountId,
        after_date: i64,
        after_block: BlockHeight,
    ) -> Result<(), Error> {
        sqlx::query!(
            r#"
            INSERT INTO dao_instances_last_updated_info (instance, after_date, after_block)
            VALUES ($1, $2, $3)
            ON CONFLICT (instance) DO UPDATE SET
              after_date = $2,
              after_block = $3
            "#,
            contract.to_string(),
            after_date,
            after_block,
        )
        .execute(&self.0)
        .await?;
        Ok(())
    }

    pub async fn get_last_updated_info_for_contract(
        &self,
        contract: &AccountId,
    ) -> Result<LastUpdatedInfo, Error> {
        let rec = query!(
            r#"
          SELECT after_date, after_block FROM dao_instances_last_updated_info
          WHERE instance = $1
          "#,
            contract.to_string()
        )
        .fetch_optional(&self.0)
        .await?;

        if let Some(rec) = rec {
            Ok(LastUpdatedInfo {
                after_date: rec.after_date,
                after_block: rec.after_block,
                cursor: "".to_string(),
            })
        } else {
            Ok(LastUpdatedInfo {
                after_date: 0,
                after_block: 0,
                cursor: "".to_string(),
            })
        }
    }

    // TODO Once we go in production put this behind authentication or a flag
    pub async fn remove_all_dao_proposals(&self, account_id: &str) -> anyhow::Result<()> {
        sqlx::query!(
            r#"DELETE FROM dao_proposals WHERE dao_instance = $1"#,
            account_id
        )
        .execute(&self.0)
        .await?;

        Ok(())
    }

    pub async fn upsert_dao_proposal_snapshot(
        tx: &mut Transaction<'static, Postgres>,
        sputnik_proposal: SputnikProposalSnapshotRecord,
    ) -> anyhow::Result<()> {
        let update_result = sqlx::query!(
            r#"
            UPDATE dao_proposals SET
                id = $1,
                proposal_id = $2,
                description = $3,
                kind = $4,
                proposer = $5,
                status = $6,
                submission_time = $7,
                vote_counts = $8,
                votes = $9,
                total_votes = $10,
                dao_instance = $11,
                proposal_action = $12,
                tx_timestamp = $13,
                hash = $14,
                block_height = $15,
                receiver_id = $16,
                token_id = $17,
                token_amount = $18
            WHERE id = $1
            RETURNING id
            "#,
            sputnik_proposal.id,
            sputnik_proposal.proposal_id as i32,
            sputnik_proposal.description,
            sputnik_proposal.kind,
            sputnik_proposal.proposer,
            sputnik_proposal.status,
            sputnik_proposal.submission_time,
            sputnik_proposal.vote_counts,
            sputnik_proposal.votes,
            sputnik_proposal.total_votes as i32,
            sputnik_proposal.dao_instance,
            sputnik_proposal.proposal_action,
            sputnik_proposal.tx_timestamp,
            sputnik_proposal.hash,
            sputnik_proposal.block_height,
            sputnik_proposal.receiver_id,
            sputnik_proposal.token_id,
            sputnik_proposal.token_amount
        )
        .fetch_optional(tx.as_mut())
        .await?;

        if let Some(record) = update_result {
            println!("Updated dao proposal on id: {:?}", record.id);
            Ok(())
        } else {
            println!("Inserting id: {:?}", sputnik_proposal.id);
            let rec = sqlx::query!(
                r#"
                INSERT INTO dao_proposals (
                    description, id, proposal_id, kind, proposer, status, submission_time, vote_counts, votes, total_votes, dao_instance, proposal_action, tx_timestamp, hash, block_height, receiver_id, token_id, token_amount
                ) VALUES (
                    $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18
                )
                ON CONFLICT (id) DO NOTHING
                RETURNING id
                "#,
                sputnik_proposal.description,
                sputnik_proposal.id,
                sputnik_proposal.proposal_id as i32,
                sputnik_proposal.kind,
                sputnik_proposal.proposer,
                sputnik_proposal.status,
                sputnik_proposal.submission_time,
                sputnik_proposal.vote_counts,
                sputnik_proposal.votes,
                sputnik_proposal.total_votes as i32,
                sputnik_proposal.dao_instance,
                sputnik_proposal.proposal_action,
                sputnik_proposal.tx_timestamp,
                sputnik_proposal.hash,
                sputnik_proposal.block_height,
                sputnik_proposal.receiver_id,
                sputnik_proposal.token_id,
                sputnik_proposal.token_amount
            )
            .fetch_optional(tx.as_mut())
            .await;

            match rec {
                Ok(Some(record)) => {
                    println!("Inserted dao proposal snapshot: {:?}", record.id);
                }
                Ok(None) => {
                    println!("No record inserted due to conflict or other issue.");
                    eprintln!("No record inserted due to conflict or other issue.");
                }
                Err(e) => {
                    eprintln!("Error inserting dao proposal snapshot: {:?}", e);
                    return Err(anyhow::anyhow!("Failed to insert dao proposal snapshot"));
                }
            }
            Ok(())
        }
    }

    pub async fn get_dao_proposals(
        &self,
        dao_instance: &str,
        limit: i64,
        order: &str,
        offset: i64,
        filters: Option<GetDaoProposalsFilters>,
    ) -> anyhow::Result<(Vec<SputnikProposalSnapshotRecord>, i64)> {
        let order_clause = match order.to_lowercase().as_str() {
            "ts_asc" => "submission_time ASC",
            "ts_desc" => "submission_time DESC",
            "id_asc" => "proposal_id ASC",
            "id_desc" => "proposal_id DESC",
            _ => "proposal_id DESC",
        };

        let kind = filters.as_ref().and_then(|f| f.kind.as_ref());
        let total_votes = filters.as_ref().and_then(|f| f.total_votes.as_ref());
        let status = filters.as_ref().and_then(|f| f.status.as_ref());
        let proposer = filters.as_ref().and_then(|f| f.proposer.as_ref());
        let from_amount = filters.as_ref().and_then(|f| f.from_amount.as_ref());
        let to_amount = filters.as_ref().and_then(|f| f.to_amount.as_ref());
        let recipient_id = filters.as_ref().and_then(|f| f.recipient_id.as_ref());
        let requested_token_ids = filters.as_ref().and_then(|f| f.requested_token_id.as_ref());
        let approvers = filters.as_ref().and_then(|f| f.approvers.as_ref());

        let sql = format!(
            r#"
          SELECT *
          FROM dao_proposals
          WHERE dao_instance = $1
          AND ($2 IS NULL OR substring(kind::text, 1, 40) ILIKE '%' || $2 || '%')
          AND ($3 IS NULL OR status::text = $3)
          AND ($4 IS NULL OR total_votes = $4)
          AND ($5 IS NULL OR proposer::text ILIKE '%' || $5 || '%')
          AND ($6 IS NULL OR CASE WHEN token_amount ~ '^[0-9]+$' THEN token_amount::numeric >= $6::numeric ELSE false END)
          AND ($7 IS NULL OR CASE WHEN token_amount ~ '^[0-9]+$' THEN token_amount::numeric <= $7::numeric ELSE false END)
          AND ($8 IS NULL OR receiver_id::text ILIKE '%' || $8 || '%')
          AND ($9 IS NULL OR token_id::text = ANY($9))
          AND ($10 IS NULL OR (
              SELECT EXISTS (
                  SELECT 1
                  FROM jsonb_object_keys(votes::jsonb) AS voter
                  WHERE voter = ANY($10)
              )
          ))
          ORDER BY {}
          LIMIT $11 OFFSET $12
        "#,
            order_clause,
        );

        let proposals = sqlx::query_as::<_, SputnikProposalSnapshotRecord>(&sql)
            .bind(dao_instance)
            .bind(kind)
            .bind(status)
            .bind(total_votes)
            .bind(proposer)
            .bind(from_amount)
            .bind(to_amount)
            .bind(recipient_id)
            .bind(requested_token_ids)
            .bind(approvers)
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.0)
            .await?;

        let count_sql = r#"
            SELECT COUNT(*)
            FROM dao_proposals
            WHERE dao_instance = $1
            AND ($2 IS NULL OR substring(kind::text, 1, 40) ILIKE '%' || $2 || '%')
            AND ($3 IS NULL OR status::text = $3)
            AND ($4 IS NULL OR total_votes = $4)
            AND ($5 IS NULL OR proposer::text ILIKE '%' || $5 || '%')
            AND ($6 IS NULL OR CASE WHEN token_amount ~ '^[0-9]+$' THEN token_amount::numeric >= $6::numeric ELSE false END)
            AND ($7 IS NULL OR CASE WHEN token_amount ~ '^[0-9]+$' THEN token_amount::numeric <= $7::numeric ELSE false END)
            AND ($8 IS NULL OR receiver_id::text ILIKE '%' || $8 || '%')
            AND ($9 IS NULL OR token_id::text = ANY($9))
            AND ($10 IS NULL OR (
              SELECT EXISTS (
                  SELECT 1
                  FROM jsonb_object_keys(votes::jsonb) AS voter
                  WHERE voter = ANY($10)
              )
          ))
        "#;

        let total_count = sqlx::query_scalar::<_, i64>(count_sql)
            .bind(dao_instance)
            .bind(kind)
            .bind(status)
            .bind(total_votes)
            .bind(proposer)
            .bind(from_amount)
            .bind(to_amount)
            .bind(recipient_id)
            .bind(requested_token_ids)
            .bind(approvers)
            .fetch_one(&self.0)
            .await?;

        Ok((proposals, total_count))
    }

    pub async fn get_dao_proposal_by_hash(
        &self,
        hash: &str,
        dao_instance: &str,
    ) -> Result<SputnikProposalSnapshotRecord, sqlx::Error> {
        sqlx::query_as!(
            SputnikProposalSnapshotRecord,
            r#"
            SELECT *
            FROM dao_proposals
            WHERE hash = $1
            AND dao_instance = $2
            ORDER BY submission_time DESC
            LIMIT 1
            "#,
            hash,
            dao_instance,
        )
        .fetch_one(&self.0)
        .await
    }

    pub async fn search_dao_proposals(
        &self,
        search_term: &str,
        dao_instance: &str,
    ) -> Result<(Vec<SputnikProposalSnapshotRecord>, i64), sqlx::Error> {
        // First get the total count
        let total = sqlx::query_scalar!(
            r#"
            SELECT COUNT(DISTINCT id)
            FROM dao_proposals
            WHERE 
                dao_instance = $1 AND
                (LOWER(description) LIKE $2 OR
                LOWER(hash) LIKE $2)
            "#,
            dao_instance,
            search_term,
        )
        .fetch_one(&self.0)
        .await?;

        // Then get the actual records
        let proposals = sqlx::query_as!(
            SputnikProposalSnapshotRecord,
            r#"
            WITH latest_snapshots AS (
                SELECT DISTINCT ON (id) *
                FROM dao_proposals
                WHERE 
                    LOWER(description) LIKE $1 OR
                    LOWER(hash) LIKE $1
                ORDER BY id, submission_time DESC
            )
            SELECT *
            FROM latest_snapshots
            ORDER BY id DESC
            LIMIT 10
            "#,
            search_term,
        )
        .fetch_all(&self.0)
        .await?;

        Ok((proposals, total.unwrap_or(0)))
    }

    pub async fn get_unique_receiver_ids(
        &self,
        dao_instance: &str,
    ) -> Result<Vec<String>, sqlx::Error> {
        // Query to get all unique receiver_ids for a given contract, excluding nulls and empty strings
        let receiver_ids = sqlx::query_scalar!(
            r#"
            SELECT DISTINCT receiver_id
            FROM dao_proposals
            WHERE 
                dao_instance = $1
                AND receiver_id IS NOT NULL
                AND receiver_id != ''
            ORDER BY receiver_id
            "#,
            dao_instance
        )
        .fetch_all(&self.0)
        .await?;

        // Filter out None values and unwrap the Some values
        let receiver_ids: Vec<String> = receiver_ids.into_iter().flatten().collect();

        Ok(receiver_ids)
    }

    pub async fn get_dao_token_ids(&self, dao_instance: &str) -> Result<Vec<String>, sqlx::Error> {
        let token_ids = sqlx::query_scalar!(
            r#"
            SELECT DISTINCT token_id
            FROM dao_proposals
            WHERE dao_instance = $1
            AND token_id IS NOT NULL
            AND token_id != ''
            ORDER BY token_id
            "#,
            dao_instance
        )
        .fetch_all(&self.0)
        .await?;

        // Filter out None values and unwrap the Some values
        let token_ids: Vec<String> = token_ids.into_iter().flatten().collect();

        Ok(token_ids)
    }

    pub async fn get_dao_approvers(&self, dao_instance: &str) -> Result<Vec<String>, sqlx::Error> {
        let approvers = sqlx::query_scalar!(
            r#"
          SELECT approver
          FROM dao_approvers
          WHERE dao_instance = $1
          ORDER BY approver
          "#,
            dao_instance
        )
        .fetch_all(&self.0)
        .await?;

        Ok(approvers)
    }

    pub async fn upsert_dao_approvers(
        tx: &mut Transaction<'static, Postgres>,
        dao_instance: &str,
        approvers: &[String],
    ) -> Result<(), sqlx::Error> {
        for approver in approvers {
            sqlx::query!(
                r#"
                  INSERT INTO dao_approvers (dao_instance, approver)
                  VALUES ($1, $2)
                  ON CONFLICT (dao_instance, approver) DO NOTHING
                  "#,
                dao_instance,
                approver
            )
            .execute(tx.as_mut())
            .await?;
        }
        Ok(())
    }

    pub async fn update_proposal_status(&self, id: String, status: &str) -> Result<(), Status> {
        sqlx::query_scalar!(
            "UPDATE dao_proposals SET status = $1 WHERE id = $2 RETURNING id",
            status,
            id,
        )
        .fetch_one(&self.0)
        .await
        .map_err(|e| {
            eprintln!("Failed to update proposal status: {:?}", e);
            Status::InternalServerError
        })?;

        Ok(())
    }
}
