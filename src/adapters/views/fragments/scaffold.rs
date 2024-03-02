use maud::{html, Markup, DOCTYPE};

pub fn scaffolding(title: &'static str, children: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html {
            head {
                meta charset="utf-8";
                link type="text/css" rel="preload" as="style" id="bootstrap"
                    href="https://unpkg.com/bootstrap@5.3.3/dist/css/bootstrap.min.css";
                title { (title) }
            }
            body {
                script
                    async
                    crossorigin="anonymous"
                    data-clerk-publishable-key="pk_test_YmFsYW5jZWQtZWxrLTc2LmNsZXJrLmFjY291bnRzLmRldiQ"
                    onload="window.Clerk.load()"
                    src="https://balanced-elk-76.clerk.accounts.dev/npm/@clerk/clerk-js@4/dist/clerk.browser.js"
                    type="text/javascript" {}
                script href="https://cdn.jsdelivr.net/npm/htmx.org@1.9.10/dist/htmx.min.js"
                    defer
                    integrity="sha256-s73PXHQYl6U2SLEgf/8EaaDWGQFCm6H26I+Y69hOZp4="
                    crossorigin="anonymous" {}
                main.container {
                    (children)
                }
                link type="text/css" rel="stylesheet"
                    href="https://unpkg.com/bootstrap@5.3.3/dist/css/bootstrap.min.css";
            }
        }
    }
}
