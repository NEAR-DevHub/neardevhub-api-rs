use crate::entrypoints::{
    proposal::proposal_types::GetProposalFilters, rfp::rfp_types::GetRfpFilters,
};
use rocket::{
    fairing::{self, AdHoc},
    Build, Rocket,
};
use rocket_db_pools::Database;
use sqlx::{migrate, query, query_scalar, Error, PgPool, Postgres, Transaction};

#[derive(Database, Clone, Debug)]
#[database("my_db")]
pub struct DB(PgPool);

pub mod db_types;

use db_types::{
    ProposalSnapshotRecord, ProposalWithLatestSnapshotView, RfpSnapshotRecord,
    RfpWithLatestSnapshotView,
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
            Ok(rec.id)
        }
    }

    pub async fn get_last_updated_timestamp(&self) -> Result<i64, Error> {
        let rec = query_scalar!(
            r#"
            SELECT after_date FROM after_date
            "#
        )
        .fetch_one(&self.0)
        .await?;
        Ok(rec)
    }

    pub async fn set_last_updated_timestamp(&self, after_date: i64) -> Result<(), Error> {
        sqlx::query!(
            r#"
            UPDATE after_date SET after_date = $1
            "#,
            after_date
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
        query!(
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
        .await?;
        Ok(())
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
            "asc" => "ASC",
            "desc" => "DESC",
            _ => "DESC", // Default to DESC if the order is not recognized
        };

        let stage = filters.as_ref().and_then(|f| f.stage.as_ref());
        // Set 'stage_clause' to None if 'stage' is None
        let stage_clause: Option<String> = stage.and_then(|s| match s.to_uppercase().as_str() {
            "DRAFT" => Some("DRAFT".to_string()),
            "REVIEW" => Some("REVIEW".to_string()),
            "APPROVED" => Some("APPROVED".to_string()),
            "REJECTED" => Some("REJECTED".to_string()),
            "CANCELED" => Some("CANCELLED".to_string()),
            "APPROVED_CONDITIONALLY" => Some("CONDITIONALLY".to_string()),
            "PAYMENT_PROCESSING" => Some("PAYMENT".to_string()),
            "FUNDED" => Some("FUNDED".to_string()),
            _ => None,
        });

        // Build the SQL query with the validated order clause
        let data_sql = format!(
            r#"
          SELECT
              ps.proposal_id,
              p.author_id,
              ps.block_height,
              ps.ts,
              ps.editor_id,
              ps.social_db_post_block_height,
              ps.labels,
              ps.proposal_version,
              ps.proposal_body_version,
              ps.name,
              ps.category,
              ps.summary,
              ps.description,
              ps.linked_proposals,
              ps.linked_rfp,
              ps.requested_sponsorship_usd_amount,
              ps.requested_sponsorship_paid_in_currency,
              ps.requested_sponsor,
              ps.receiver_account,
              ps.supervisor,
              ps.timeline,
              ps.views
          FROM
              proposals p
          INNER JOIN (
              SELECT
                  proposal_id,
                  MAX(ts) AS max_ts
              FROM
                  proposal_snapshots
              GROUP BY
                  proposal_id
          ) latest_snapshots ON p.id = latest_snapshots.proposal_id
          INNER JOIN proposal_snapshots ps ON latest_snapshots.proposal_id = ps.proposal_id
              AND latest_snapshots.max_ts = ps.ts
          WHERE
              ($3 IS NULL OR p.author_id = $3)
              AND ($4 IS NULL OR ps.ts > $4)
              AND ($5 IS NULL OR ps.timeline::text ~ $5)
              AND ($6 IS NULL OR ps.category = $6)    
              AND ($7 IS NULL OR ps.labels::jsonb ?| $7)
          ORDER BY ps.ts {}
          LIMIT $1 OFFSET $2
          "#,
            order_clause,
        );

        // Build the count query
        let count_sql = r#"
          SELECT COUNT(*)
          FROM (
              SELECT
                  ps.proposal_id
              FROM
                  proposals p
              INNER JOIN (
                  SELECT
                      proposal_id,
                      MAX(ts) AS max_ts
                  FROM
                      proposal_snapshots
                  GROUP BY
                      proposal_id
              ) latest_snapshots ON p.id = latest_snapshots.proposal_id
              INNER JOIN proposal_snapshots ps ON latest_snapshots.proposal_id = ps.proposal_id
                  AND latest_snapshots.max_ts = ps.ts
              WHERE
                  ($1 IS NULL OR p.author_id = $1)
                  AND ($2 IS NULL OR ps.ts > $2)
                  AND ($3 IS NULL OR ps.timeline::text ~ $3)
                  AND ($4 IS NULL OR ps.category = $4)    
                  AND ($5 IS NULL OR ps.labels::jsonb ?| $5)
          ) AS count_subquery
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
            Ok(rec.id)
        }
    }

    pub async fn insert_rfp_snapshot(
        tx: &mut Transaction<'static, Postgres>,
        snapshot: &RfpSnapshotRecord,
    ) -> anyhow::Result<()> {
        // Primary key is (rfp_id, ts)
        sqlx::query!(
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
        .await?;
        Ok(())
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
            "asc" => "ASC",
            "desc" => "DESC",
            _ => "DESC", // Default to DESC if the order is not recognized
        };

        // Extract and validate the stage filter
        let stage = filters.as_ref().and_then(|f| f.stage.as_ref());
        let stage_clause: Option<String> = stage.and_then(|s| match s.to_uppercase().as_str() {
            // AcceptingSubmissions,
            // Evaluation,
            // "ACCEPTING_SUBMISSIONS" => Some(),
            "PROPOSAL_SELECTED" => Some("PROPOSAL_SELECTED".to_string()),
            "CANCELLED" => Some("CANCELLED".to_string()),

            _ => None,
        });

        // Build the SQL query for fetching data with the validated order clause
        let data_sql = format!(
            r#"
            SELECT
                ps.rfp_id,
                p.author_id,
                ps.block_height,
                ps.ts,
                ps.editor_id,
                ps.social_db_post_block_height,
                ps.labels,
                ps.linked_proposals,
                ps.rfp_version,
                ps.rfp_body_version,
                ps.name,
                ps.category,
                ps.summary,
                ps.description,
                ps.timeline,
                ps.views,
                ps.submission_deadline
            FROM
                rfps p
            INNER JOIN (
                SELECT
                    rfp_id,
                    MAX(ts) AS max_ts
                FROM
                    rfp_snapshots
                GROUP BY
                    rfp_id
            ) latest_snapshots ON p.id = latest_snapshots.rfp_id
            INNER JOIN rfp_snapshots ps ON latest_snapshots.rfp_id = ps.rfp_id
                AND latest_snapshots.max_ts = ps.ts
            WHERE
                ($3 IS NULL OR p.author_id = $3)
                AND ($4 IS NULL OR ps.ts > $4)
                AND ($5 IS NULL OR ps.timeline::text ~ $5)
                AND ($6 IS NULL OR ps.category = $6)
                AND ($7 IS NULL OR ps.labels::jsonb ?| $7)
            ORDER BY ps.ts {order}
            LIMIT $1 OFFSET $2
            "#,
            order = order_clause,
        );

        // Build the SQL query for counting total records
        let count_sql = r#"
            SELECT COUNT(*)
            FROM (
                SELECT
                    ps.rfp_id
                FROM
                    rfps p
                INNER JOIN (
                    SELECT
                        rfp_id,
                        MAX(ts) AS max_ts
                    FROM
                        rfp_snapshots
                    GROUP BY
                        rfp_id
                ) latest_snapshots ON p.id = latest_snapshots.rfp_id
                INNER JOIN rfp_snapshots ps ON latest_snapshots.rfp_id = ps.rfp_id
                    AND latest_snapshots.max_ts = ps.ts
                WHERE
                    ($1 IS NULL OR p.author_id = $1)
                    AND ($2 IS NULL OR ps.ts > $2)
                    AND ($3 IS NULL OR ps.timeline::text ~ $3)
                    AND ($4 IS NULL OR ps.category = $4)
                    AND ($5 IS NULL OR ps.labels::jsonb ?| $5)
            ) AS count_subquery
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

    // Additional functions can be added as needed
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
