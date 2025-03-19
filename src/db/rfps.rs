use super::db_types::{RfpSnapshotRecord, RfpWithLatestSnapshotView};
use super::DB;
use crate::entrypoints::rfp::rfp_types::GetRfpFilters;
use sqlx::{Error, Postgres, Transaction};

impl DB {
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

    pub async fn insert_rfp_snapshot(
        tx: &mut Transaction<'static, Postgres>,
        snapshot: &RfpSnapshotRecord,
    ) -> anyhow::Result<()> {
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
    ) -> anyhow::Result<Vec<RfpSnapshotRecord>> {
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

        match result {
            Ok(recs) => Ok(recs),
            Err(e) => {
                eprintln!("Failed to get rfp with all snapshots: {:?}", e);
                Err(anyhow::anyhow!("Failed to get rfp with all snapshots"))
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
}
