use anyhow::{anyhow, Result};
use askama_axum::Template;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Form, Router};
use serde::Deserialize;
use tracing::{info, warn};

use crate::database::breed::Breed;
use crate::database::{Database, DatabaseRecord};
use app_error::AppError;

mod app_error;

pub async fn run() -> Result<()> {
    let shared_state = Database::new().await?;

    // build our application with a single route
    let app = Router::new()
        .route("/", get(get_home))
        .route("/mares", post(post_mares))
        .route("/mares/:id", get(get_mare))
        .route("/mares/:id/delete", post(delete_mare))
        .route("/mares/:id/edit", post(edit_mare))
        .route("/mares/:id/image", get(mare_image))
        .with_state(shared_state);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

#[derive(Debug, Template)]
#[template(path = "index.askama.html")]
struct IndexTemplate {
    ponies: Vec<DatabaseRecord>,
}

async fn get_home(State(pool): State<Database>) -> Result<impl IntoResponse, AppError> {
    let vec_of_ponies = pool.list().await?;

    let html = IndexTemplate {
        ponies: vec_of_ponies,
    };

    Ok(html)
}

#[derive(Deserialize, Debug)]
pub(crate) struct PonyData {
    pub(crate) pony_name: String,
    pub(crate) breed: Breed,
}

async fn post_mares(
    State(pool): State<Database>,
    form: Form<PonyData>,
) -> Result<impl IntoResponse, AppError> {
    let form = form.0;

    let id = pool.add(&form).await?;

    info!(
        "Added new record: {id} | {} | {}",
        form.breed, form.pony_name
    );

    Ok(axum::response::Redirect::to("/"))
}

async fn delete_mare(
    State(pool): State<Database>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    let Some(record) = pool.remove(id).await? else {
        // FIXME
        warn!("Record with id = {id} was not found, but a request has been received to delete it.");
        // TODO ErrorCode: 404
        return Err(AppError::with_status_404(anyhow!("Cannot find record with {id} id.")));
    };

    info!("Successfully deleted record: {record}");

    Ok(axum::response::Redirect::to("/"))
}

#[derive(Debug, Template)]
#[template(path = "get_mare.askama.html")]
struct GetMareTemplate {
    name: String,
    breed: Breed,
    id: i64,
}

async fn get_mare(
    State(pool): State<Database>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    let Some(mare) = pool.get(id).await? else {
        warn!("Record with id = {id} was not found.");
        // TODO return HTML page ErrorCode: 404
        return Err(anyhow!("{}", axum::http::StatusCode::NOT_FOUND).into());
    };

    let html = GetMareTemplate {
        name: mare.name,
        breed: mare.breed,
        id,
    };

    Ok(html)
}

#[derive(Debug, Deserialize)]
struct ImageResponse {
    images: Vec<Image>,
}

#[derive(Debug, Deserialize)]
struct Image {
    id: i64,
    representations: Representations,
}

#[derive(Debug, Deserialize)]
struct Representations {
    // large: String,
    medium: String,
    // small: String,
}

#[derive(Debug, Template)]
#[template(path = "mare_image.askama.html")]
struct MareImageTemplate {
    pony_id: i64,
    image_id: i64,
    image: String,
}

async fn mare_image(
    State(pool): State<Database>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    let name = match pool.get(id).await? {
        Some(record) => record.name,
        None => {
            // FIXME is warn necessary? is returning Err(anyhow!(StasusCode)) correct?
            warn!("Record with id = {id} was not found.");
            return Err(anyhow!(axum::http::StatusCode::NOT_FOUND).into());
        }
    };

    let client = reqwest::Client::builder()
        .user_agent(concat!(
            "SimpleMareWebsite",
            env!("CARGO_PKG_VERSION"),
            "https://github.com/nitkach",
        ))
        .build()?;

    // ?per_page=1&q=score.gte%3A100%2C{}%2Cpony%2Cmare%2C!irl&sf=random&sd=desc
    let tags = format!("score.gte:100, {name}, pony, mare, !irl");
    let request = client
        .get("https://derpibooru.org/api/v1/json/search/images")
        .query(&[("per_page", "1"), ("sf", "random"), ("q", &tags)]);

    let mut response = request.send().await?.json::<ImageResponse>().await?;

    let image = match response.images.pop() {
        Some(image) => image,
        None => {
            // let html = "Images not found.\n<a href=\"/\">Back to main page</a>.";
            // return Ok(axum::response::Html(html.to_owned()));
            todo!()
        }
    };

    let html = MareImageTemplate {
        pony_id: id,
        image_id: image.id,
        image: image.representations.medium,
    };

    Ok(html)
}

async fn edit_mare(
    State(pool): State<Database>,
    Path(id): Path<i64>,
    form: Form<PonyData>,
) -> Result<impl IntoResponse, AppError> {
    // let pony_data = form.0;

    // // problem: if first user edit data, but not send it, this is problem
    // // fix: add timestamp_updated_at as field -> send to request to update
    // // if timestamp != timestamp of record -> problem
    // // optimistic concurrency

    // // chrono::DateTime
    // // sqlx feature to support Timestamp

    // // html: timestamp (when sended) - hidden form input
    // let Some(record) = pool.set(id, pony_data).await? else {
    //     return Err(anyhow!(axum::http::StatusCode::NOT_FOUND).into());
    // };

    // info!("Successfully set ");

    info!("edit_mare was invoked!");

    dbg!(&form.0);

    if id == 44 {
        return Ok(axum::response::Html("Function returns ok!"));
    }

    Err(AppError::with_status_404(anyhow!("{}, the {} pony is not found is mares list :(", form.0.pony_name, form.0.breed)))
}
