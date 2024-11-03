use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use time::OffsetDateTime;

#[derive(sqlx::FromRow, Clone, Debug, Serialize, Deserialize)]
pub struct Message {
    pub created_at: Option<OffsetDateTime>, // Changed to non-optional since it's part of primary key
    // The creator and created_at values are necessary for fast
    // metadata and also searching by datetimes.
    pub creator_id: i64,

    // A message's unique id is composite of channel_id and created_at timestamp
    // This provides natural chronological ordering
    pub channel_id: i64,

    // The rest of the data (message state, including history, deleted
    // status, links, metadata for plugins, embedded images, etc) is
    // encoded as content. This makes it invisible to the database,
    // but also very flexible.
    pub content: String,
    pub edited_at: Option<OffsetDateTime>,
}

impl Message {
    pub async fn get_by_channel_id_and_timestamp(
        channel_id: i64,
        timestamp: OffsetDateTime,
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
