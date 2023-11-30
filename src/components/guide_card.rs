
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::models::Guide;

#[allow(non_snake_case)]
#[inline_props]
pub fn GuideCard<'a>(cx: Scope, guide: &'a Guide) -> Element {
    render! {
        div { class: "text-white p-4 bg-blue-1 rounded-md",
            table { class: "text-left [&_th]:pr-4",
                tr {
                    th { "👀 Title" }
                    td { "{guide.title}" }
                }
                tr {
                    th { "📜 Description" }
                    td { "{guide.description}" }
                }
                tr {
                    th { "🌐 Website" }
                    td {
                        Link { class: "underline", to: "{guide.website}", "{guide.website}" }
                    }
                }
            }
        }
    }
}