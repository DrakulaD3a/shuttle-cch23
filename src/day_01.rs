use axum::{extract::Path, response::IntoResponse};

pub async fn day01(Path(nums): Path<String>) -> impl IntoResponse {
    nums.split('/')
        .map(|num| num.parse::<i32>().unwrap_or(0))
        .reduce(|a, b| a ^ b)
        .unwrap_or(0)
        .pow(3)
        .to_string()
        .into_response()
}
