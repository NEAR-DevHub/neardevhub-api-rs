use super::db_types::{ProposalSnapshotRecord, ProposalWithLatestSnapshotView};
use super::DB;
use crate::entrypoints::proposal::proposal_types::GetProposalFilters;
use sqlx::{query, Error, Postgres, Transaction};

impl DB {
    pub async fn upsert_proposal(
        tx: &mut Transaction<'static, Postgres>,
        proposal_id: u32,
        author_id: String,
    ) -> Result<i32, Error> {
        let rec = sqlx::query!(
            r#"
          UPDATE proposals SET author_id = $1 WHERE id = $2
          RETURNING id
          "#,
            author_id,
            proposal_id as i32
        )
        .fetch_optional(tx.as_mut())
        .await?;

        if let Some(record) = rec {
            println!("Updated proposal: {:?}", record.id);
            Ok(record.id)
        } else {
            let rec = sqlx::query!(
                r#"
              INSERT INTO proposals (id, author_id)
              VALUES ($1, $2)
              ON CONFLICT (id) 
              DO UPDATE SET author_id = EXCLUDED.author_id
              RETURNING id
              "#,
                proposal_id as i32,
                author_id
            )
            .fetch_one(tx.as_mut())
            .await?;

            println!("Inserted or updated proposal: {:?}", rec.id);
            Ok(rec.id)
        }
    }

    pub async fn insert_proposal_snapshot(
        tx: &mut Transaction<'static, Postgres>,
        snapshot: &ProposalSnapshotRecord,
    ) -> anyhow::Result<()> {
        // Since primary key is (proposal_id, ts)
        let result = query!(
            r#"
        INSERT INTO proposal_snapshots (
            proposal_id,
            block_height,
            ts,
            editor_id,
            social_db_post_block_height,
            labels,
            proposal_version,
            proposal_body_version,
            name,
            category,
            summary,
            description,
            linked_proposals,
            linked_rfp,
            requested_sponsorship_usd_amount,
            requested_sponsorship_paid_in_currency,
            requested_sponsor,
            receiver_account,
            supervisor,
            timeline,
            views
        ) VALUES (
            $1, $2, $3, $4, $5, $6, $7, $8,
            $9, $10, $11, $12, $13, $14,
            $15, $16, $17, $18, $19, $20, $21
        ) ON CONFLICT (proposal_id, ts) DO UPDATE SET
            block_height = $2,
            editor_id = $4,
            social_db_post_block_height = $5,
            labels = $6,
            proposal_version = $7,
            proposal_body_version = $8,
            name = $9,
            category = $10,
            summary = $11,
            description = $12,
            linked_proposals = $13,
            linked_rfp = $14,
            requested_sponsorship_usd_amount = $15,
            requested_sponsorship_paid_in_currency = $16,
            requested_sponsor = $17,
            receiver_account = $18,
            supervisor = $19,
            timeline = $20,
            views = $21
        "#,
            snapshot.proposal_id,
            snapshot.block_height,
            snapshot.ts,
            snapshot.editor_id,
            snapshot.social_db_post_block_height,
            snapshot.labels,
            snapshot.proposal_version,
            snapshot.proposal_body_version,
            snapshot.name,
            snapshot.category,
            snapshot.summary,
            snapshot.description,
            snapshot.linked_proposals,
            snapshot.linked_rfp,
            snapshot.requested_sponsorship_usd_amount,
            snapshot.requested_sponsorship_paid_in_currency,
            snapshot.requested_sponsor,
            snapshot.receiver_account,
            snapshot.supervisor,
            snapshot.timeline,
            snapshot.views
        )
        .execute(tx.as_mut())
        .await;

        match result {
            Ok(_) => {
                println!(
                    "Inserted proposal snapshot {:?} with name {:?}",
                    snapshot.proposal_id,
                    snapshot.name.as_ref().unwrap()
                );
                Ok(())
            }
            Err(e) => {
                eprintln!("Failed to insert proposal snapshot: {:?}", e);
                Err(anyhow::anyhow!("Failed to insert proposal snapshot"))
            }
        }
    }

    pub async fn get_proposals_with_latest_snapshot(
        &self,
        limit: i64,
        order: &str,
        offset: i64,
        filters: Option<GetProposalFilters>,
    ) -> anyhow::Result<(Vec<ProposalWithLatestSnapshotView>, i64)> {
        // Validate the order clause to prevent SQL injection
        let order_clause = match order.to_lowercase().as_str() {
            "ts_asc" => "ps.ts ASC",
            "ts_desc" => "ps.ts DESC",
            "id_asc" => "ps.proposal_id ASC",
            "id_desc" => "ps.proposal_id DESC",
            _ => "ps.proposal_id DESC",
        };

        let stage = filters.as_ref().and_then(|f| f.stage.as_ref());
        // Set 'stage_clause' to None if 'stage' is None
        let stage_clause: Option<String> = stage.and_then(|s| match s.to_uppercase().as_str() {
            "DRAFT" => Some("DRAFT".to_string()),
            "REVIEW" => Some("REVIEW".to_string()),
            "APPROVED" => Some("APPROVED".to_string()),
            "REJECTED" => Some("REJECTED".to_string()),
            "CANCELLED" => Some("CANCELLED".to_string()),
            "CONDITIONAL" => Some("CONDITIONALLY".to_string()),
            "PAYMENT" => Some("PAYMENT".to_string()),
            "FUNDED" => Some("FUNDED".to_string()),
            _ => None,
        });

        // TODO: add a token_id filter where multiple token_ids are provided
        // TODO: add a receiver_id filter where multiple receiver_ids are provided
        // TODO: add a min_amount and max_amount filter where two amounts are provided
        // TODO: add a approvers filter where multiple approvers are provided
        // Approvers are stored in the votes serde_jsonb column as an array of account ids

        // Convert u128 to strings for the database query
        // let min_amount_str = min_amount.map(|val| val.to_string());
        // let max_amount_str = max_amount.map(|val| val.to_string());
        // let token_range_clause =
        // AND ($2::TEXT IS NULL OR token_amount::NUMERIC >= $2::NUMERIC)
        // AND ($3::TEXT IS NULL OR token_amount::NUMERIC <= $3::NUMERIC)
        // min_amount_str.as_deref(),
        // max_amount_str.as_deref(),

        // Build the SQL query with the validated order clause
        let data_sql = format!(
            r#"
        SELECT
            *
        FROM
            proposals_with_latest_snapshot ps
        WHERE
            ($3 IS NULL OR ps.author_id = $3)
            AND ($4 IS NULL OR ps.ts > $4)
            AND ($5 IS NULL OR ps.timeline::text ~ $5)
            AND ($6 IS NULL OR ps.category = $6)    
            AND ($7 IS NULL OR ps.labels::jsonb ?| $7)
        ORDER BY {}
        LIMIT $1 OFFSET $2
        "#,
            order_clause,
        );

        // Build the count query
        let count_sql = r#"
        SELECT COUNT(*)
        FROM proposals_with_latest_snapshot ps
        WHERE
            ($1 IS NULL OR ps.author_id = $1)
            AND ($2 IS NULL OR ps.ts > $2)
            AND ($3 IS NULL OR ps.timeline::text ~ $3)
            AND ($4 IS NULL OR ps.category = $4)    
            AND ($5 IS NULL OR ps.labels::jsonb ?| $5)
    "#;

        // Extract filter parameters
        let author_id = filters.as_ref().and_then(|f| f.author_id.as_ref());
        let block_timestamp = filters.as_ref().and_then(|f| f.block_timestamp);
        let category = filters.as_ref().and_then(|f| f.category.as_ref());
        let labels = filters.as_ref().and_then(|f| f.labels.as_ref());

        // Execute the data query
        let recs = sqlx::query_as::<_, ProposalWithLatestSnapshotView>(&data_sql)
            .bind(limit)
            .bind(offset)
            .bind(author_id)
            .bind(block_timestamp)
            .bind(stage_clause.clone())
            .bind(category)
            .bind(labels)
            .fetch_all(&self.0)
            .await?;

        // Execute the count query
        let total_count: i64 = sqlx::query_scalar(count_sql)
            .bind(author_id)
            .bind(block_timestamp)
            .bind(stage_clause)
            .bind(category)
            .bind(labels)
            .fetch_one(&self.0)
            .await?;

        Ok((recs, total_count))
    }

    pub async fn search_proposals_with_latest_snapshot(
        &self,
        input: &str,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<(Vec<ProposalWithLatestSnapshotView>, i64)> {
        let sql = r#"
          SELECT
             *
          FROM
              proposals_with_latest_snapshot ps
          WHERE
              to_tsvector('english', coalesce(ps.name, '') || ' ' || coalesce(ps.summary, '') || ' ' || coalesce(ps.description, '')) @@ plainto_tsquery($1)
              OR lower(ps.name) ILIKE $1
              OR lower(ps.summary) ILIKE $1
              OR lower(ps.description) ILIKE $1
          ORDER BY ps.ts DESC
          LIMIT $2 OFFSET $3
      "#;

        let proposals = sqlx::query_as::<_, ProposalWithLatestSnapshotView>(sql)
            .bind(input)
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.0)
            .await?;

        let total_count_sql = r#"
          SELECT
              COUNT(*)
          FROM
              proposals_with_latest_snapshot ps
          WHERE
              to_tsvector('english', coalesce(ps.name, '') || ' ' || coalesce(ps.summary, '') || ' ' || coalesce(ps.description, '')) @@ plainto_tsquery($1)
              OR lower(ps.name) ILIKE $1
              OR lower(ps.summary) ILIKE $1
              OR lower(ps.description) ILIKE $1
      "#;

        let total_count = sqlx::query_scalar::<_, i64>(total_count_sql)
            .bind(input)
            .fetch_one(&self.0)
            .await?;

        Ok((proposals, total_count))
    }

    pub async fn get_proposal_with_latest_snapshot_by_id(
        &self,
        id: i32,
    ) -> anyhow::Result<ProposalWithLatestSnapshotView> {
        println!("Getting proposal with latest snapshot by id: {:?}", id);
        let sql = r#"
            SELECT
               *
            FROM
               proposals_with_latest_snapshot ps
            WHERE
                ps.proposal_id = $1
        "#;
        // Start Generation Here
        let proposal = sqlx::query_as::<_, ProposalWithLatestSnapshotView>(sql)
            .bind(id)
            .fetch_one(&self.0)
            .await?;

        Ok(proposal)
    }

    // TODO Remove this once we go in production or put it behind authentication or a flag
    pub async fn remove_proposal_snapshots_by_id(&self, proposal_id: i32) -> anyhow::Result<()> {
        sqlx::query!(
            r#"DELETE FROM proposal_snapshots WHERE proposal_id = $1"#,
            proposal_id
        )
        .execute(&self.0)
        .await?;
        Ok(())
    }

    pub async fn get_proposal_with_all_snapshots(
        &self,
        id: i32,
    ) -> anyhow::Result<Vec<ProposalSnapshotRecord>> {
        // Group by ts
        // Build the SQL query for fetching data with the validated order clause
        let data_sql = r#"
      SELECT
          proposal.*
      FROM  
          proposal_snapshots proposal
      WHERE
         proposal.proposal_id = $1
      ORDER BY
          proposal.ts DESC
      "#;

        // Execute the data query
        let result = sqlx::query_as::<_, ProposalSnapshotRecord>(data_sql)
            .bind(id)
            .fetch_all(&self.0)
            .await;

        match result {
            Ok(recs) => Ok(recs),
            Err(e) => {
                eprintln!("Failed to get proposal with all snapshots: {:?}", e);
                Err(anyhow::anyhow!("Failed to get proposal with all snapshots"))
            }
        }
    }

    pub async fn get_proposal_with_latest_snapshot_view(
        &self,
        proposal_id: i32,
    ) -> Result<Option<ProposalWithLatestSnapshotView>, sqlx::Error> {
        let sql = r#"
        SELECT *
        FROM proposals_with_latest_snapshot
        WHERE proposal_id = $1
      "#;
        let proposal = sqlx::query_as::<_, ProposalWithLatestSnapshotView>(sql)
            .bind(proposal_id)
            .fetch_optional(&self.0)
            .await?;

        Ok(proposal)
    }
}
