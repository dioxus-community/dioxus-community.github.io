use dioxus::prelude::*;
use crate::models::Project;
use crate::components::ProjectCard;

#[allow(non_snake_case)]
#[inline_props]
pub fn ProjectFlexbox<'a>(cx: Scope, projects: &'a [Project<'a>]) -> Element {
    render! {
        div { class: "flex flex-wrap flex-row gap-4",
            for project in projects {
                ProjectCard { key: "{project.name}", project: project }
            }
        }
    }
}