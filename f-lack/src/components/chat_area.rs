use rstml_to_string_macro::html;
pub fn chat_area(
    channel_name: String,
    channel_chat_content: String,
    channel_id: i64,
    logged_in_user_id: i64,
    logged_in_user_name: String,
) -> String {
    html! {

        <main class="box" style="margin-top:16px; ">
            <header>
                <h2>"# "{channel_name}</h2>
            </header>
            <section class="messages" style="flex: 1; overflow-y: auto; padding: 1rem;">
            { channel_chat_content.split('\u{001E}').into_iter().map(|message| {
                let parts: Vec<&str> = message.split('\u{001F}').collect();
                let (content, creator_id, username, timestamp, edited_at) = match parts.as_slice() {
                    [content, creator_id, username, timestamp, edited_at] =>
                        (content, creator_id, username, timestamp, edited_at),
                    _ => (&"", &"", &"", &"", &""),
                };
                html! {
                    <article id={timestamp.to_string()}>
                        <div style="display: flex; align-items: baseline; gap: 0.5rem;">
                            <strong>{username}</strong>  {if *creator_id == logged_in_user_id.to_string()
                                {
                                    html! {
                                        <>
                                            <button
                                                onclick={format!("editMessage('{}', '{}')", timestamp, content.replace("'", "\\'"))}
                                                class="button"
                                                style="font-size: 0.8rem; padding: 0.2rem 0.5rem; background-color: #007bff; color: white; transition: background-color 0.2s; cursor: pointer;"
                                                onmouseover="this.style.backgroundColor='#0056b3'"
                                                onmouseout="this.style.backgroundColor='#007bff'"
                                            >
                                                Edit
                                            </button>
                                            <button
                                                onclick={format!("deleteMessage('{}')", timestamp)}
                                                class="button"
                                                style="font-size: 0.8rem; padding: 0.2rem 0.5rem; background-color: #dc3545; color: white; transition: background-color 0.2s; cursor: pointer;"
                                                onmouseover="this.style.backgroundColor='#c82333'"
                                                onmouseout="this.style.backgroundColor='#dc3545'"
                                            >
                                                Delete
                                            </button>
                                        </>
                                    }}
                                else {
                                    "".to_string()
                                }
                            }
                            <span style="color: var(--text-secondary); font-size: 0.8rem;">
                            {if edited_at != &"" {
                                let ts = edited_at.parse::<i64>().unwrap_or(0);
                                let date = time::OffsetDateTime::from_unix_timestamp(ts).unwrap_or_else(|_| time::OffsetDateTime::now_utc());

                                format!("{} {:02}, {}, {:02}:{:02} {} (edited)",
                                    date.month().to_string()[..3].to_string(),
                                    date.day(),
                                    date.year(),
                                    if date.hour() > 12 { date.hour() - 12 } else if date.hour() == 0 { 12 } else { date.hour() },
                                    date.minute(),
                                    if date.hour() >= 12 { "p.m." } else { "a.m." }
                                )
                            } else {
                                let ts = timestamp.parse::<i64>().unwrap_or(0);
                                let date = time::OffsetDateTime::from_unix_timestamp(ts).unwrap_or_else(|_| time::OffsetDateTime::now_utc());

                                format!("{} {:02}, {}, {:02}:{:02} {}",
                                    date.month().to_string()[..3].to_string(),
                                    date.day(),
                                    date.year(),
                                    if date.hour() > 12 { date.hour() - 12 } else if date.hour() == 0 { 12 } else { date.hour() },
                                    date.minute(),
                                    if date.hour() >= 12 { "p.m." } else { "a.m." }
                                )
                            }}
                            </span>
                        </div>
                        <p>{content}</p>
                    </article>
                }
            }).collect::<Vec<String>>().join("") }

            </section>
            <footer>
                <div
                    id="drop-zone"
                    style="border: 2px dashed var(--border-color); border-radius: 4px; padding: 1rem; text-align: center; margin-bottom: 0.5rem; display: none;"
                >
                    Drop files here to upload
                </div>
                <textarea
                    id="message-input"
                    class="field"
                    placeholder="Type your message here..."
                    rows="1"
                    oninput="this.style.height = ''; this.style.height = Math.min(this.scrollHeight, 200) + 'px'"
                    style="min-height: 2.4em; resize: none; overflow-y: hidden;"
                    onkeydown="if(event.key === 'Enter' && !event.shiftKey) { event.preventDefault(); sendMessage(); }"
                ></textarea>
                <div style="display: flex; gap: 0.5rem;">

                    <button
                        onclick="sendMessage()"
                        class="button"
                        style="margin-top:8px;"
                    >
                        Send message
                    </button>
                    <button
                    onclick="toggleDropZone()"
                    class="button"
                    style="margin-top:8px;background-color: var(--bg-secondary); color: var(--text-primary); border: 1px solid var(--border-color);"
                >
                    "📎 Attach"
                </button>
                </div>
            </footer>

            {format!(r###"
                <script>
                    window.CHANNEL_ID = {};
                    window.USER_ID = {};
                    window.USERNAME = "{}";
            
                {}
                    </script>
            "###, channel_id, logged_in_user_id, logged_in_user_name, include_str!("chat_area.js"))}

        </main>
    }
}
