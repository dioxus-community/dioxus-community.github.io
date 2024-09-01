use dioxus::prelude::*;
use crate::models::Project;
use crate::components::ProjectCard;

#[allow(non_snake_case)]
#[component]
pub fn ProjectGrid(projects: &'static [Project<'static>], insert_stars: bool) -> Element {
    rsx!(
        div { class: "grid grid-cols-2 lg:grid-cols-2 gap-4",
            for project in projects {
                ProjectCard { key: "{project.name}", project, insert_stars }
            }
        }
    )
}