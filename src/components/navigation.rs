
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::Route;

#[derive(Props, Clone, PartialEq)]
struct NavigationLinkProps {
    pub label: String,
    /// Same as the corresponding [`LinkProps`](https://docs.rs/dioxus-router/latest/dioxus_router/components/struct.LinkProps.html) property.
    #[props(into)]
    pub to: IntoRoutable,
}

#[allow(non_snake_case)]
fn NavigationLink(NavigationLinkProps { to, label }: NavigationLinkProps) -> Element {
    rsx!(
        Link {
            active_class: "bg-blue-2",
            class: "p-1 px-4 justify-center grow flex text-white hover:border-transparent hover:bg-blue-2 border-2 border-blue-2 rounded-md md:rounded-full transition ease-in-out delay-30",
            to: to.clone(),
            "{label}"
        }
    )
}

#[allow(non_snake_case)]
pub fn Navigation() -> Element {
    rsx!(
        nav { class: "mx-auto p-2 flex flex-wrap bg-blue-1 rounded-md md:rounded-full gap-2",
            NavigationLink { to: Route::Home {}, label: "Home" }
            NavigationLink { to: "https://github.com/dioxus-community", label: "GitHub" }
            NavigationLink { to: "https://discord.com/invite/XgGxMSkvUM", label: "Discord" }
        }
    )
}