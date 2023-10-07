use dioxus_router::prelude::*;

use dioxus::prelude::*;
use crate::Route;

const NAVBAR_LINK_STYLE: &str = "p-1 px-4 text-center flex items-center text-white hover:border-transparent hover:bg-blue-2 border-2 border-blue-2 rounded-full transition ease-in-out delay-30";

#[inline_props]
pub fn NavBar(cx: Scope) -> Element {
    render!(
        div {
            class: "w-full p-8 flex flex-col",
            span {
                class: "mx-auto text-white pb-8",
                "D I O X U S - C O M M U N I T Y"
            }
            div {
                class: "mx-auto p-2 flex justify-stretch bg-blue-1 rounded-full gap-2",
                Link {
                    class: NAVBAR_LINK_STYLE,
                    to: Route::Home { },
                    "Home"
                }
                Link {
                    class: NAVBAR_LINK_STYLE,
                    to: "https://github.com/dioxus-community",
                    "GitHub"
                }
                // Link {
                //     class: NAVBAR_LINK_STYLE,
                //     to: Route::MadeWithDioxus { },
                //     "Made with Dioxus"
                // }
            }
        }
    )
}