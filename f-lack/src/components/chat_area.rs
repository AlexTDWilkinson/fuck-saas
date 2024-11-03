use rstml_to_string_macro::html;
pub fn chat_area(channel_chat_content: String, channel_id: i64) -> String {
    html! {
        <main class="box" style="margin-top:16px; ">
            <header>
                <h2># general</h2>
            </header>
            <section class="messages" style="flex: 1; overflow-y: auto; padding: 1rem;">
            { channel_chat_content.split('\u{001E}').into_iter().map(|message| {
                let parts: Vec<&str> = message.split('\u{001F}').collect();
                let (content, creator_id, username, timestamp) = match parts.as_slice() {
                    [content, creator_id, username, timestamp] => (content, creator_id, username, timestamp),
                    _ => (&"", &"", &"", &""),
                };
                html! {
                    <article id={timestamp.to_string()}>
                        <div style="display: flex; align-items: baseline; gap: 0.5rem;">
                            <strong>{username}</strong>
                            <span style="color: var(--text-secondary); font-size: 0.8rem;">{timestamp}</span>
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


        {format!(r###"  <script defer>
            const dropZone = document.getElementById('drop-zone');
            const messageInput = document.getElementById('message-input');
            const channelId = {};  // Will interpolate channel_id here

            // Message sending
            async function sendMessage() {{
                const message = messageInput.value.trim();
                if (!message) return;

                try {{
                    const response = await fetch('/api/messages', {{
                        method: 'POST',
                        headers: {{'Content-Type': 'application/json'}},
                        body: JSON.stringify({{
                            channel_id: channelId,
                            content: message
                        }})
                    }});

                    if (response.ok) {{
                        messageInput.value = '';
                        // TODO: Refresh messages
                    }}
                }} catch (error) {{
                    console.error('Error sending message:', error);
                }}
            }}

            // File upload handling
            function toggleDropZone() {{
                dropZone.style.display = dropZone.style.display === 'none' ? 'block' : 'none';
            }}

            function handleDragEvent(e, highlight) {{
                e.preventDefault();
                e.stopPropagation();
                dropZone.style.borderColor = highlight ? 'var(--accent-color)' : 'var(--border-color)';
            }}

            dropZone.addEventListener('dragover', e => handleDragEvent(e, true));
            dropZone.addEventListener('dragleave', e => handleDragEvent(e, false));
            dropZone.addEventListener('drop', e => {{
                handleDragEvent(e, false);
                const files = e.dataTransfer.files;
                if (files.length > 0) {{
                    // TODO: Implement file upload
                    console.log('Files dropped:', files);
                }}
            }});  </script>
        "###, channel_id)}


        </main>
    }
}
