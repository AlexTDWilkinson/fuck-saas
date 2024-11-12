use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

#[derive(sqlx::FromRow, Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub created_at: i64,
    pub permissions: String,
    pub set_password_mode: bool,
    pub set_password_pin: Option<i64>,
    pub set_password_attempts: Option<i64>,
    pub user_disabled: bool,
    pub user_deleted: bool,
}

impl User {
    pub async fn get_user_by_id(id: i64, pool: &SqlitePool) -> Option<Self> {
        let user = sqlx::query_as!(
            User,
            "SELECT * FROM account WHERE id = ? AND user_deleted = 0",
            id
        )
        .fetch_optional(pool)
        .await;

        if let Err(err) = user {
            eprintln!("get user by id error: {:?}", err);
            return None;
        }

        match user {
            Ok(user) => user,
            _ => None,
        }
    }

    pub async fn get_all_users(pool: &SqlitePool) -> Option<Vec<Self>> {
        let users = sqlx::query_as!(User, "SELECT * FROM account WHERE user_deleted = 0")
            .fetch_all(pool)
            .await;

        if let Err(err) = users {
            eprintln!("get all users error: {:?}", err);
            return None;
        }

        match users {
            Ok(users) => Some(users),
            _ => None,
        }
    }
}
