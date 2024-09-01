use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::Route;
use crate::components::Navigation;

#[allow(non_snake_case)]
#[component]
pub fn Layout() -> Element {
    rsx!(
        header { class: "w-full p-8 flex flex-col",
            span { class: "mx-auto text-white pb-8", "D I O X U S - C O M M U N I T Y" }
            Navigation {}
        }
        main { class: "w-full p-8 pt-0 flex flex-col", Outlet::<Route> {} }
    )
}