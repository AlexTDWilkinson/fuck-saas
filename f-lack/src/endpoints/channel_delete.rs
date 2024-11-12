use crate::AppState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;

#[axum::debug_handler]
pub async fn channel_delete(
    State(state): State<AppState>,
    Path(channel_id): Path<i64>,
) -> impl IntoResponse {
    // TODO: Check if user is admin

    // First delete all messages in the channel
    match sqlx::query!("DELETE FROM message WHERE channel_id = ?", channel_id)
        .execute(&state.pool)
        .await
    {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Failed to delete channel messages: {}", e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    }

    // Then delete the channel itself
    match sqlx::query!("DELETE FROM channel WHERE id = ?", channel_id)
        .execute(&state.pool)
        .await
    {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => {
            eprintln!("Failed to delete channel: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
