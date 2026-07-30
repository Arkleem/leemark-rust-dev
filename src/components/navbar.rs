use maud::{html, Markup};

pub fn navbar() -> Markup {
    html! {
        nav {
            ul {
                li {
                    strong {
                        a href="#" class="contrast" {
                            "Leemark Arojo"
                        }
                    }
                }
            }
            ul {
                li { a href="#home" { "Home" } }
                li { a href="#summary" { "About" } }
                li { a href="#skills" { "Skills" } }
                li { a href="#projects" { "Projects" } }
                li { a href="#experience" { "Experience" } }
                li { a href="#contact" { "Contact" } }
            }
        }
    }
}
