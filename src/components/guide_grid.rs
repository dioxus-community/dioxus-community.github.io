use dioxus::prelude::*;
use crate::models::Guide;
use crate::components::GuideCard;

#[allow(non_snake_case)]
#[component]
pub fn GuideGrid(guides: &'static [Guide]) -> Element {
    rsx!(
        div { class: "grid grid-cols-1 lg:grid-cols-2 gap-4",
            for guide in guides {
                GuideCard { key: "{guide.name}", guide }
            }
        }
    )
}