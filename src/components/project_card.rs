use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::{models::Project, GITHUB_API_BASE_URL};

const STARGAZERS_PROPERTY_PATTERN: &str = "\"stargazers_count\":";

/// Creates a card with all the properties of the given project.
/// 
/// # Arguments
/// 
/// - `project` - The project to display.
/// - `insert_stars` - Will insert GitHub stars if the project has a `None` `star_count` property and a `Some(_)` `repository_url` property.
/// If the repository URL is not a GitHub repository, this will fail.
#[allow(non_snake_case)]
#[inline_props]
pub fn ProjectCard<'a>(cx: Scope, project: &'a Project<'a>, insert_stars: bool) -> Element {
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
                Stars { project: project, insert_stars: *insert_stars }
            }
        }
    }
}

#[allow(non_snake_case)]
#[inline_props]
fn Stars<'a>(cx: Scope, project: &'a Project<'a>, insert_stars: bool) -> Element {
    if let Some(star_count) = project.star_count {
        return render! {
            tr {
                th { "⭐ Stars" }
                td { "{star_count}" }
            }
        };
    }

    if !insert_stars {
        return None;
    };

    let Some(repository_url) = &project.repository_url else {
        return None;
    };

    let fetched_stars = use_state(cx, || Option::None::<usize>);

    use_effect(cx, &repository_url.to_string(), |url| {
        to_owned![fetched_stars];

        async move {
            fetched_stars.set(fetch_star_count(&url).await);
        }
    });

    match fetched_stars.get() {
        Some(star_count) => render! {
            tr {
                th { "⭐ Stars" }
                td { "{star_count}" }
            }
        },
        None => None,
    }
}

async fn fetch_star_count(repository_url: &str) -> Option<usize> {
    // Skip initial https://
    let mut segments = repository_url.split('/').skip(2);

    if segments.next() != Some("github.com") {
        return None;
    }

    let Some(owner) = segments.next() else {
        return None;
    };
    let Some(repo) = segments.next() else {
        return None;
    };

    let get_url = format!("{GITHUB_API_BASE_URL}/repos/{owner}/{repo}");

    let Ok(response) = reqwest::get(get_url).await else {
        return None;
    };
    let Ok(response) = response.text().await else {
        return None;
    };

    let Some(stargazers_prop_index) = response.find(STARGAZERS_PROPERTY_PATTERN) else {
        return None;
    };

    let response = response.as_bytes();
    let mut star_count: usize = 0;

    for i in stargazers_prop_index + STARGAZERS_PROPERTY_PATTERN.len()..response.len() {
        let c = response[i] as char;

        if c == ',' {
            break;
        }

        // Skip whitespace after "<PROP>":<WHITESPACE><VALUE>
        if c.is_whitespace() {
            continue;
        }

        if c.is_numeric() {
            star_count = (star_count * 10) + (c.to_digit(10).unwrap() as usize);
        }
    }

    Some(star_count)
}