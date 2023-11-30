use axum::Router;
use axum::{http::StatusCode, routing::get};

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
        .route("/-1/error", get(server_error));

    Ok(router.into())
}
