use crate::components::page_shell::page_shell;

use axum::response::IntoResponse;

use rstml_to_string_macro::html;

pub fn error_template(errors: Option<Vec<(String, String)>>) -> impl IntoResponse {
    if let Some(errors) = &errors {
        for (key, error) in errors {
            println!("{:?}: {:?}", key, error);
        }
    }

    let html_content = html! {
        <>
          "This is the error template. You can modify it if you want to."

        </>
    };

    let shelled_content = page_shell(
        "Error".to_string(),
        html_content,
        "".to_string(),
        "".to_string(),
    );
    axum::http::Response::builder()
        .header(axum::http::header::CONTENT_TYPE, "text/html")
        .body(shelled_content)
        .expect("Failed to render home page")
}
