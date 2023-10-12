use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::Route;

const NAVBAR_LINK_STYLE: &str = "p-1 px-4 text-center flex items-center text-white hover:border-transparent hover:bg-blue-2 border-2 border-blue-2 rounded-md transition ease-in-out delay-30";

#[allow(non_snake_case)]
#[inline_props]
pub fn Navigation(cx: Scope) -> Element {
    render! {
        nav { class: "mx-auto p-2 flex justify-stretch bg-blue-1 rounded-md gap-2",
            Link { class: NAVBAR_LINK_STYLE, to: Route::Home {}, "Home" }
            Link { class: NAVBAR_LINK_STYLE, to: "https://github.com/dioxus-community", "GitHub" }
            Link { class: NAVBAR_LINK_STYLE, to: Route::OurProjects {}, "Our projects" }
            Link { class: NAVBAR_LINK_STYLE, to: Route::MadeWithDioxus {}, "Made with Dioxus" }
        }
    }
}
