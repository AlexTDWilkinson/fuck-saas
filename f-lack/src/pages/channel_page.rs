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
    let (channels, channel_chat_content) = tokio::join!(
        Channel::get_all_channels(&state.pool),
        Channel::get_channel_messages_with_users(&state.pool, channel_id)
    );

    let channel_chat_content: String = channel_chat_content.unwrap_or("".to_string());

    let html_content = html! {
        <div style="height: 100vh;">
            {header_menu()}
            <div style="height: calc(100% - 60px); position: relative;">
                {sidebar(channels)}
                {chat_area(channel_chat_content, channel_id)}
            </div>
        </div>
    };

    let shelled_content = page_shell(
        "Flack Chat".to_string(),
        html_content,
        "".to_string(),
        "".to_string(),
    );

    axum::http::Response::builder()
        .header(axum::http::header::CACHE_CONTROL, "public, max-age=86400")
        .header(axum::http::header::CONTENT_TYPE, "text/html")
        .body(shelled_content)
        .expect("Failed to render chat page")
}
