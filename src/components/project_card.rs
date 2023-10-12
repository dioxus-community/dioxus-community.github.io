use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::models::Project;

#[allow(non_snake_case)]
#[inline_props]
pub fn ProjectCard<'a>(cx: Scope, project: &'a Project<'a>) -> Element {
    render! {
        div { class: "text-white p-4 bg-blue-1 rounded-md",
            table { class: "text-left [&_th]:pr-4",
                tr {
                    th { "👀 Name" }
                    td { "{project.name}" }
                }
                if let Some(description) = &project.description {
                    rsx! {
                        tr {
                            th { "📜 Description" }
                            td { "{description}" }
                        }
                    }
                }
                if let Some(repository_url) = &project.repository_url {
                    rsx! {
                        tr {
                            th { "💾 Repository" }
                            td { Link { class: "underline", to: "{repository_url}", "{repository_url}" } }
                        }
                    }
                }
                if let Some(website) = &project.website {
                    if !website.is_empty() {
                        rsx! {
                            tr {
                                th { "🌐 Website" }
                                td { Link { class: "underline", to: "{website}", "{website}" } }
                            }
                        }
                    } else {
                        rsx! { "" }
                    }
                }
                if let Some(star_count) = project.star_count {
                    rsx! {
                        tr {
                            th { "⭐ Stars" }
                            td { pre { "{star_count}" } }
                        }
                    }
                }
            }
        }
    }
}
