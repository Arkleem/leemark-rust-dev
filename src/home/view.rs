use maud::{html, Markup};  // Only one import
use crate::state::AppState;

pub fn home_page(state: &AppState) -> Markup {
    use crate::components::layout::base_layout;
    use crate::hero::view::hero_section;
    use crate::summary::view::summary_section;
    use crate::skills::view::skills_section;
    use crate::projects::view::projects_section;
    use crate::experience::view::experience_section;
    use crate::contact::view::contact_section;

    base_layout("Leemark - Rust Developer", html! {
        (hero_section())
        (summary_section())
        (skills_section(state))
        (projects_section(state))
        (experience_section(state))
        (contact_section())
    })
}
