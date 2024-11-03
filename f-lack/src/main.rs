pub mod auth;
pub mod channel;
pub mod components;
pub mod error_template;
pub mod fileserv;
pub mod message;
pub mod pages;

use crate::auth::auth::{login, logout, signup};
use crate::fileserv::file_and_error_handler;
use crate::pages::channel_page::channel_page;
use crate::pages::home::home;
use dotenv::dotenv;

// use crate::pages::admin_page::admin_page;
// use crate::pages::channel::channel;
// use crate::pages::direct_messages::direct_messages;
// use crate::pages::home_page::home_page;
// use crate::pages::user_settings::user_settings;
// use crate::pages::workspace_settings::workspace_settings;

use axum::{
    routing::{get, post},
    Router,
};
use sqlx::sqlite::SqlitePoolOptions;
use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

#[derive(Clone)]
pub struct AppState {
    pub pool: sqlx::SqlitePool,
}

#[tokio::main]
async fn main() {
    let _ = match dotenv() {
        Ok(_) => println!(".env loaded"),
        Err(err) => println!("Missing .env file: {:?}", err),
    };

    let db_path = format!(
        "sqlite:{}/db/flack.db",
        env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not found")
    );

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_path)
        .await
        .expect("Could not make pool.");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to migrate database");

    let app_state = AppState { pool: pool.clone() };

    let app = Router::new()
        // Auth routes
        // .route("/login", get(login_page).post(login))
        // .route("/signup", get(signup_page).post(signup))
        .route("/logout", get(logout))
        // Main pages
        .route("/channel/:channel_id", get(channel_page))
        .route("/", get(home))
        // .route("/channel/:channel_name", get(channel))
        // .route("/dm/:user_id", get(direct_messages))
        // .route("/settings", get(user_settings))
        // .route("/workspace/settings", get(workspace_settings))
        // Fallback
        .fallback(file_and_error_handler)
        .with_state(app_state);

    let port = env::var("PORT").map_or(3000, |var| var.parse::<u16>().unwrap());
    let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port);

    println!("listening on {}", address);
    axum::serve(
        tokio::net::TcpListener::bind(&address)
            .await
            .expect("Failed to bind"),
        app.into_make_service(),
    )
    .await
    .expect("Server failed to start");
}
