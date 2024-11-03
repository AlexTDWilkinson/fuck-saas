use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::time::Instant;
use time::OffsetDateTime;

#[derive(sqlx::FromRow, Clone, Debug, Serialize, Deserialize)]
pub struct Channel {
    pub id: i64,
    pub name: String,
    pub created_at: Option<OffsetDateTime>,
}

#[derive(sqlx::FromRow, Clone, Debug, Serialize, Deserialize)]
struct MessageWithUser {
    pub content: String,
    pub creator_id: i64,
    pub username: String,
    pub created_at: Option<OffsetDateTime>,
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

    pub async fn get_channel_messages_with_users(
        pool: &SqlitePool,
        channel_id: i64,
    ) -> Option<String> {
        let start = Instant::now();

        let messages = sqlx::query_as!(
            MessageWithUser,
            r#"
                SELECT 
                    m.content,
                    m.creator_id,
                    a.username as "username!", 
                    m.created_at
                FROM message m
                JOIN account a ON m.creator_id = a.id
                WHERE m.channel_id = ?
                ORDER BY m.created_at ASC
                LIMIT 100
                "#,
            channel_id
        )
        .fetch_all(pool)
        .await;

        let duration = start.elapsed();
        println!("Query execution time: {:?}", duration);

        // Add debug logging
        match &messages {
            Ok(msgs) => println!("Found {} messages", msgs.len()),
            Err(e) => println!("Query error: {}", e),
        }
        let now = OffsetDateTime::now_utc();

        match messages {
            Ok(messages) => Some(
                messages
                    .into_iter()
                    .map(|m| {
                        let timestamp = m.created_at.unwrap_or(OffsetDateTime::now_utc());

                        format!(
                            "{}\u{001F}{}\u{001F}{}\u{001F}{}",
                            m.content, m.creator_id, m.username, timestamp
                        )
                    })
                    .collect::<Vec<String>>()
                    .join("\u{001E}"),
            ),
            Err(err) => {
                eprintln!("get channel messages with users error: {}", err);
                None
            }
        }
    }
}
