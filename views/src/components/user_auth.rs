use maud::{html, Markup, PreEscaped};

const AUTH_SCRIPT: &str = include_str!("./user_auth.js");

pub fn user_auth() -> Markup {
    html! {
        // script
        //     #auth-script-tag
        //     defer
        //     src="/public/app_auth.js"
        //     type="text/javascript" {}
        button type="button" #sign-in .btn style="display: none" {
            "Sign in"
        }
        #clerk-container {}
        // script { (PreEscaped(AUTH_SCRIPT)) }
    }
}
