use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use time::OffsetDateTime;

#[derive(sqlx::FromRow, Clone, Debug, Serialize, Deserialize)]
pub struct Message {
    pub created_at: i64, // Unix timestamp in milliseconds
    pub creator_id: i64,
    pub channel_id: i64,
    pub content: String,
    pub edited_at: Option<i64>, // Unix timestamp in milliseconds
}

impl Message {
    pub async fn get_by_channel_id_and_timestamp(
        channel_id: i64,
        timestamp: i64,
        pool: &SqlitePool,
    ) -> Option<Self> {
        let server_message = sqlx::query_as!(
            Message,
            "SELECT created_at, creator_id, channel_id, content, edited_at FROM message WHERE channel_id = ? AND created_at = ?",
            channel_id,
            timestamp
        )
        .fetch_optional(pool)
        .await;

        if let Err(err) = server_message {
            println!("get by id error {:?}", err);
            return None;
        }

        match server_message {
            Ok(Some(server_message)) => Some(server_message),
            _ => None,
        }
    }
}

#[derive(sqlx::FromRow, Clone, Debug, Serialize, Deserialize)]
pub struct MessageWithUser {
    pub content: String,
    pub creator_id: i64,
    pub username: String,
    pub created_at: i64,
    pub edited_at: Option<i64>,
}
