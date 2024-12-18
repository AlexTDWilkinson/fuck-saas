use crate::AppState;
use axum::response::IntoResponse;
use axum::{extract::State, http::StatusCode, Json};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct DeleteMessage {
    pub channel_id: i64,
    pub created_at: i64,
}

#[axum::debug_handler]
pub async fn message_delete(
    State(state): State<AppState>,
    Json(message): Json<DeleteMessage>,
) -> impl IntoResponse {
    // For now, anyone can delete any message
    // TODO: Add authorization checks

    // Parse the timestamp to match database format

    let timestamp = message.created_at;

    match sqlx::query!(
        "DELETE FROM message 
         WHERE channel_id = ? AND created_at = ?",
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
                println!("Message deleted with timestamp: {}", timestamp);
                StatusCode::OK.into_response()
            }
        }
        Err(e) => {
            eprintln!("Failed to delete message: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
