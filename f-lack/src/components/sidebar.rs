use crate::channel::channel::Channel;
use rstml_to_string_macro::html;

pub fn sidebar(channels: Option<Vec<Channel>>) -> String {
    match channels {
        None => sidebar_error(),
        Some(channels) => sidebar_success(channels),
    }
}

fn sidebar_success(channels: Vec<Channel>) -> String {
    html! {
        <aside class="box" style="margin-top:16px;">
            <h3>"Channels"</h3>
            <nav>
                <ul>
    {
        channels.iter()
        .map(|channel| html! {
            <li><a href={format!("/channel/{}", channel.id)} class="link"># { channel.name.clone() } </a></li>
        }).collect::<Vec<String>>()
        .join("")
    }
                </ul>
            </nav>
            <button
                onclick="createChannel()"
                class="button"
                style="margin-top: 1rem; width: 100%;"
            >
                "Create channel"
            </button>

           {r#" <script>
                function createChannel() {
                    const name = prompt("Enter channel name:");
                    if (!name) return;
                    
                    fetch('/api/channels/create', {
                        method: 'POST',
                        headers: { 'Content-Type': 'application/json' },
                        body: JSON.stringify({ name })
                    })
                    .then(response => {
                        if (response.ok) {
                            window.location.reload();
                        } else {
                            alert('Failed to create channel');
                        }
                    });
                }
            </script>
            "#}
        </aside>
    }
}

fn sidebar_error() -> String {
    html! {
        <aside class="box" style="margin-top:16px;">
           <h3>"Channels"</h3>
       <b>"Error loading channels"</b>
        </aside>
    }
}
