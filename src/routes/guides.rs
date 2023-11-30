use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::GUIDES;
use crate::components::{GuideGrid};

#[allow(non_snake_case)]
#[inline_props]
pub fn Guides(cx: Scope) -> Element {
    render! {
        div { class: "text-white",
            p { class: "w-2/3 text-center mx-auto pb-4",
                "A list of some guides that will help you in your Dioxus journey!"
            }
            div { class: "mx-auto lg:w-9/12", GuideGrid { guides: &GUIDES } }
        }
    }
}