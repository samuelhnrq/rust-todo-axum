use maud::{html, Markup};

use super::spinner;

pub(crate) fn navbar() -> Markup {
    html! {
        nav .navbar .navbar-expand-lg .bg-body-tertiary {
            .container-fluid {
                a .navbar-brand href="#" { "Navbar" }
                button .button .navbar-toggler
                    type="button"
                    data-bs-toggle="collapse"
                    data-bs-target="#navbarNav"
                    aria-controls="navbarNav"
                    aria-expanded="false"
                    aria-label="Toggle navigation" {
                        span .fa-bars {}
                }
                div hx-get="/fragments/login" hx-trigger="load" {
                    (spinner())
                }
                div {
                    span .collapse .navbar-collapse #navbarNav {
                        ul .navbar-nav {
                            li .nav-item {
                                a .nav-link href="#" { "Home" }
                            }
                            li .nav-item {
                                a .nav-link href="#" { "Features" }
                            }
                            li .nav-item {
                                a .nav-link href="#" { "Pricing" }
                            }
                        }
                    }
                }
            }
        }
    }
}
