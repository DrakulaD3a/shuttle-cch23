use axum::{extract::Path, response::IntoResponse, http::StatusCode};

pub async fn task1(
    Path((num1, num2)): Path<(i32, i32)>,
) -> impl IntoResponse {
    let response = (num1 ^ num2).pow(3);
    (StatusCode::OK, format!("{response}"))
}
