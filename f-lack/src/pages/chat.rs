use crate::components::page_shell::page_shell;

use axum::http::HeaderMap;
use axum::response::IntoResponse;
use rstml_to_string_macro::html;

fn header_menu() -> String {
    html! {
        <header class="menu">
            <div style="display: flex; justify-content: space-between; align-items: center; gap: 2rem; width: 100%;">
                <h1 style="font-size: 1.5rem; font-weight: bold; flex-shrink: 0;">F-lack</h1>
                <div style="flex: 1; max-width: 600px;">
                    <input
                        type="search"
                        placeholder="Search channel..."
                        class="field"
                        style="width: 100%;"
                    />
                </div>
                <nav style="flex-shrink: 0;">
                    <ul style="display: flex; gap: 1rem; list-style: none;">
                        <li><a href="/settings" class="link">Settings</a></li>
                        <li><a href="/logout" class="link">Logout</a></li>
                    </ul>
                </nav>
            </div>
        </header>
    }
}

fn sidebar() -> String {
    html! {
        <aside class="box" style="margin-top:16px;">
            <h3>Channels</h3>
            <nav>
                <ul>
                    <li><a href="#" class="link"># general</a></li>
                    <li><a href="#" class="link"># random</a></li>
                    <li><a href="#" class="link"># announcements</a></li>
                </ul>
            </nav>

        </aside>
    }
}

fn chat_area() -> String {
    html! {
            <main class="box" style="margin-top:16px; ">
                <header>
                    <h2># general</h2>
                </header>
                <section class="messages" style="flex: 1; overflow-y: auto; padding: 1rem;">
                    <article >
                        <div style="display: flex; align-items: baseline; gap: 0.5rem;">
                            <strong>John Doe</strong>
                            <span style="color: var(--text-secondary); font-size: 0.8rem;">11:23 AM</span>
                        </div>
                        <p>{"Hello everyone! 👋"}</p>
                    </article>
                    <article>
                        <div style="display: flex; align-items: baseline; gap: 0.5rem;">
                            <strong>Jane Smith</strong>
                            <span style="color: var(--text-secondary); font-size: 0.8rem;">11:24 AM</span>
                        </div>
                        <p>Hi John! How are you?</p>
                    </article>             <article >
                    <div style="display: flex; align-items: baseline; gap: 0.5rem;">
                        <strong>John Doe</strong>
                        <span style="color: var(--text-secondary); font-size: 0.8rem;">11:23 AM</span>
                    </div>
                    <p>{"Hello everyone! 👋"}</p>
                </article>
                <article>
                    <div style="display: flex; align-items: baseline; gap: 0.5rem;">
                        <strong>Jane Smith</strong>
                        <span style="color: var(--text-secondary); font-size: 0.8rem;">11:24 AM</span>
                    </div>
                    <p>Hi John! How are you?</p>
                </article>             <article >
                <div style="display: flex; align-items: baseline; gap: 0.5rem;">
                    <strong>John Doe</strong>
                    <span style="color: var(--text-secondary); font-size: 0.8rem;">11:23 AM</span>
                </div>
                <p>{"Hello everyone! 👋"}</p>
            </article>
            <article>
                <div style="display: flex; align-items: baseline; gap: 0.5rem;">
                    <strong>Jane Smith</strong>
                    <span style="color: var(--text-secondary); font-size: 0.8rem;">11:24 AM</span>
                </div>
                <p>Hi John! How are you?</p>
            </article>             <article >
            <div style="display: flex; align-items: baseline; gap: 0.5rem;">
                <strong>John Doe</strong>
                <span style="color: var(--text-secondary); font-size: 0.8rem;">11:23 AM</span>
            </div>
            <p>{"Hello everyone! 👋"}</p>
        </article>
        <article>
            <div style="display: flex; align-items: baseline; gap: 0.5rem;">
                <strong>Jane Smith</strong>
                <span style="color: var(--text-secondary); font-size: 0.8rem;">11:24 AM</span>
            </div>
            <p>Hi John! How are you?</p>
        </article>             <article >
        <div style="display: flex; align-items: baseline; gap: 0.5rem;">
            <strong>John Doe</strong>
            <span style="color: var(--text-secondary); font-size: 0.8rem;">11:23 AM</span>
        </div>
        <p>{"Hello everyone! 👋"}</p>
    </article>
    <article>
        <div style="display: flex; align-items: baseline; gap: 0.5rem;">
            <strong>Jane Smith</strong>
            <span style="color: var(--text-secondary); font-size: 0.8rem;">11:24 AM</span>
        </div>
        <p>Hi John! How are you?</p>
    </article>             <article >
    <div style="display: flex; align-items: baseline; gap: 0.5rem;">
        <strong>John Doe</strong>
        <span style="color: var(--text-secondary); font-size: 0.8rem;">11:23 AM</span>
    </div>
    <p>{"Hello everyone! 👋"}</p>
    </article>
    <article>
    <div style="display: flex; align-items: baseline; gap: 0.5rem;">
        <strong>Jane Smith</strong>
        <span style="color: var(--text-secondary); font-size: 0.8rem;">11:24 AM</span>
    </div>
    <p>Hi John! How are you?</p>
    </article>             <article >
    <div style="display: flex; align-items: baseline; gap: 0.5rem;">
        <strong>John Doe</strong>
        <span style="color: var(--text-secondary); font-size: 0.8rem;">11:23 AM</span>
    </div>
    <p>{"Hello everyone! 👋"}</p>
    </article>
    <article>
    <div style="display: flex; align-items: baseline; gap: 0.5rem;">
        <strong>Jane Smith</strong>
        <span style="color: var(--text-secondary); font-size: 0.8rem;">11:24 AM</span>
    </div>
    <p>Hi John! How are you?</p>
    </article>             <article >
    <div style="display: flex; align-items: baseline; gap: 0.5rem;">
        <strong>John Doe</strong>
        <span style="color: var(--text-secondary); font-size: 0.8rem;">11:23 AM</span>
    </div>
    <p>{"Hello everyone! 👋"}</p>
    </article>
    <article>
    <div style="display: flex; align-items: baseline; gap: 0.5rem;">
        <strong>Jane Smith</strong>
        <span style="color: var(--text-secondary); font-size: 0.8rem;">11:24 AM</span>
    </div>
    <p>Hi John! How are you?</p>
    </article>
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

#[axum::debug_handler]
pub async fn chat(headers: axum::http::HeaderMap) -> impl IntoResponse {
    let html_content = html! {
        <div style="height: 100vh;">
            {header_menu()}
            <div style="height: calc(100% - 60px); position: relative;">
                {sidebar()}
                {chat_area()}
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
