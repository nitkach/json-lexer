use askama::Template;
use askama_axum::IntoResponse;
use axum::{
    routing::get,
    Router,
};
use tracing::{info, span, Level, info_span};

#[derive(Debug, Template)]
#[template(path = "E:\\dev\\json\\mare-website\\src\\bin\\templates\\test.askama.html")]
struct IndexTemplate {
    foo: String
}

async fn get_index() -> impl IntoResponse {
    info!("Info from get_index");

    IndexTemplate {
        foo: r#"mare"#.to_owned(),
    }
}

#[tokio::main]
async fn main() {
    let env = env_logger::Env::default().default_filter_or("info");
    env_logger::Builder::from_env(env).init();

    let my_span = span!(Level::INFO, "my_span");

    info!("Info from main");

    let x = my_span.enter();

    let foo_span = info_span!("foo_span");

    // build our application with a single route
    let app = Router::new().route("/", get(get_index));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
