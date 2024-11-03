use crate::AppState;
use axum::response::IntoResponse;
use axum::{extract::State, http::StatusCode, Json};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct DeleteMessage {
    pub channel_id: i64,
    pub created_at: String,
}

#[axum::debug_handler]
pub async fn message_delete(
    State(state): State<AppState>,
    Json(message): Json<DeleteMessage>,
) -> impl IntoResponse {
    // For now, anyone can delete any message
    // TODO: Add authorization checks

    // Parse the timestamp to match database format
    // Dunno why we have to do this, should be refactored 'cause this is probably not necessary at all
    let timestamp = message
        .created_at
        .split(" +")
        .next()
        .unwrap_or(&message.created_at)
        .replace("T", " "); // Convert ISO timestamp format to match SQLite format

    match sqlx::query!(
        "DELETE FROM message 
         WHERE channel_id = ? AND created_at = datetime(?)",
        message.channel_id,
        timestamp
    )
    .execute(&state.pool)
    .await
    {
        Ok(ok) => {
            println!("Delete result: {:?}", ok);
            if ok.rows_affected() == 0 {
                println!("No message found to delete with timestamp: {}", timestamp);
                StatusCode::NOT_FOUND.into_response()
            } else {
                StatusCode::OK.into_response()
            }
        }
        Err(e) => {
            eprintln!("Failed to delete message: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
