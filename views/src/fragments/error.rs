use maud::{html, Markup};

pub fn build_error_fragment(message: &str) -> Markup {
    html! {
        p { (message) }
    }
}
