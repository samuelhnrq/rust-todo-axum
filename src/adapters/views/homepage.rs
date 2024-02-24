use maud::{html, Markup};

use super::fragments::scaffolding;

#[axum_macros::debug_handler]
pub async fn homepage() -> Markup {
    let body = html! {
        p { "Hello world" }
    };
    scaffolding("Hello World", body)
}
