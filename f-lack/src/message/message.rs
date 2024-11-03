use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use time::OffsetDateTime;

#[derive(sqlx::FromRow, Clone, Debug, Serialize, Deserialize)]
pub struct Message {
    pub created_at: Option<OffsetDateTime>,
    // The creator and created_at values are necessary for fast
    // metadata and also searching by datetimes.
    pub creator_id: i64,
    // pub created_at: OffsetDateTime,

    // A messages unique id is composite of channel_id, message_id
    // message_ids are sequential and go from 0 upwards per
    // channel_id.  Ordering is easy.
    pub channel_id: i64,
    pub message_index: i64,

    // The rest of the data (message state, including history, deleted
    // status, links, metadata for plugins, embedded images, etc) is
    // encoded as content.  This makes it invisible to the database,
    // but also very flexible.
    pub content: String,
}

impl Message {
    pub async fn get_by_channel_id_and_message_index(
        channel_id: i64,
        message_index: i64,
        pool: &SqlitePool,
    ) -> Option<Self> {
        let server_message = sqlx::query_as!(
            Message,
            "SELECT * FROM message WHERE channel_id = ? AND message_index = ?",
            channel_id,
            message_index
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
