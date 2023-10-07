use dioxus::prelude::*;

use crate::navbar::NavBar;

// pub struct Project<'a> {
//     name: &'a str
// }

// const PROJECTS: [Project<'static>; 1] = [
//     Project {
//         name: "Ambient.run"
//     }
// ];

#[inline_props]
pub fn MadeWithDioxus(cx: Scope) -> Element {
    render!(
        NavBar {},
        p {
            class: "text-white text-center",
            "Coming soon!"
        }
    )
}

// #[inline_props]
// pub fn ProjectCard<'a>(cx: Scope, project: Project<'a>) -> Element {
//     render!(
//         p {
//             class: "text-white p-4 h-40 w-80 bg-blue-1 rounded-md",
//             "{project.name}"
//         }
//     )
// }
