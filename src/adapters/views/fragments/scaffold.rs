use maud::{html, Markup, DOCTYPE};

pub fn scaffolding(title: &'static str, children: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html {
            head {
                meta charset="utf-8";
                link type="text/css" rel="stylesheet" href="https://unpkg.com/bootstrap@5.3.3/dist/css/bootstrap.min.css";
                script href="https://cdn.jsdelivr.net/npm/htmx.org@1.9.10/dist/htmx.min.js"
                    defer
                    integrity="sha256-s73PXHQYl6U2SLEgf/8EaaDWGQFCm6H26I+Y69hOZp4="
                    crossorigin="anonymous" {}
                title { (title) }
            }
            body {
                main.container {
                    (children)
                }
            }
        }
    }
}
