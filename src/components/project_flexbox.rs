use dioxus::prelude::*;
use crate::models::Project;
use crate::components::ProjectCard;

#[allow(non_snake_case)]
#[inline_props]
pub fn ProjectGrid<'a>(cx: Scope, projects: &'a [Project<'a>]) -> Element {
    render! {
        div { class: "grid grid-cols-2 gap-4",
            for project in projects {
                ProjectCard { key: "{project.name}", project: project }
            }
        }
    }
}