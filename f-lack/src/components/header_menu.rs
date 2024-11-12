use rstml_to_string_macro::html;
pub fn header_menu() -> String {
    html! {
        <header class="menu">
            <div style="display: flex; justify-content: space-between; align-items: center; gap: 2rem; width: 100%;">
                <a href="/" style="text-decoration: none; color: inherit;">
                    <h1 style="font-size: 1.5rem; font-weight: bold; flex-shrink: 0;">"F-lack"</h1>
                </a>
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
                        <li><a href="/settings" class="link">"Settings"</a></li>
                        <li><a href="/logout" class="link">"Logout"</a></li>
                    </ul>
                </nav>
            </div>
        </header>
    }
}
