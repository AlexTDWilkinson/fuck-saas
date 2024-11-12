use crate::message::message::MessageWithUser;
use crate::user::user::User;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::collections::HashMap;
use std::time::Instant;

#[derive(sqlx::FromRow, Clone, Debug, Serialize, Deserialize)]
pub struct Channel {
    pub id: i64,
    pub name: String,
    pub created_at: i64,
}

impl Channel {
    pub async fn get_channel_by_id(id: i64, pool: &SqlitePool) -> Option<Self> {
        let server_channel = sqlx::query_as!(
            Channel,
            "SELECT id, name, created_at FROM channel WHERE id = ?",
            id
        )
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
        let server_channels = sqlx::query_as!(
            Channel,
            "SELECT id, name, created_at FROM channel ORDER BY name"
        )
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

    pub async fn get_all_channels_with_users(pool: &SqlitePool) -> Option<Vec<(Self, Vec<User>)>> {
        // Get all channels with their users in a single query
        let results = sqlx::query!(
            r#"
            SELECT 
                c.id as channel_id,
                c.name as channel_name,
                c.created_at as channel_created_at,
                a.id as user_id,
                a.username as username,
                a.email as email,
                a.password_hash as password_hash,
                a.created_at as user_created_at,
                a.permissions as permissions,
                a.set_password_mode as set_password_mode,
                a.set_password_pin as set_password_pin,
                a.set_password_attempts as set_password_attempts,
                a.user_disabled as user_disabled,
                a.user_deleted as user_deleted
            FROM channel c
            LEFT JOIN channel_user cu ON c.id = cu.channel_id
            LEFT JOIN account a ON cu.user_id = a.id
            WHERE a.user_deleted = 0 OR a.id IS NULL
            ORDER BY c.name, a.username
            "#
        )
        .fetch_all(pool)
        .await
        .map_err(|err| {
            eprintln!("get all channels with users error: {:?}", err);
            err
        })
        .ok()?;

        let mut channel_map: std::collections::HashMap<i64, (Channel, Vec<User>)> = HashMap::new();

        for row in results {
            let channel = Channel {
                id: row.channel_id,
                name: row.channel_name,
                created_at: row.channel_created_at,
            };

            let entry = channel_map
                .entry(row.channel_id)
                .or_insert_with(|| (channel, Vec::new()));

            entry.1.push(User {
                id: row.user_id,
                username: row.username.unwrap_or_default(),
                email: row.email.unwrap_or_default(),
                password_hash: row.password_hash.unwrap_or_default(),
                created_at: row.user_created_at.unwrap_or_default(),
                permissions: row.permissions.unwrap_or_default(),
                set_password_mode: row.set_password_mode.unwrap_or_default(),
                set_password_pin: row.set_password_pin,
                set_password_attempts: row.set_password_attempts,
                user_disabled: row.user_disabled.unwrap_or_default(),
                user_deleted: row.user_deleted.unwrap_or_default(),
            });
        }

        // sort for UI stability
        let mut channels: Vec<_> = channel_map.into_values().collect();
        channels.sort_by(|a, b| a.0.name.cmp(&b.0.name));
        Some(channels)
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
                    m.created_at,
                    m.edited_at
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

        // // Add debug logging
        // match &messages {
        //     Ok(msgs) => println!("Found {} messages", msgs.len()),
        //     Err(e) => println!("Query error: {}", e),
        // }

        match messages {
            Ok(messages) => Some(
                messages
                    .into_iter()
                    .map(|m| {
                        format!(
                            "{}\u{001F}{}\u{001F}{}\u{001F}{}\u{001F}{}",
                            m.content,
                            m.creator_id,
                            m.username,
                            m.created_at,
                            m.edited_at.map_or("".to_string(), |dt| dt.to_string())
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
