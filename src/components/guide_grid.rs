use dioxus::prelude::*;
use crate::models::Guide;
use crate::components::GuideCard;

#[allow(non_snake_case)]
#[inline_props]
pub fn GuideGrid<'a>(cx: Scope, guides: &'a [Guide]) -> Element {
    render! {
        div { class: "grid grid-cols-1 lg:grid-cols-2 gap-4",
            for guide in guides {
                GuideCard { key: "{guide.name}", guide: guide }
            }
        }
    }
}