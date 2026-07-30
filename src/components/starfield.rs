use maud::{html, Markup};

pub fn starfield() -> Markup {
    html! {
        // Starfield web component with DataStar signals
        star-field
            data-attr:center-x="$starX"
            data-attr:center-y="$starY"
            data-attr:speed="$starSpeed"
            style="position: fixed; top: 0; left: 0; width: 100%; height: 100%; z-index: 0; pointer-events: none;"
        {}
    }
}
