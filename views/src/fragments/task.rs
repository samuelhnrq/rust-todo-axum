use maud::{html, Markup};
use std::time::{SystemTime, UNIX_EPOCH};

#[axum_macros::debug_handler]
pub async fn tasks_fragment() -> Markup {
    let now = SystemTime::now();
    let since_the_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
    return html! {
        .wow { "hello world its now " (since_the_epoch.as_secs()) }
    };
}
