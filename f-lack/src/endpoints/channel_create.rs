use crate::AppState;
use axum::response::IntoResponse;
use axum::{extract::State, http::StatusCode, Json};
use serde::Deserialize;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Deserialize, Debug)]
pub struct CreateChannel {
    pub name: String,
}

#[axum::debug_handler]
pub async fn channel_create(
    State(state): State<AppState>,
    Json(channel): Json<CreateChannel>,
) -> impl IntoResponse {
    println!("channel create {:?}", channel);
    // Basic validation
    let name = channel.name.trim();

    match sqlx::query!("INSERT INTO channel (name) VALUES (?)", name)
        .execute(&state.pool)
        .await
    {
        Ok(db_resp) => {
            println!("db_resp: {:?}", db_resp);
            StatusCode::CREATED.into_response()
        }
        Err(e) => {
            eprintln!("Failed to create channel: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
