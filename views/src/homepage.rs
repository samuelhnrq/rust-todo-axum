use maud::{html, Markup};

use super::fragments::scaffolding;

#[axum_macros::debug_handler]
pub async fn homepage() -> Markup {
    let body = html! {
        h2.display-2 { "Hello HTMX!" }
        #test { "I'll be replaced" }
        button.btn.btn-primary hx-get="./fragments/wow" hx-target="#test" { "Do something" }
    };
    scaffolding("Hello World", body)
}
