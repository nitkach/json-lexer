use askama::Template;
use askama_axum::IntoResponse;
use axum::{
    routing::{get, post},
    Form, Router, debug_handler,
};

#[derive(Debug, Template)]
#[template(path = "E:\\dev\\json\\mare-website\\src\\bin\\templates\\test.askama.html")]
struct IndexTemplate {
    foo: i64,
}

async fn get_index() -> impl IntoResponse {
    IndexTemplate {
        foo: chrono::offset::Utc::now().timestamp(),
    }
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
enum Breed {
    Earth,
    Pegasus,
    Unicorn,
}

#[derive(Debug, serde::Deserialize)]
struct Data {
    name: String,
    breed: Breed,
    timestamp: i64,
}

#[debug_handler]
async fn post_index(Form(data): Form<Data>) -> impl IntoResponse {
    dbg!(data);

    axum::response::Redirect::to("/")
}

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(get_index))
        .route("/", post(post_index));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
