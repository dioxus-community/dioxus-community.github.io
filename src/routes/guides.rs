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
                "A list of all our projects. If you want to add your project to our organization, let us know at the Dioxus "
                Link { class: "underline", to: "https://discord.com/invite/XgGxMSkvUM", "Discord server" }
                "! Make sure you select \"Dioxus Ecosystem\" in the \"Channels & Roles\" to see our channel.
                Here's the "
                Link {
                    class: "underline",
                    to: "https://discord.com/channels/899851952891002890/1155572382157783141",
                    "link"
                }
                "."
            }
            div { class: "mx-auto lg:w-9/12", GuideGrid { guides: &GUIDES } }
        }
    }
}