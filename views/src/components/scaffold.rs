use super::navbar::navbar;
use maud::{html, Markup, PreEscaped, DOCTYPE};

const CRITICAL_CSS: &str = include_str!("scaffold.css");
const BASIC_JS: &str = include_str!("scaffold.js");

// TODO: Reciever user login status here
pub(crate) fn scaffolding(title: &'static str, children: &Markup) -> Markup {
    html! {
        (DOCTYPE)
        html {
            head {
                title { (title) }
                meta charset="utf-8";
                link type="text/css" rel="preload" as="style"
                    href="/public/bootstrap.min.css";
                style #critical-css { (CRITICAL_CSS) }
                script
                    defer
                    src="/public/bootstrap.min.js" {}
                script defer src="/public/htmx.min.js" {}
                script defer src="/public/idiomorph-ext.min.js" {}
                script defer src="/public/_hyperscript.min.js" {}
            }
            body hx-ext="morph" hx-boost="true" {
                banner {
                    (navbar()) // pass user here
                }
                main.container {
                    (children)
                }
                script { (PreEscaped(BASIC_JS)) }
            }
        }
    }
}
