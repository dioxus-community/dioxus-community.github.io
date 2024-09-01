use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::Route;

#[allow(non_snake_case)]
#[allow(unused_variables)]
#[component]
pub fn NotFound(segments: Vec<String>) -> Element {
    rsx!(
        p { class: "mx-auto w-2/3 text-white text-center",
            "😕 We couldn't find that route. How about going back "
            Link { class: "underline", to: Route::Home {}, "home" }
            "?"
        }
    )
}