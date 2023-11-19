
use axum::http;
use std::fs;

pub async fn health() -> http::StatusCode {
    http::StatusCode::OK
}

pub async fn capacity() -> Result<axum::Json<i32>, http::StatusCode> {
    let content = fs::read_to_string("capacity.txt").expect("Unable to read file");
    Ok(axum::Json(content.trim().parse().unwrap()))
}
