use axum::{response::IntoResponse, Json};
use serde_json::json;

pub async fn day_06(body: String) -> impl IntoResponse {
    // Appending the string with any chars at the end and start so it works even when elf is the
    // last or first substring in string
    let num_of_elves = format!("x{body}x").split("elf").count() - 1;
    let num_of_elves_on_shelfs = format!("x{body}x").split("elf on a shelf").count() - 1;
    let num_of_shelfs_without_elfs =
        format!("x{body}x").split("shelf").count() - 1 - num_of_elves_on_shelfs;

    Json(json!({
        "elf": num_of_elves,
        "elf on a shelf": num_of_elves_on_shelfs,
        "shelf with no elf on it": num_of_shelfs_without_elfs
    }))
}
