use std::fmt::Display;

use crate::error::{Error, ErrorKind};

use clap::ValueEnum;
use sqlx::SqlitePool;


#[derive(Debug)]
pub struct Database {
    pool: SqlitePool,
}

impl Drop for Database {
    fn drop(&mut self) {
        tokio::task::block_in_place(move || {
            tokio::runtime::Handle::current().block_on(async move { self.pool.close().await });
        })
    }
}

#[repr(i64)]
#[derive(Debug, Clone, ValueEnum, sqlx::Type)]
pub(crate) enum Breed {
    Earth = 0,
    Pegasus = 1,
    Unicorn = 2,
}

impl Display for Breed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let breed = match self {
            Breed::Earth => " Earth ",
            Breed::Pegasus => "Pegasus",
            Breed::Unicorn => "Unicorn",
        };

        write!(f, "{breed}")
    }
}

impl From<i64> for Breed {
    fn from(value: i64) -> Self {
        match value {
            0 => Breed::Earth,
            1 => Breed::Pegasus,
            2 => Breed::Unicorn,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RecordData {
    pub(crate) name: String,
    pub(crate) breed: Breed,
}

#[derive(Debug, Clone)]
pub(crate) struct Record {
    pub(crate) id: i64,
    pub(crate) name: String,
    pub(crate) breed: Breed,
}

impl Display for Record {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Record { name, id, breed } = self;

        write!(f, "{id:4} | {breed} | {name}")
    }
}

impl Database {
    pub async fn new() -> Result<Self, Error> {
        // TODO get database url from another source?
        let database_url =
            std::env::var("DATABASE_URL").map_err(|err| Error::fatal(err.to_string()))?;

        let pool = SqlitePool::connect(&database_url).await.map_err(|err| {
            Error::new(ErrorKind::Database {
                message: err.to_string(),
                url: database_url,
            })
        })?;

        Ok(Self { pool })
    }

    pub async fn add(&mut self, record: RecordData) -> Result<i64, Error> {
        let query = sqlx::query!(
            r#"
            insert into mares (name, breed)
            values (?1, ?2)
            "#,
            record.name,
            record.breed
        );

        dbg!(&record.breed);

        let id = query
            .execute(&self.pool)
            .await
            .map_err(|err| Error::fatal(err.to_string()))?;

        Ok(id.last_insert_rowid())
    }

    pub(crate) async fn get(&self, id: i64) -> Result<Option<Record>, Error> {
        let query = sqlx::query_as!(
            Record,
            r#"
            select name as "name!", breed as "breed!", id as "id!"
            from mares
            where id = ?1
            "#,
            id
        );

        let record = query
            .fetch_optional(&self.pool)
            .await
            .map_err(|err| Error::fatal(err.to_string()))?;

        Ok(record)
    }

    pub(crate) async fn remove(&mut self, id: i64) -> Result<Option<Record>, Error> {
        let query = sqlx::query_as!(
            Record,
            r#"
            delete from mares
            where id = ?1
            returning name as "name!", breed as "breed!", id as "id!"
            "#,
            id
        );

        let record = query
            .fetch_optional(&self.pool)
            .await
            .map_err(|err| Error::fatal(err.to_string()))?;

        Ok(record)
    }

    pub(crate) async fn list(&self) -> Result<Vec<Record>, Error> {
        let query = sqlx::query_as!(
            Record,
            r#"
            select id as "id!", name as "name!", breed as "breed!"
            from mares
            "#
        );

        let records = query
            .fetch_all(&self.pool)
            .await
            .map_err(|err| Error::fatal(err.to_string()))?;

        Ok(records)
    }
}
