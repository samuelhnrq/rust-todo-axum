use maud::{html, Markup, PreEscaped};

const SPINNER_SVG: &str = include_str!("spinner.svg");

pub(crate) fn spinner() -> Markup {
  html! {
    (PreEscaped(SPINNER_SVG))
  }
}
