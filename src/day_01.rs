use axum::{extract::Path, http::StatusCode, response::IntoResponse};

pub async fn day01(Path(nums): Path<String>) -> impl IntoResponse {
    let mut sum = 0;
    nums.split('/').for_each(|num| {
        sum = sum ^ num.parse::<i32>().unwrap();
    });
    (StatusCode::OK, format!("{}", sum.pow(3)))
}
