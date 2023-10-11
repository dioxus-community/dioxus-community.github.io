use crate::navbar::*;
use dioxus::prelude::*;

#[inline_props]
pub fn Home(cx: Scope) -> Element {
    render!(
        NavBar {},
        div {
            class: "flex justify-center items-center ",
            div {
                img {
                    class: "mx-auto my-8",
                    width: "200",
                    src: "logo.svg"
                },
                p {
                    class: "text-white w-2/3 text-center mx-auto",
                    "We are a community organization with the goal of developing and maintaining libraries 📦 and resources 📗 for "
                    a {
                        href: "https://dioxuslabs.com",
                        class: "text-slate-400",
                        "Dioxus 🧬"
                    }
                }
            }
        }
    )
}
