use rstml_to_string_macro::html;

pub fn page_shell(
    title: String,
    body_content: String,
    additional_meta_tags: String,
    end_body_tag_scripts: String,
) -> String {
    // FOUC prevention script
    let fouc_prevention_script = r#"
    <script>
        document.documentElement.className += 'js';
    </script>
    "#;

    let fouc_prevention_script_dom_content_loaded = r#"
    <script>
        window.addEventListener('load', function() {
            document.documentElement.className = '';
        });
    </script>
    "#;

    let style = format!("<style>{}</style>", include_str!("../../styles/main.css"));

    html! {
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="utf-8">
            <meta name="viewport" content="width=device-width, initial-scale=1">
            <meta name="description" content="Flack - A modern chat application for teams">
            <meta name="theme-color" content="#cf0909">
            <meta property="og:title" content={format!("{} - Flack", title)}>
            <meta property="og:description" content="Flack - A modern chat application for teams">
            <meta property="og:image" content="/flack.png">
            <meta property="og:type" content="website">
            <meta property="og:site_name" content="Flack">
            <link rel="icon" href="/favicon.ico">
            <link rel="apple-touch-icon" href="/apple-touch-icon.png">
            <title>{format!("Flack - {}", title)}</title>
           {style}
            {additional_meta_tags}
            {fouc_prevention_script}
        </head>
        <body class="bg-eggshell">
            {body_content}
            {end_body_tag_scripts}
            {fouc_prevention_script_dom_content_loaded}
        </body>
        </html>
    }
}
