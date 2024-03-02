use maud::{html, Markup};

use super::fragments::scaffolding;

#[axum_macros::debug_handler]
pub async fn homepage() -> Markup {
    let body = html! {
        div.display-2 { "Hello world" }
    };
    scaffolding("Hello World", body)
}
