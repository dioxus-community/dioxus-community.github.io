use dioxus::prelude::*;
use crate::OUR_PROJECTS;
use crate::components::ProjectGrid;

#[allow(non_snake_case)]
#[inline_props]
pub fn OurProjects(cx: Scope) -> Element {
    render! {
        div { class: "text-white",
            p { class: "w-2/3 text-center mx-auto pb-4",
                "A list of all projects that gather under the name of Dioxus Community!"
            }
            div { class: "mx-auto lg:w-9/12", ProjectGrid { projects: &OUR_PROJECTS } }
        }
    }
}