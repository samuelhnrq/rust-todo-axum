use maud::{html, Markup};

use super::fragments::scaffolding;

pub async fn homepage() -> Markup {
    let body = html! {
        div.display-2 { "Hello world" }
    };
    scaffolding("Hello World", body)
}
