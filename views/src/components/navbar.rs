use maud::{html, Markup};

use super::spinner;

pub(crate) fn navbar() -> Markup {
    html! {
        nav .navbar .bg-body-tertiary {
            .nav-container {
                a .navbar-brand href="#" { "Hyper Tarot" }
                div hx-get="/fragments/login" hx-trigger="load from:document" {
                    (spinner())
                    a id="login-anchor" style="display: none" {}
                }
            }
        }
    }
}
