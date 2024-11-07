use crate::AppState;
use axum::response::IntoResponse;
use axum::{extract::State, http::StatusCode, Json};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct EditMessage {
    pub channel_id: i64,
    pub created_at: i64,
    pub content: String,
}

#[axum::debug_handler]
pub async fn message_edit(
    State(state): State<AppState>,
    Json(message): Json<EditMessage>,
) -> impl IntoResponse {
    // For now, anyone can edit any message
    // TODO: Add authorization checks
    let now = time::OffsetDateTime::now_utc().unix_timestamp();

    match sqlx::query!(
        "UPDATE message 
         SET content = ?, edited_at = ?
         WHERE channel_id = ? AND created_at = ?",
        message.content,
        now,
        message.channel_id,
        message.created_at
    )
    .execute(&state.pool)
    .await
    {
        Ok(ok) => {
            if ok.rows_affected() == 0 {
                StatusCode::NOT_FOUND.into_response()
            } else {
                StatusCode::OK.into_response()
            }
        }
        Err(e) => {
            eprintln!("Failed to edit message: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
