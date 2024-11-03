use axum::extract::Query;
use axum::response::IntoResponse;
use serde::Deserialize;

#[axum::debug_handler]
pub async fn home() -> impl IntoResponse {
    axum::http::Response::builder()
        .status(302)
        .header("Location", format!("/channel/{}", 0))
        .body("".to_string())
        .expect("Failed to build redirect response")
}
