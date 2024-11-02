use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

#[derive(sqlx::FromRow, Clone, Debug, Serialize, Deserialize)]
pub struct Channel {
    // Channels are just an id and a name, no metadata about creation times
    // or whom or what at this point.
    pub id: i64,
    pub name: String,
    // pub created_at: OffsetDateTime,
}

impl Channel {
    pub async fn get_channel_by_id(id: i64, pool: &SqlitePool) -> Option<Self> {
        let server_channel = sqlx::query_as!(Channel, "SELECT * FROM channel WHERE id = ?", id)
            .fetch_optional(pool)
            .await;

        if let Err(err) = server_channel {
            println!("get by id error {:?}", err);
            return None;
        }

        match server_channel {
            Ok(Some(server_channel)) => Some(server_channel),
            _ => None,
        }
    }

    pub async fn get_all_channels(pool: &SqlitePool) -> Option<Vec<Self>> {
        let server_channels = sqlx::query_as!(Channel, "SELECT * FROM channel ORDER BY name")
            .fetch_all(pool)
            .await;

        if let Err(err) = server_channels {
            println!("get all channels error {:?}", err);
            return None;
        }

        match server_channels {
            Ok(server_channels) => Some(server_channels),
            _ => None,
        }
    }
}
