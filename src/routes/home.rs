use dioxus::prelude::*;
use dioxus_router::prelude::Link;

use crate::{our_projects::OUR_PROJECTS, ProjectGrid};

#[allow(non_snake_case)]
#[component]
pub fn Home() -> Element {
    rsx!(
        div { class: "flex justify-center items-center text-white",
            div {
                img { class: "mx-auto my-8", width: "200", src: "logo.svg" }
                p { class: "w-2/3 text-center mx-auto",
                    "We are a community organization with the goal of developing and maintaining libraries 📦 and resources 📗 for "
                    a { href: "https://dioxuslabs.com", class: "text-slate-400", "Dioxus 🧬" }
                }
                p { class: "w-2/3 text-center mx-auto pb-4",
                    "A list of all our projects. If you want to add your project to our organization, let us know at the Dioxus "
                    Link { class: "underline", to: "https://discord.com/invite/XgGxMSkvUM", "Discord server" }
                    "! Make sure you select \"Dioxus Ecosystem\" in the \"Channels & Roles\" to see our channel."
                }
                div { class: "mx-auto lg:w-9/12", ProjectGrid { projects: OUR_PROJECTS, insert_stars: true } }
            }
        }
    )
}