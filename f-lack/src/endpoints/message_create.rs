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

#[derive(Deserialize)]
pub struct CreateMessage {
    pub channel_id: i64,
    pub content: String,
}

#[axum::debug_handler]
pub async fn message_create(
    State(state): State<AppState>,
    Json(message): Json<CreateMessage>,
) -> impl IntoResponse {
    let creator_id = 1;

    loop {
        let now: i64 = (OffsetDateTime::now_utc().unix_timestamp_nanos() / 1_000_000) as i64;

        // Sanitize message content to prevent injection attacks and formatting issues
        let content = message
            .content
            // Remove our DB record separator characters
            .replace('\u{001E}', "")
            .replace('\u{001F}', "")
            // Remove null bytes and other control characters
            .chars()
            .filter(|&c| !c.is_control() || c == '\n' || c == '\t')
            .collect::<String>()
            // Escape HTML/XML tags to prevent injection
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            // Remove potential SQL injection characters
            .replace('\'', "'")
            .replace('"', "\"")
            // Remove HTML comments that could break parsing
            .replace("<!--", "&lt;!--")
            .replace("-->", "--&gt;");

        // if the content is empty, return bad request
        if content.trim().is_empty() {
            return StatusCode::BAD_REQUEST.into_response();
        }

        match sqlx::query!(
            "INSERT INTO message (channel_id, created_at, creator_id, content) 
             VALUES (?, ?, ?, ?)",
            message.channel_id,
            now,
            creator_id,
            content
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
