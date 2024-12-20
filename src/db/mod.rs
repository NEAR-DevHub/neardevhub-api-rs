use crate::{
    entrypoints::{
        proposal::proposal_types::GetProposalFilters,
        rfp::rfp_types::GetRfpFilters,
        sputnik::{GetDaoProposalsFilters, GetTxnsFilters},
    },
    timestamp_to_date_string,
};
use near_sdk::AccountId;
use rocket::{
    fairing::{self, AdHoc},
    Build, Rocket,
};
use rocket_db_pools::Database;
use sqlx::{migrate, query, Error, PgPool, Postgres, Transaction};

#[derive(Database, Clone, Debug)]
#[database("my_db")]
pub struct DB(PgPool);

pub mod db_types;

use db_types::{
    BlockHeight, LastUpdatedInfo, ProposalSnapshotRecord, ProposalWithLatestSnapshotView,
    RfpSnapshotRecord, RfpWithLatestSnapshotView, SputnikProposalSnapshotRecord, SputnikTxnsRecord,
};

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

        // If the update did not find a matching row, insert the user
        if let Some(record) = rec {
            println!("Updated proposal: {:?}", record.id);
            Ok(record.id)
        } else {
            // INSERT ON CONFLICT DO NOTHING
            let rec = sqlx::query!(
                r#"
                INSERT INTO proposals (id, author_id)
                VALUES ($1, $2)
                ON CONFLICT (id) DO NOTHING
                RETURNING id
                "#,
                proposal_id as i32,
                author_id
            )
            .fetch_one(tx.as_mut())
            .await?;
            println!("Inserted proposal: {:?}", rec.id);
            Ok(rec.id)
        }
    }

    pub async fn get_last_updated_info(&self) -> Result<LastUpdatedInfo, Error> {
        let rec = query!(
            r#"
            SELECT after_date, after_block, cursor FROM last_updated_info
            "#
        )
        .fetch_one(&self.0)
        .await?;
        Ok(LastUpdatedInfo {
            after_date: rec.after_date,
            after_block: rec.after_block,
            cursor: rec.cursor,
        })
    }

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

    pub async fn set_last_updated_info(
        &self,
        after_date: i64,
        after_block: BlockHeight,
        cursor: String,
    ) -> Result<(), Error> {
        println!(
            "Storing timestamp: {} and block: {} and cursor: {}",
            after_date, after_block, cursor
        );
        println!("Storing date: {}", timestamp_to_date_string(after_date));
        sqlx::query!(
            r#"
            UPDATE last_updated_info SET after_date = $1, after_block = $2, cursor = $3
            "#,
            after_date,
            after_block,
            cursor
        )
        .execute(&self.0)
        .await?;
        Ok(())
    }

    pub async fn set_last_updated_timestamp(&self, after_date: i64) -> Result<(), Error> {
        println!("Storing timestamp: {}", after_date);
        println!("Storing date: {}", timestamp_to_date_string(after_date));
        sqlx::query!(
            r#"
          UPDATE last_updated_info SET after_date = $1
          "#,
            after_date,
        )
        .execute(&self.0)
        .await?;
        Ok(())
    }

    pub async fn set_last_updated_block(&self, after_block: BlockHeight) -> Result<(), Error> {
        println!("Storing block: {}", after_block);
        sqlx::query!(
            r#"
          UPDATE last_updated_info SET after_block = $1
          "#,
            after_block,
        )
        .execute(&self.0)
        .await?;
        Ok(())
    }

    pub async fn set_last_updated_cursor(&self, cursor: String) -> Result<(), Error> {
        println!("Storing cursor: {}", cursor);
        sqlx::query!(
            r#"
          UPDATE last_updated_info SET cursor = $1
          "#,
            cursor,
        )
        .execute(&self.0)
        .await?;
        Ok(())
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
            _ => "ps.proposal_id DESC", // Default to DESC if the order is not recognized
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

    // Functions for RFPs

    pub async fn upsert_rfp(
        tx: &mut Transaction<'static, Postgres>,
        rfp_id: u32,
        author_id: String,
    ) -> Result<i32, Error> {
        let rec = sqlx::query!(
            r#"
          UPDATE rfps SET author_id = $1 WHERE id = $2
          RETURNING id
          "#,
            author_id,
            rfp_id as i32,
        )
        .fetch_optional(tx.as_mut())
        .await?;

        if let Some(record) = rec {
            println!("Updated rfp: {:?}", record.id);
            Ok(record.id)
        } else {
            let rec = sqlx::query!(
                r#"
                INSERT INTO rfps (id, author_id)
                VALUES ($1, $2)
                ON CONFLICT (id) DO NOTHING
                RETURNING id
                "#,
                rfp_id as i32,
                author_id
            )
            .fetch_one(tx.as_mut())
            .await?;
            println!("Inserted rfp: {:?}", rec.id);
            Ok(rec.id)
        }
    }

    // TODO Remove this once we go in production or put it behind authentication or a flag
    pub async fn remove_rfp_snapshots_by_rfp_id(&self, rfp_id: i32) -> anyhow::Result<()> {
        sqlx::query!(r#"DELETE FROM rfp_snapshots WHERE rfp_id = $1"#, rfp_id)
            .execute(&self.0)
            .await?;
        Ok(())
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

    // TODO Remove this once we go in production or put it behind authentication or a flag
    pub async fn remove_all_snapshots(&self) -> anyhow::Result<()> {
        sqlx::query!(r#"DELETE FROM proposal_snapshots"#)
            .execute(&self.0)
            .await?;

        sqlx::query!(r#"DELETE FROM rfp_snapshots"#)
            .execute(&self.0)
            .await?;
        Ok(())
    }

    // TODO Remove this once we go in production or put it behind authentication or a flag
    pub async fn remove_all_dao_proposals(&self, account_id: &str) -> anyhow::Result<()> {
        sqlx::query!(
            r#"DELETE FROM dao_proposals WHERE dao_instance = $1"#,
            account_id
        )
        .execute(&self.0)
        .await?;

        Ok(())
    }

    pub async fn remove_all_data(&self) -> anyhow::Result<()> {
        sqlx::query!(r#"DELETE FROM proposals"#)
            .execute(&self.0)
            .await?;

        sqlx::query!(r#"DELETE FROM rfps"#).execute(&self.0).await?;

        sqlx::query!(r#"DELETE FROM proposal_snapshots"#)
            .execute(&self.0)
            .await?;

        sqlx::query!(r#"DELETE FROM rfp_snapshots"#)
            .execute(&self.0)
            .await?;
        Ok(())
    }

    pub async fn insert_rfp_snapshot(
        tx: &mut Transaction<'static, Postgres>,
        snapshot: &RfpSnapshotRecord,
    ) -> anyhow::Result<()> {
        // Primary key is (rfp_id, ts)
        let result = sqlx::query!(
            r#"
        INSERT INTO rfp_snapshots (
            rfp_id,
            block_height,
            ts,
            editor_id,
            social_db_post_block_height,
            labels,
            linked_proposals,
            rfp_version,
            rfp_body_version,
            name,
            category,
            summary,
            description,
            timeline,
            submission_deadline,
            views
        ) VALUES (
            $1, $2, $3, $4, $5, $6, $7, $8,
            $9, $10, $11, $12, $13, $14, $15, $16
        ) ON CONFLICT (rfp_id, ts) DO UPDATE SET
            block_height = $2,
            editor_id = $4,
            social_db_post_block_height = $5,
            labels = $6,
            linked_proposals = $7,
            rfp_version = $8,
            rfp_body_version = $9,
            name = $10,
            category = $11,
            summary = $12,
            description = $13,
            timeline = $14,
            submission_deadline = $15,
            views = $16
        "#,
            snapshot.rfp_id,
            snapshot.block_height,
            snapshot.ts,
            snapshot.editor_id,
            snapshot.social_db_post_block_height,
            snapshot.labels,
            snapshot.linked_proposals,
            snapshot.rfp_version,
            snapshot.rfp_body_version,
            snapshot.name,
            snapshot.category,
            snapshot.summary,
            snapshot.description,
            snapshot.timeline,
            snapshot.submission_deadline,
            snapshot.views
        )
        .execute(tx.as_mut())
        .await;

        match result {
            Ok(_) => {
                println!("Inserted rfp snapshot {:?}", snapshot.rfp_id);
                Ok(())
            }
            Err(e) => {
                eprintln!("Failed to insert rfp snapshot: {:?}", e);
                Err(anyhow::anyhow!("Failed to insert rfp snapshot"))
            }
        }
    }

    pub async fn get_rfps_with_latest_snapshot(
        &self,
        limit: i64,
        order: &str,
        offset: i64,
        filters: Option<GetRfpFilters>,
    ) -> anyhow::Result<(Vec<RfpWithLatestSnapshotView>, i64)> {
        // Validate the order clause to prevent SQL injection
        let order_clause = match order.to_lowercase().as_str() {
            "ts_asc" => "ps.ts ASC",
            "ts_desc" => "ps.ts DESC",
            "id_asc" => "ps.rfp_id ASC",
            "id_desc" => "ps.rfp_id DESC",
            _ => "ps.rfp_id DESC", // Default to DESC if the order is not recognized
        };

        // Extract and validate the stage filter
        let stage = filters.as_ref().and_then(|f| f.stage.as_ref());
        let stage_clause: Option<String> = stage.and_then(|s| match s.to_uppercase().as_str() {
            "ACCEPTING_SUBMISSIONS" => Some("ACCEPTING_SUBMISSIONS".to_string()),
            "EVALUATION" => Some("EVALUATION".to_string()),
            "PROPOSAL_SELECTED" => Some("PROPOSAL_SELECTED".to_string()),
            "CANCELLED" => Some("CANCELLED".to_string()),
            _ => None,
        });

        // Build the SQL query for fetching data with the validated order clause
        let data_sql = format!(
            r#"
          SELECT
              *
          FROM
              rfps_with_latest_snapshot ps
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

        // Build the SQL query for counting total records
        let count_sql = r#"
          SELECT COUNT(*)
          FROM rfps_with_latest_snapshot ps
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
        let recs = sqlx::query_as::<_, RfpWithLatestSnapshotView>(&data_sql)
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

    pub async fn get_rfp_with_latest_snapshot_by_id(
        &self,
        id: i32,
    ) -> anyhow::Result<RfpWithLatestSnapshotView> {
        let sql = r#" 
          SELECT
              ps.*
          FROM
              rfps_with_latest_snapshot ps
          WHERE
              ps.rfp_id = $1
      "#;

        let result = sqlx::query_as::<_, RfpWithLatestSnapshotView>(sql)
            .bind(id)
            .fetch_one(&self.0)
            .await;

        match result {
            Ok(rfp) => Ok(rfp),
            Err(e) => {
                eprintln!("Failed to get rfp with latest snapshot: {:?}", e);
                Err(anyhow::anyhow!("Failed to get rfp with latest snapshot"))
            }
        }
    }

    pub async fn get_rfp_with_all_snapshots(
        &self,
        id: i64,
    ) -> anyhow::Result<(Vec<RfpSnapshotRecord>, i64)> {
        // Group by ts
        // Build the SQL query for fetching data with the validated order clause
        let data_sql = r#"
          SELECT
              rfp.*
          FROM
              rfp_snapshots rfp
          WHERE
             rfp.rfp_id = $1
          ORDER BY
              rfp.ts DESC
          "#;

        // Execute the data query
        let result = sqlx::query_as::<_, RfpSnapshotRecord>(data_sql)
            .bind(id)
            .fetch_all(&self.0)
            .await;

        let count_sql = r#"
            SELECT COUNT(*)
            FROM rfp_snapshots
            WHERE rfp_id = $1
        "#;

        // Build the SQL query for counting total records
        let total_count = sqlx::query_scalar(count_sql)
            .bind(id)
            .fetch_one(&self.0)
            .await?;

        match result {
            Ok(recs) => Ok((recs, total_count)),
            Err(e) => {
                eprintln!("Failed to get rfp with all snapshots: {:?}", e);
                Err(anyhow::anyhow!("Failed to get rfp with all snapshots"))
            }
        }
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

    pub async fn search_rfps_with_latest_snapshot(
        &self,
        input: &str,
        limit: i64,
        offset: i64,
    ) -> anyhow::Result<(Vec<RfpWithLatestSnapshotView>, i64)> {
        let sql = r#"
          SELECT
              ps.*
          FROM
              rfps_with_latest_snapshot ps
          WHERE
              to_tsvector('english', coalesce(ps.name, '') || ' ' || coalesce(ps.summary, '') || ' ' || coalesce(ps.description, '')) @@ plainto_tsquery($1)
              OR lower(ps.name) ILIKE $1
              OR lower(ps.summary) ILIKE $1
              OR lower(ps.description) ILIKE $1
          ORDER BY ps.ts DESC
          LIMIT $2 OFFSET $3
      "#;

        let rfps = sqlx::query_as::<_, RfpWithLatestSnapshotView>(sql)
            .bind(input)
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.0)
            .await?;

        let total_count_sql = r#"
          SELECT
              COUNT(*)
          FROM
              rfps_with_latest_snapshot ps
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

        Ok((rfps, total_count))
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

    pub async fn get_latest_rfp_snapshot(
        &self,
        rfp_id: i32,
    ) -> Result<Option<RfpSnapshotRecord>, sqlx::Error> {
        let sql = r#"
          SELECT *
          FROM rfp_snapshots
          WHERE rfp_id = $1
          ORDER BY ts DESC
          LIMIT 1
        "#;

        let snapshot = sqlx::query_as::<_, RfpSnapshotRecord>(sql)
            .bind(rfp_id)
            .fetch_optional(&self.0)
            .await?;

        Ok(snapshot)
    }

    pub async fn upsert_dao_proposal_snapshot(
        tx: &mut Transaction<'static, Postgres>,
        record: SputnikProposalSnapshotRecord,
    ) -> anyhow::Result<()> {
        let sql = r#"
          INSERT INTO dao_proposals (description, id, kind, proposer, status, submission_time, vote_counts, votes, total_votes, dao_instance, proposal_action, tx_timestamp, hash)
          VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
          ON CONFLICT (id) DO UPDATE SET
            description = $1,
            kind = $3,
            proposer = $4,
            status = $5,
            submission_time = $6,
            vote_counts = $7,
            votes = $8,
            total_votes = $9,
            dao_instance = $10,
            proposal_action = $11,
            tx_timestamp = $12,
            hash = $13
        "#;
        let result = sqlx::query(sql)
            .bind(record.description)
            .bind(record.id)
            .bind(record.kind)
            .bind(record.proposer)
            .bind(record.status)
            .bind(record.submission_time)
            .bind(record.vote_counts)
            .bind(record.votes)
            .bind(record.total_votes)
            .bind(record.dao_instance)
            .bind(record.proposal_action)
            .bind(record.tx_timestamp)
            .bind(record.hash)
            .execute(tx.as_mut())
            .await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => {
                eprintln!("Failed to insert dao proposal snapshot: {:?}", e);
                Err(anyhow::anyhow!("Failed to insert dao proposal snapshot"))
            }
        }
    }

    pub async fn insert_txn(
        tx: &mut Transaction<'static, Postgres>,
        record: SputnikTxnsRecord,
    ) -> anyhow::Result<()> {
        let sql = r#"
          INSERT INTO txns (id, hash, author_id, dao_instance, proposer, description, kind, status, total_votes, vote_counts, votes, submission_time, proposal_action)
          VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
        "#;
        let result = sqlx::query(sql)
            .bind(record.id)
            .bind(record.hash)
            .bind(record.author_id)
            .bind(&record.dao_instance)
            .bind(record.proposer)
            .bind(record.description)
            .bind(record.kind)
            .bind(record.status)
            .bind(record.total_votes)
            .bind(record.vote_counts)
            .bind(record.votes)
            .bind(record.submission_time)
            .bind(record.proposal_action)
            .execute(tx.as_mut())
            .await;

        match result {
            Ok(_) => {
                println!(
                    "Inserted transaction from account {} and id {:?}",
                    &record.dao_instance, record.id
                );
                Ok(())
            }
            Err(e) => {
                eprintln!("Failed to insert txn: {:?}", e);
                Err(anyhow::anyhow!("Failed to insert txn"))
            }
        }
    }

    // pub async fn get_proposal_by_id(
    //     &self,
    //     proposal_id: i64,
    // ) -> Result<Option<ProposalRecord>, sqlx::Error> {
    //     let sql = r#"
    //       SELECT *
    //       FROM proposals
    //       WHERE id = $1
    //     "#;
    //     let proposal = sqlx::query_as::<_, ProposalRecord>(sql)
    //         .bind(proposal_id)
    //         .fetch_optional(&self.0)
    //         .await?;
    //     Ok(proposal)
    // }

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
            "id_asc" => "id ASC",
            "id_desc" => "id DESC",
            _ => "id DESC", // Default to DESC if the order is not recognized
        };

        let kind = filters.as_ref().and_then(|f| f.kind.as_ref());
        // let total_votes = filters.as_ref().and_then(|f| f.total_votes.as_ref());
        let status = filters.as_ref().and_then(|f| f.status.as_ref());

        /*
         * TODO add filters
        *  AND ($2 IS NULL OR kind->>'key' ILIKE '%' || $2 || '%')
         AND ($3 IS NULL OR status->>'key' ILIKE '%' || $3 || '%')
        */
        let sql = format!(
            r#"
          SELECT *
          FROM dao_proposals
          WHERE dao_instance = $1
          ORDER BY {}
          LIMIT $4 OFFSET $5
        "#,
            order_clause,
        );

        let proposals = sqlx::query_as::<_, SputnikProposalSnapshotRecord>(&sql)
            .bind(dao_instance)
            .bind(kind)
            .bind(status)
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.0)
            .await?;

        /*
        AND ($2 IS NULL OR kind->>'key' ILIKE '%' || $2 || '%')
        AND ($3 IS NULL OR status->>'key' ILIKE '%' || $3 || '%') */
        let count_sql = r#"
            SELECT COUNT(*)
            FROM dao_proposals
            WHERE dao_instance = $1
        "#;

        let total_count = sqlx::query_scalar::<_, i64>(count_sql)
            .bind(dao_instance)
            .bind(kind)
            .bind(status)
            .fetch_one(&self.0)
            .await?;

        Ok((proposals, total_count))
    }

    pub async fn get_txns(
        &self,
        account_id: &str,
        limit: i64,
        order: &str,
        offset: i64,
        filters: Option<GetTxnsFilters>,
    ) -> anyhow::Result<(Vec<SputnikTxnsRecord>, i64)> {
        // Validate the order clause to prevent SQL injection
        let order_clause = match order.to_lowercase().as_str() {
            "ts_asc" => "submission_time ASC",
            "ts_desc" => "submission_time DESC",
            "id_asc" => "id ASC",
            "id_desc" => "id DESC",
            _ => "id DESC", // Default to DESC if the order is not recognized
        };

        let author_id = filters.as_ref().and_then(|f| f.author_id.as_ref());
        let kind = filters.as_ref().and_then(|f| f.kind.as_ref());
        // let total_votes = filters.as_ref().and_then(|f| f.total_votes.as_ref());
        let status = filters.as_ref().and_then(|f| f.status.as_ref());

        let sql = format!(
            r#"
          SELECT *
          FROM txns
          WHERE dao_instance = $1
          AND ($2 IS NULL OR author_id = $2)
          AND ($3 IS NULL OR kind = $3)
          AND ($4 IS NULL OR status = $4)
          ORDER BY {}
          LIMIT $5 OFFSET $6
        "#,
            order_clause,
        );

        let txns = sqlx::query_as::<_, SputnikTxnsRecord>(&sql)
            .bind(account_id)
            .bind(author_id)
            .bind(kind)
            .bind(status)
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.0)
            .await?;

        let count_sql = r#"
            SELECT COUNT(*)
            FROM txns
            WHERE dao_instance = $1
        "#;

        let total_count = sqlx::query_scalar::<_, i64>(count_sql)
            .bind(account_id)
            .fetch_one(&self.0)
            .await?;

        Ok((txns, total_count))
    }
}

async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    match DB::fetch(&rocket) {
        Some(db) => match migrate!("./migrations").run(&**db).await {
            Ok(_) => Ok(rocket),
            Err(e) => {
                rocket::error!("Failed to initialize SQLx database: {}", e);
                Err(rocket)
            }
        },
        None => Err(rocket),
    }
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("SQLx Stage", |rocket| async {
        rocket
            .attach(DB::init())
            .attach(AdHoc::try_on_ignite("SQLx Migrations", run_migrations))
    })
}
