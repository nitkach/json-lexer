use std::fmt::Display;

use anyhow::Result;
use axum::extract::{Path, State};
use axum::response::{IntoResponse, Response};
use axum::routing::post;
use axum::Form;
use axum::{routing::get, Router};
use hyper::StatusCode;
use itertools::Itertools;
use serde::Deserialize;
use sqlx::SqlitePool;

#[derive(Deserialize, Debug)]
struct FormData {
    pony_name: String,
    breed: Breed,
}

#[repr(i64)]
#[derive(Deserialize, Debug, Clone, sqlx::Type)]
#[serde(rename_all = "snake_case")]
enum Breed {
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

struct Pony {
    name: String,
    breed: Breed,
}

async fn get_home(State(pool): State<Database>) -> Result<impl IntoResponse, AppError> {
    let html = std::fs::read_to_string("assets/index.html").unwrap();

    let vec_of_ponies = pool.list().await?;
    let list = vec_of_ponies
        .iter()
        .map(|Record { id, name, breed }| {
            format!(
                r#"<li>
                    Name {name}, Breed: {breed:?}
                    <form method="post" action="/mares/{id}/delete">
                        <input type="submit" value="Delete">
                    </form>
                </li>"#,
            )
        })
        .join("");

    Ok(axum::response::Html(html.replace("{{ ponies }}", &list)))
}

async fn post_mares(
    State(pool): State<Database>,
    form: Form<FormData>,
) -> Result<impl IntoResponse, AppError> {
    let form = form.0;

    let msg = format!("Added record: {} | {}", form.pony_name, form.breed);

    let id = pool.add(form).await?;

    eprintln!("{msg} with id: {id}");

    Ok(axum::response::Redirect::to("/"))
}

async fn delete_mares(
    State(pool): State<Database>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    let record = pool.remove(id).await?;

    if let Some(record) = record {
        eprintln!("Successfully deleted record: {record}");
    } else {
        eprintln!(
            "Record with id = {id} does not exist, but a request has been received to delete it."
        )
    }

    Ok(axum::response::Redirect::to("/"))
}

#[derive(Clone)]
struct Database {
    pool: SqlitePool,
}

#[derive(Debug, Clone)]
pub(crate) struct Record {
    pub(crate) id: i64,
    pub(crate) name: String,
    pub(crate) breed: Breed,
}

impl Display for Record {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Record { id, name, breed } = self;

        write!(f, "{id} | {breed} | {name}")
    }
}

impl Database {
    async fn new() -> Result<Self> {
        let database_url = std::env::var("DATABASE_URL")?;

        let pool = SqlitePool::connect(&database_url).await?;

        Ok(Self { pool })
    }

    async fn add(&self, record: FormData) -> Result<i64> {
        let query = sqlx::query!(
            r#"
            insert into mares (name, breed)
            values (?1, ?2)
            "#,
            record.pony_name,
            record.breed
        );

        let id = query.execute(&self.pool).await?;

        Ok(id.last_insert_rowid())
    }

    async fn list(&self) -> Result<Vec<Record>> {
        let query = sqlx::query_as!(
            Record,
            r#"
            select id as "id!", name as "name!", breed as "breed!"
            from mares
            "#
        );

        let records = query.fetch_all(&self.pool).await?;

        Ok(records)
    }

    async fn remove(&self, id: i64) -> Result<Option<Record>> {
        let query = sqlx::query_as!(
            Record,
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

struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    if let Err(err) = dotenvy::dotenv() {
        return Err(anyhow::Error::new(err));
    }

    let pool = Database::new().await?;

    let shared_state = pool;

    // build our application with a single route
    let app = Router::new()
        .route("/", get(get_home))
        .route("/mares", post(post_mares))
        .route("/mares/:id/delete", post(delete_mares))
        .with_state(shared_state);

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
