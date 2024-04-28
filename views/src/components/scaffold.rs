use super::navbar::navbar;
use maud::{html, Markup, DOCTYPE};

const CRITICAL_CSS: &str = "
body, html {
    margin: 0;
    font-size: 16px;
    font-family: sans-serif;
}

button {
    padding: 0.375rem 0.75rem;
    border: none;
    background-color: unset;
}
";

pub fn scaffolding(title: &'static str, children: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html {
            head {
                meta charset="utf-8";
                link type="text/css" rel="preload" as="style"
                    href="https://unpkg.com/bootstrap@5.3.3/dist/css/bootstrap.min.css";
                style #critical-css { (CRITICAL_CSS) }
                title { (title) }
            }
            body {
                script
                    defer
                    src="https://cdn.jsdelivr.net/npm/bootstrap@5.3.3/dist/js/bootstrap.min.js"
                    integrity="sha256-3gQJhtmj7YnV1fmtbVcnAV6eI4ws0Tr48bVZCThtCGQ="
                    crossorigin="anonymous" {}
                script defer src="https://kit.fontawesome.com/ffba154c07.js" crossorigin="anonymous" {}
                script src="https://unpkg.com/htmx.org@1.9.10"
                    integrity="sha384-D1Kt99CQMDuVetoL1lrYwg5t+9QdHe7NLX/SoJYkXDFfX37iInKRy5xLSi8nO7UC"
                    crossorigin="anonymous" {}
                banner {
                    (navbar())
                }
                main.container {
                    (children)
                }
                link type="text/css" rel="stylesheet"
                    href="https://unpkg.com/bootstrap@5.3.3/dist/css/bootstrap.min.css";
            }
        }
    }
}
