use dioxus::prelude::*;
use crate::models::Project;
use crate::components::ProjectCard;

#[allow(non_snake_case)]
#[inline_props]
pub fn ProjectFlexbox<'a>(cx: Scope, projects: &'a [Project<'a>]) -> Element {
    render! {
        div { class: "flex flex-wrap gap-4 [&_*]:grow",
            for project in projects {
                ProjectCard { key: "{project.name}", project: project }
            }
        }
    }
}