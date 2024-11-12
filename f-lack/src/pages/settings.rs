use crate::channel::channel::Channel;
use crate::components::header_menu::header_menu;
use crate::components::page_shell::page_shell;
use crate::user::user::User;
use crate::AppState;
use axum::extract::{Query, State};
use axum::response::IntoResponse;
use rstml_to_string_macro::html;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SettingsQuery {
    error: Option<String>,
    success: Option<String>,
}

#[axum::debug_handler]
pub async fn settings(
    State(state): State<AppState>,
    Query(params): Query<SettingsQuery>,
) -> impl IntoResponse {
    // TODO: Get user info from session
    let user_name = "username_from_auth".to_string();

    let (channels_with_users, all_users): (Option<Vec<(Channel, Vec<User>)>>, Option<Vec<User>>) = tokio::join!(
        Channel::get_all_channels_with_users(&state.pool),
        User::get_all_users(&state.pool),
    );

    let content = html! {
        <div style="height: 100vh;">
            {header_menu()}
            <datalist id="usernames">
                {if let Some(users) = all_users {
                    html! {
                        {users.iter().map(|user| {
                            html! {
                                <option value={&user.username} />
                            }
                        }).collect::<Vec<_>>().join("")}
                    }
                } else {
                    html! {}
                }}
            </datalist>
            <div class="settings-container" style="padding: 2rem;">
                {if let Some(error) = params.error {
                    html! {
                        <div class="error-message" style="color: red; margin-bottom: 1rem;">
                            {error}
                        </div>
                    }
                } else {
                    html! {}
                }}

                {if let Some(success) = params.success {
                    html! {
                        <div class="success-message" style="color: green; margin-bottom: 1rem;">
                            {success}
                        </div>
                    }
                } else {
                    html! {}
                }}

                <h1 style="margin-bottom: 2rem;">{"Settings"}</h1>

                <div class="settings-section">
                    <h2>{"Profile settings"}</h2>
                    <form action="/api/settings/profile" method="POST" enctype="multipart/form-data">
                        <div style="margin-bottom: 1rem;">
                            <label style="display: block; margin-bottom: 0.5rem;">{"Display Name:"}</label>
                            <input
                                type="text"
                                name="display_name"
                                value={user_name}
                                style="padding: 0.5rem; width: 300px;"
                            />
                        </div>
                        <div style="margin-bottom: 1rem;">
                            <label style="display: block; margin-bottom: 0.5rem;">{"Avatar:"}</label>
                            <input
                                type="file"
                                name="avatar"
                                accept="image/*"
                                style="margin-bottom: 1rem;"
                            />
                        </div>
                        <button
                            type="submit"
                            style="padding: 0.5rem 1rem; background: #007bff; color: white; border: none; border-radius: 4px; cursor: pointer;"
                        >
                            {"Update profile"}
                        </button>
                    </form>
                </div>

                <div class="settings-section" style="margin-top: 2rem;">
                    <h2>{"Channel management"}</h2>
                    <div id="channels" style="margin-top: 1rem;">
                        {if let Some(channel_list) = channels_with_users {
                            html! {
                                <div>
                                    {channel_list.iter().map(|(channel, users)| {
                                        html! {
                                            <div style="margin-bottom: 1.5rem; padding: 1rem; border: 1px solid #eee; border-radius: 4px;">
                                                <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 1rem;">
                                                    <span style="font-weight: bold;">{&channel.name}</span>
                                                    <button
                                                        onclick={format!("deleteChannel({})", channel.id)}
                                                        style="padding: 0.25rem 0.5rem; background: #dc3545; color: white; border: none; border-radius: 4px; cursor: pointer;"
                                                    >
                                                        {"Delete Channel"}
                                                    </button>
                                                </div>

                                                <div style="margin-top: 0.5rem;">
                                                    <h4>{"Manage users"}</h4>
                                                    <div style="display: flex; gap: 1rem; margin-top: 0.5rem;">
                                                        <input
                                                            type="text"
                                                            id={format!("userInput_{}", channel.id)}
                                                            placeholder="Enter username"
                                                            list="usernames"
                                                            style="padding: 0.5rem; flex-grow: 1;"
                                                        />
                                                        <button
                                                            onclick={format!("addUserToChannel({}, document.getElementById('userInput_{}').value)", channel.id, channel.id)}
                                                            style="padding: 0.5rem 1rem; background: #28a745; color: white; border: none; border-radius: 4px; cursor: pointer;"
                                                        >
                                                            {"Add User"}
                                                        </button>
                                                    </div>
                                                    <div style="margin-top: 1rem;">
                                                        {if users.is_empty() {
                                                            html! {
                                                                <div>{"No users in this channel"}</div>
                                                            }
                                                        } else {
                                                            html! {
                                                                <div>
                                                                    {users.iter().map(|user| {
                                                                        let username = &user.username;
                                                                        html! {
                                                                            <div style="display: flex; justify-content: space-between; align-items: center; padding: 0.5rem; border-bottom: 1px solid #eee;">
                                                                                <span>{username}</span>
                                                                                <button
                                                                                    onclick={format!("removeUserFromChannel({}, '{}')", channel.id, username)}
                                                                                    style="padding: 0.25rem 0.5rem; background: #dc3545; color: white; border: none; border-radius: 4px; cursor: pointer;"
                                                                                >
                                                                                    {"Remove"}
                                                                                </button>
                                                                            </div>
                                                                        }
                                                                    }).collect::<Vec<_>>().join("")}
                                                                </div>
                                                            }
                                                        }}
                                                    </div>
                                                </div>
                                            </div>
                                        }
                                    }).collect::<Vec<_>>().join("")}
                                </div>
                            }
                        } else {
                            html! {
                                <div>{"Failed to load channels"}</div>
                            }
                        }}
                    </div>
                </div>

                {r#" <script>
                    function deleteChannel(id) {
                        if (!confirm('Are you sure you want to delete this channel?')) return;
                        
                        fetch(`/api/channels/delete/${id}`, {
                            method: 'POST'
                        })
                        .then(r => {
                            if (r.ok) location.reload();
                            else alert('Failed to delete channel');
                        });
                    }

                    function addUserToChannel(channelId, username) {
                        if (!username.trim()) {
                            alert('Please enter a username');
                            return;
                        }

                        fetch(`/api/channels/${channelId}/users`, {
                            method: 'POST',
                            headers: {
                                'Content-Type': 'application/json'
                            },
                            body: JSON.stringify({ username })
                        })
                        .then(r => {
                            if (r.ok) {
                                location.reload();
                            } else {
                                alert('Failed to add user to channel');
                            }
                        });
                    }

                    function removeUserFromChannel(channelId, username) {
                        if (!confirm(`Are you sure you want to remove ${username} from this channel?`)) return;

                        fetch(`/api/channels/${channelId}/users/${username}`, {
                            method: 'DELETE'
                        })
                        .then(r => {
                            if (r.ok) {
                                location.reload();
                            } else {
                                alert('Failed to remove user from channel');
                            }
                        });
                    }
                </script> "#}
            </div>
        </div>
    };

    let shelled_content = page_shell(
        "Settings".to_string(),
        content,
        "".to_string(),
        "".to_string(),
    );

    axum::http::Response::builder()
        .header(
            axum::http::header::CACHE_CONTROL,
            "no-cache, no-store, must-revalidate",
        )
        .header(axum::http::header::PRAGMA, "no-cache")
        .header(axum::http::header::EXPIRES, "0")
        .header(axum::http::header::CONTENT_TYPE, "text/html")
        .body(shelled_content)
        .expect("Failed to render settings page")
}
