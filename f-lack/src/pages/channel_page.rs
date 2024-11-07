use crate::channel::channel::Channel;
use crate::components::chat_area::chat_area;
use crate::components::header_menu::header_menu;
use crate::components::page_shell::page_shell;
use crate::components::sidebar::sidebar;
use crate::AppState;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use rstml_to_string_macro::html;

#[axum::debug_handler]
pub async fn channel_page(
    State(state): State<AppState>,
    Path(channel_id): Path<i64>,
    _headers: axum::http::HeaderMap,
) -> impl IntoResponse {
    // todo! run the auth here to get user_id
    let user_id = 1;
    let user_name = "username_from_auth".to_string();

    let (channels, channel_chat_content) = tokio::join!(
        Channel::get_all_channels(&state.pool),
        Channel::get_channel_messages_with_users(&state.pool, channel_id)
    );

    let channel_chat_content: String = channel_chat_content.unwrap_or("".to_string());

    // print them out for debug
    println!("channel_chat_content: {}", channel_chat_content);
    println!("channel_id: {}", channel_id);
    // channel name
    let channel_name = channels.as_ref().map_or("".to_string(), |channels| {
        channels
            .iter()
            .find(|c| c.id == channel_id)
            .map_or("".to_string(), |c| c.name.clone())
    });

    let html_content = html! {
        <div style="height: 100vh;">
            {header_menu()}
            <div style="height: calc(100% - 60px); position: relative;">
                {sidebar(channels)}
                {chat_area(channel_name.clone(), channel_chat_content, channel_id, user_id, user_name)}
            </div>
        </div>
    };

    let shelled_content = page_shell(channel_name, html_content, "".to_string(), "".to_string());

    axum::http::Response::builder()
        .header(
            axum::http::header::CACHE_CONTROL,
            "no-cache, no-store, must-revalidate",
        )
        .header(axum::http::header::PRAGMA, "no-cache")
        .header(axum::http::header::EXPIRES, "0")
        .header(axum::http::header::CONTENT_TYPE, "text/html")
        .body(shelled_content)
        .expect("Failed to render chat page")
}
