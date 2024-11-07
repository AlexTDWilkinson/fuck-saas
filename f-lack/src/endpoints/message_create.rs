use crate::AppState;
use axum::response::IntoResponse;
use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use time::OffsetDateTime;

use tokio::time::Duration;
// Add this with your other routes in the Router setup

#[axum::debug_handler]

pub async fn message_create(
    State(state): State<AppState>,
    Json(message): Json<CreateMessage>,
) -> impl IntoResponse {
    let creator_id = 1;

    loop {
        let now: i64 = (OffsetDateTime::now_utc().unix_timestamp_nanos() / 1_000_000) as i64;

        match sqlx::query!(
            "INSERT INTO message (channel_id, created_at, creator_id, content) 
             VALUES (?, ?, ?, ?)",
            message.channel_id,
            now,
            creator_id,
            message.content
        )
        .execute(&state.pool)
        .await
        {
            Ok(_) => return StatusCode::CREATED.into_response(),
            Err(e) if e.to_string().contains("UNIQUE constraint failed") => {
                tokio::time::sleep(Duration::from_millis(1)).await;
                continue;
            }
            Err(e) => {
                eprintln!("Failed to create message: {}", e);
                return StatusCode::INTERNAL_SERVER_ERROR.into_response();
            }
        }
    }
}

// Add this struct
#[derive(Deserialize)]
pub struct CreateMessage {
    pub channel_id: i64,
    pub content: String,
}
