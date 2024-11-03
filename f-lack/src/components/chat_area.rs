use rstml_to_string_macro::html;
pub fn chat_area(channel_chat_content: Vec<String>, channel_id: i64) -> String {
    html! {
        <main class="box" style="margin-top:16px; ">
            <header>
                <h2># general</h2>
            </header>
            <section class="messages" style="flex: 1; overflow-y: auto; padding: 1rem;">
                { channel_chat_content.iter().map(|message| html! {
                    <article>
                        <div style="display: flex; align-items: baseline; gap: 0.5rem;">
                            <strong>{"User"}</strong>
                            <span style="color: var(--text-secondary); font-size: 0.8rem;">{"Time"}</span>
                        </div>
                        <p>{message}</p>
                    </article>
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
            {r#"<script>
                function toggleDropZone() {
                    const dropZone = document.getElementById('drop-zone');
                    dropZone.style.display = dropZone.style.display === 'none' ? 'block' : 'none';
                }

                function sendMessage() {
                    const input = document.getElementById('message-input');
                    const message = input.value;
                    if (message.trim()) {
                        // Here you would typically send the message to your server
                        console.log('Sending message:', message);
                        input.value = '';
                    }
                }

                const dropZone = document.getElementById('drop-zone');

                dropZone.addEventListener('dragover', (e) => {
                    e.preventDefault();
                    e.stopPropagation();
                    dropZone.style.borderColor = 'var(--accent-color)';
                });

                dropZone.addEventListener('dragleave', (e) => {
                    e.preventDefault();
                    e.stopPropagation();
                    dropZone.style.borderColor = 'var(--border-color)';
                });

                dropZone.addEventListener('drop', (e) => {
                    e.preventDefault();
                    e.stopPropagation();
                    dropZone.style.borderColor = 'var(--border-color)';
                    
                    const files = e.dataTransfer.files;
                    if (files.length > 0) {
                        // Here you would typically handle the file upload
                        console.log('Files dropped:', files);
                    }
                });
            </script>"#}
        </main>
    }
}
