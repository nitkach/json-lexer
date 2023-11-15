use std::{fmt::Display, str::FromStr};

use anyhow::Result;
use log::{info, LevelFilter};
use serde::Deserialize;
use sqlx::{
    sqlite::SqliteConnectOptions, ConnectOptions, Pool, Sqlite, SqliteConnection, SqlitePool,
};
use tracing::{instrument, Level};

use crate::app::PonyData;

pub(crate) mod breed;

#[derive(Debug, Clone)]
pub(crate) struct DatabaseRecord {
    pub(crate) id: i64,
    pub(crate) name: String,
    pub(crate) breed: breed::Breed,
}

impl Display for DatabaseRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let DatabaseRecord { id, name, breed } = self;

        write!(f, "{id} | {breed} | {name}")
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Database {
    pool: SqlitePool,
}

impl Database {
    pub(crate) async fn new() -> Result<Self> {
        let database_url = std::env::var("DATABASE_URL")?;

        let options = SqliteConnectOptions::from_str(&database_url)?
            .log_statements(LevelFilter::Info)
            .log_slow_statements(log::LevelFilter::Warn, core::time::Duration::from_secs(1));
        let pool = SqlitePool::connect_with(options).await?;

        // let pool = SqlitePool::connect(&database_url).await?;

        info!(
            "Connection pool with SQLx database has been created from: `{}`",
            &database_url
        );

        Ok(Self { pool })
    }

    #[instrument(level = Level::DEBUG)]
    pub(crate) async fn add(&self, data: &PonyData) -> Result<i64> {
        let query = sqlx::query!(
            r#"
            insert into mares (name, breed)
            values (?1, ?2)
            "#,
            data.pony_name,
            data.breed
        );

        // TODO rows_affected=1 rows_returned=0 elapsed=3.8952ms
        let id = query.execute(&self.pool).await?.last_insert_rowid();

        Ok(id)
    }

    #[instrument(level = Level::DEBUG)]
    pub(crate) async fn get(&self, id: i64) -> Result<Option<DatabaseRecord>> {
        let query = sqlx::query_as!(
            DatabaseRecord,
            r#"
            select id as "id!", name as "name!", breed as "breed!"
            from mares
            where id = ?1
            "#,
            id
        );

        let record = query.fetch_optional(&self.pool).await?;

        info!("Getting record");

        Ok(record)
    }

    pub(crate) async fn set(&self, id: i64, data: &PonyData) -> Result<Option<DatabaseRecord>> {
        // TODO return previous name
        let query = sqlx::query_as!(
            DatabaseRecord,
            r#"
            update mares
            set breed = ?1, name = ?2
            where id = ?3
            returning id as "id!", name as "name!", breed as "breed!"
            "#,
            data.breed,
            data.pony_name,
            id
        );

        let record = query.fetch_optional(&self.pool).await?;

        Ok(record)
    }

    // FIXME target not working! it says that `tracing::span` is the current
    // target, when `skip(self)` is used
    // #[instrument(level = Level::DEBUG)]
    #[instrument(target = "mare_website::database", skip(self))]
    pub(crate) async fn list(&self) -> Result<Vec<DatabaseRecord>> {
        let query = sqlx::query_as!(
            DatabaseRecord,
            r#"
            select id as "id!", name as "name!", breed as "breed!"
            from mares
            "#
        );

        // FIXME
        // query.fetch_all(&self.pool).await.map_err(|err| err.into())

        let records = query.fetch_all(&self.pool).await?;
        info!("List of records");
        Ok(records)
    }

    #[instrument(level = Level::DEBUG)]
    pub(crate) async fn remove(&self, id: i64) -> Result<Option<DatabaseRecord>> {
        let query = sqlx::query_as!(
            DatabaseRecord,
            r#"
            delete from mares
            where id = ?1
            returning name as "name!", breed as "breed!", id as "id!"
            "#,
            id
        );

        let record = query.fetch_optional(&self.pool).await?;

        Ok(record)
    }
}
