use axum::Router;
use axum::routing::post;
use axum::{http::StatusCode, routing::get};

mod day_01;
mod day_04;

async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn server_error() -> StatusCode {
    StatusCode::INTERNAL_SERVER_ERROR
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/-1/error", get(server_error))
        .route("/1/*nums", get(day_01::day01))
        .route("/4/strength", post(day_04::task1))
        .route("/4/contest", post(day_04::task2));

    Ok(router.into())
}
