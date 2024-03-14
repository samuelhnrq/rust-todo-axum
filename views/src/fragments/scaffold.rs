use maud::{html, Markup, DOCTYPE};

const CRITICAL_CSS: &'static str = "
body, html {
    margin: 0;
    font-size: 16px;
    font-family: sans-serif;
}

button {
    padding: 0.375rem 0.75rem;
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
                    crossorigin="anonymous"
                    data-clerk-publishable-key="pk_test_YmFsYW5jZWQtZWxrLTc2LmNsZXJrLmFjY291bnRzLmRldiQ"
                    onload="window.Clerk.load()"
                    src="https://balanced-elk-76.clerk.accounts.dev/npm/@clerk/clerk-js@4/dist/clerk.browser.js"
                    type="text/javascript" {}
                script src="https://unpkg.com/htmx.org@1.9.10"
                    integrity="sha384-D1Kt99CQMDuVetoL1lrYwg5t+9QdHe7NLX/SoJYkXDFfX37iInKRy5xLSi8nO7UC"
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
