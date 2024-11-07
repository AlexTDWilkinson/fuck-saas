use crate::AppState;
use axum::extract::{Query, State};
use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct PollQuery {
    channel_id: i64,
    last_timestamp: Option<String>,
}

#[derive(Serialize)]
pub struct MessageResponse {
    content: String,
    creator_id: i64,
    username: String,
    created_at: i64,
    edited_at: Option<i64>,
}

#[derive(Serialize)]
pub struct PollResponse {
    messages: Vec<MessageResponse>,
}

pub async fn message_poll(
    State(state): State<AppState>,
    Query(query): Query<PollQuery>,
) -> Json<PollResponse> {
    // Parse the last_timestamp to i64 if provided
    let last_ts = query
        .last_timestamp
        .and_then(|ts| ts.parse::<i64>().ok())
        .unwrap_or(0);

    let messages = sqlx::query_as!(
        MessageResponse,
        r#"
        SELECT 
            m.content,
            m.creator_id,
            a.username,
            m.created_at,
            m.edited_at
        FROM message m
        JOIN account a ON m.creator_id = a.id
        WHERE m.channel_id = ?
        AND (? = 0 OR m.created_at > ? OR (m.edited_at IS NOT NULL AND m.edited_at > ?))
        ORDER BY m.created_at ASC
        LIMIT 100
        "#,
        query.channel_id,
        last_ts,
        last_ts,
        last_ts
    )
    .fetch_all(&state.pool)
    .await
    .unwrap_or_default();

    Json(PollResponse { messages })
}
