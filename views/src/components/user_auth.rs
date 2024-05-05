use maud::{html, Markup, PreEscaped};

const AUTH_SCRIPT: &str = include_str!("./user_auth.js");

pub fn user_auth() -> Markup {
    html! {
        script
            #clerk-script-tag
            defer
            crossorigin="anonymous"
            data-clerk-publishable-key="pk_test_YmFsYW5jZWQtZWxrLTc2LmNsZXJrLmFjY291bnRzLmRldiQ"
            src="https://balanced-elk-76.clerk.accounts.dev/npm/@clerk/clerk-js@4/dist/clerk.browser.js"
            type="text/javascript" {}
        button type="button" #sign-in .btn style="display: none" {
            "Sign in"
        }
        #clerk-container {}
        script { (PreEscaped(AUTH_SCRIPT)) }
    }
}
