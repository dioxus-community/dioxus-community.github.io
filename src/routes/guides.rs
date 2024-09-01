use dioxus::prelude::*;

use crate::GUIDES;
use crate::components::GuideGrid;

#[allow(non_snake_case)]
#[component]
pub fn Guides() -> Element {
    rsx!(
        div { class: "text-white",
            p { class: "w-2/3 text-center mx-auto pb-4",
                "A list of some guides that will help you in your Dioxus journey!"
            }
            div { class: "mx-auto lg:w-9/12", GuideGrid { guides: GUIDES } }
        }
    )
}