use anyhow::Result;
use chrono::Utc;
use log::{info, LevelFilter};
use serde::Deserialize;
use sqlx::{postgres::PgConnectOptions, ConnectOptions, PgPool};
use tracing::{instrument, warn, Level};
use url::{self, Url};

use crate::app::{AddPonyForm, EditPonyForm};

pub(crate) mod breed;

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct DatabaseRecord {
    pub(crate) id: i64,
    pub(crate) name: String,
    pub(crate) breed: breed::Breed,
    pub(crate) modified_at: chrono::DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub(crate) struct Database {
    pool: PgPool,
}

impl Database {
    #[instrument(level = Level::INFO)]
    pub(crate) async fn init() -> Result<Self> {
        // let database_url = "postgres://mare:mare@localhost/postgres";

        let database_url = Url::parse(&std::env::var("DATABASE_URL")?)?;

        let options = PgConnectOptions::from_url(&database_url)?
            // PgConnectOptions::new()
            // .host("postgres")
            // .port(5432)
            // .database("mare_data")
            // .username("mare")
            // .password("mare")
            .log_statements(LevelFilter::Debug)
            .log_slow_statements(LevelFilter::Warn, core::time::Duration::from_secs(1));

        let pool = PgPool::connect_with(options).await?;

        sqlx::migrate!().run(&pool).await?;

        // info!(
        //     "Connection pool with SQLx database has been created from: `{}`",
        //     &database_url
        // );

        Ok(Self { pool })
    }

    #[instrument(level = Level::INFO, skip(self))]
    pub(crate) async fn add(&self, data: &AddPonyForm) -> Result<i64> {
        let breed: i32 = data.breed.into();

        let query = sqlx::query_as!(
            DatabaseRecord,
            r#"insert into mares (name, breed, modified_at)
            values ($1, $2, CURRENT_TIMESTAMP)
            returning id as "id!", name as "name!", breed as "breed!", modified_at as "modified_at!";
            "#,
            data.name,
            breed
        );

        let record = query.fetch_one(&self.pool).await?;

        // TODO rows_affected=1 rows_returned=0 elapsed=3.8952ms
        info!(
            "Added new record: {} | {} | {} | {}",
            record.id, record.breed, record.modified_at, record.name
        );

        Ok(record.id)
    }

    #[instrument(level = Level::INFO, skip(self))]
    pub(crate) async fn get(&self, id: i32) -> Result<Option<DatabaseRecord>> {
        let query = sqlx::query_as!(
            DatabaseRecord,
            r#"
            select id as "id!", name as "name!", breed as "breed!", modified_at as "modified_at!"
            from mares
            where id = $1
            "#,
            id
        );

        let record = query.fetch_optional(&self.pool).await?;

        if let Some(record) = &record {
            info!(
                "Record with id = {id} found, returning {} | {} | {}",
                record.name, record.breed, record.modified_at
            );
        } else {
            warn!("Record with id = {id} not found in database.");
        }

        Ok(record)
    }

    #[instrument(level = Level::INFO, skip(self))]
    pub(crate) async fn set(&self, id: i32, data: &EditPonyForm) -> Result<bool> {
        // Result<Option<DatabaseRecord>>

        // TODO return previous record data
        // SQLite doesn't support this feature :/
        // https://stackoverflow.com/questions/6725964/sqlite-get-the-old-value-after-update

        let breed: i32 = data.breed.into();

        // select for update
        let query = sqlx::query_as!(
            DatabaseRecord,
            r#"
            update mares
            set name = $1, breed = $2, modified_at = CURRENT_TIMESTAMP
            where id = $3 and modified_at = $4
            returning id as "id!", name as "name!", breed as "breed!", modified_at as "modified_at!"
            "#,
            data.name,
            breed,
            id,
            data.modified_at
        );

        let record = query.fetch_optional(&self.pool).await?;

        if let Some(record) = &record {
            info!(
                "Record with id = {id} modified to {} | {} | {}",
                record.name, record.breed, record.modified_at
            );
        } else {
            warn!(
                "Record with id = {id} and timestamp = {} not found in database.",
                data.modified_at
            );
        }

        Ok(record.is_some())
    }

    #[instrument(level = Level::INFO, skip(self))]
    pub(crate) async fn list(&self) -> Result<Vec<DatabaseRecord>> {
        let query = sqlx::query_as!(
            DatabaseRecord,
            r#"
            select *
            from mares
            "#
        );

        let records = query.fetch_all(&self.pool).await?;

        info!("Getting list of records, total {} records.", records.len());

        Ok(records)
    }

    #[instrument(level = Level::INFO, skip(self))]
    pub(crate) async fn remove(&self, id: i32) -> Result<Option<DatabaseRecord>> {
        let query = sqlx::query_as!(
            DatabaseRecord,
            r#"
            delete from mares
            where id = $1
            returning name as "name!", breed as "breed!", id as "id!", modified_at as "modified_at!"
            "#,
            id
        );

        let record = query.fetch_optional(&self.pool).await?;

        if let Some(record) = &record {
            info!(
                "Record removed from database: {id} | {} | {} | {}",
                record.name, record.breed, record.modified_at
            );
        } else {
            warn!("Record with id = {id} not found in database.",);
        }

        Ok(record)
    }
}
