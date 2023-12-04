use axum::{extract, response::IntoResponse, Json};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
pub struct Reindeer {
    name: String,
    strength: i32,
}

pub async fn task1(extract::Json(payload): extract::Json<Vec<Reindeer>>) -> impl IntoResponse {
    payload
        .iter()
        .map(|reindeer| reindeer.strength)
        .sum::<i32>()
        .to_string()
        .into_response()
}

#[derive(Deserialize)]
pub struct ContestReindeer {
    name: String,
    strength: i32,
    speed: f32,
    height: i32,
    antler_width: i32,
    snow_magic_power: i32,
    favorite_food: String,
    #[serde(rename = "cAnD13s_3ATeN-yesT3rdAy")]
    candies_eaten_yesterday: i32,
}

pub async fn task2(
    extract::Json(payload): extract::Json<Vec<ContestReindeer>>,
) -> impl IntoResponse {
    let fastest = payload
        .iter()
        .max_by(|a, b| a.speed.total_cmp(&b.speed))
        .unwrap();
    let tallest = payload
        .iter()
        .max_by_key(|reindeer| reindeer.height)
        .unwrap();
    let magician = payload
        .iter()
        .max_by_key(|reindeer| reindeer.snow_magic_power)
        .unwrap();
    let consumer = payload
        .iter()
        .max_by_key(|reindeer| reindeer.candies_eaten_yesterday)
        .unwrap();

    let response = json!({
        "fastest": format!("Speeding past the finish line with a strength of {} is {}", fastest.strength, fastest.name),
        "tallest": format!("{} is standing tall with his {} cm wide antlers", tallest.name, tallest.height),
        "magician": format!("{} could blast you away with a snow magic power of {}", magician.name, magician.snow_magic_power),
        "consumer": format!("{} ate lots of candies, but also some {}", consumer.name, consumer.favorite_food),
    });

    Json(response)
}
