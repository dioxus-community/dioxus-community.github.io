use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::Route;
use crate::components::Navigation;

#[allow(non_snake_case)]
#[inline_props]
pub fn Layout(cx: Scope) -> Element {
    render! {
        header { class: "w-full p-8 flex flex-col",
            span { class: "mx-auto text-white pb-8", "D I O X U S - C O M M U N I T Y" }
            Navigation {}
        }
        main { class: "w-full p-8 flex flex-col", Outlet::<Route> {} }
    }
}