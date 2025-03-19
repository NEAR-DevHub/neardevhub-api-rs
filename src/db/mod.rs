use crate::timestamp_to_date_string;
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

pub mod dao_proposals;
pub mod db_types;
pub mod proposals;
pub mod rfps;

use db_types::{BlockHeight, HandlerError, LastUpdatedInfo};

impl DB {
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

    pub async fn set_after_block(
        tx: &mut Transaction<'static, Postgres>,
        contract: &AccountId,
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
            0,
            after_block,
        )
        .execute(tx.as_mut())
        .await?;
        Ok(())
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

    pub async fn set_last_updated_block_on_tx(
        tx: &mut Transaction<'static, Postgres>,
        after_block: BlockHeight,
    ) -> anyhow::Result<()> {
        println!("Storing block: {}", after_block);
        let result = sqlx::query!(
            r#"
          UPDATE last_updated_info SET after_block = $1
          "#,
            after_block
        )
        .execute(tx.as_mut())
        .await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => {
                eprintln!("Failed to set last updated block on tx: {:?}", e);
                Err(anyhow::anyhow!("Failed to set last updated block on tx"))
            }
        }
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

    pub async fn track_handler_error(&self, error: HandlerError) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO handler_errors (transaction_id, error_type, message, block_height, timestamp)
            VALUES ($1, $2, $3, $4, $5)
            "#,
            error.transaction_id,
            error.error_type,
            error.message,
            error.block_height,
            error.timestamp,
        )
        .execute(&self.0)
        .await?;

        eprintln!("Handler error: {:?}", error);

        Ok(())
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
