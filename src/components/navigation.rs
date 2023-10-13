use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::Route;

const NAVBAR_LINK_STYLE: &str = "p-1 px-4 justify-center grow flex text-white hover:border-transparent hover:bg-blue-2 border-2 border-blue-2 rounded-md md:rounded-full transition ease-in-out delay-30";

#[allow(non_snake_case)]
#[inline_props]
pub fn Navigation(cx: Scope) -> Element {
    render! {
        nav { class: "mx-auto p-2 flex flex-wrap bg-blue-1 rounded-md md:rounded-full gap-2",
            Link { class: NAVBAR_LINK_STYLE, to: Route::Home {}, "Home" }
            Link { class: NAVBAR_LINK_STYLE, to: "https://github.com/dioxus-community", "GitHub" }
            Link { class: NAVBAR_LINK_STYLE, to: Route::OurProjects {}, "Our projects" }
        }
    }
}
