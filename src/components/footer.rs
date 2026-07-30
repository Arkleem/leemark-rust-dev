use maud::{html, Markup};

pub fn footer() -> Markup {
    html! {
        footer {
            div class="container" {
                small style="display: block; text-align: center;" {
                    "© 2026 Leemark Arojo. Built with "
                    span style="color: var(--primary);" { "♥" }
                    " using Rust, Axum & Maud"
                }
            }
        }
    }
}
