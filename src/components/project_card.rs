use crate::{models::Project, GITHUB_API_BASE_URL};
use dioxus::prelude::*;
use dioxus_router::prelude::*;

const STARGAZERS_PROPERTY_PATTERN: &str = "\"stargazers_count\":";

/// Creates a card with all the properties of the given project.
///
/// # Arguments
///
/// - `project` - The project to display.
/// - `insert_stars` - Will insert GitHub stars if the project has a `None` `star_count` property and a `Some(_)` `repository_url` property.
/// If the repository URL is not a GitHub repository, this will fail.
#[allow(non_snake_case)]
#[component]
pub fn ProjectCard(project: &'static Project<'static>, insert_stars: bool) -> Element {
    rsx!(
        div { class: "text-white p-4 bg-blue-1 rounded-md",
            table { class: "text-left [&_th]:pr-4",
                tr {
                    th { "👀 Name" }
                    td { "{project.name}" }
                }
                tr {
                    th { "🛠️ Status" }
                    td { "{project.status}" }
                }
                if let Some(description) = &project.description {
                    tr {
                        th { "📜 Description" }
                        td { "{description}" }
                    }
                }
                if let Some(repository_url) = &project.repository_url {
                    tr {
                        th { "💾 Repository" }
                        td { Link { class: "underline", to: "{repository_url}", "{repository_url}" } }
                    }
                }
                if let Some(website) = &project.website {
                    if !website.is_empty() {
                        tr {
                            th { "🌐 Website" }
                            td { Link { class: "underline", to: "{website}", "{website}" } }
                        }
                    }
                }
                Stars { project, insert_stars }
            }
        }
    )
}

#[allow(non_snake_case)]
#[component]
fn Stars(project: &'static Project<'static>, insert_stars: bool) -> Element {
    if let Some(star_count) = project.star_count {
        return rsx!(
            tr {
                th { "⭐ Stars" }
                td { "{star_count}" }
            }
        );
    }

    let empty_star_row = rsx! {
        tr {
            th { "⭐ Stars" }
            td { "N/A" }
        }
    };

    if !insert_stars {
        return empty_star_row;
    };

    let Some(repository_url) = &project.repository_url else {
        return empty_star_row;
    };

    let fetched_stars = use_resource(use_reactive(&repository_url.to_string(), |repository_url| {
        fetch_star_count(repository_url)
    }));

    let fetched_stars = &*fetched_stars.read();

    match fetched_stars {
        Some(Ok(star_count)) => rsx!(
            tr {
                th { "⭐ Stars" }
                td { "{star_count}" }
            }
        ),
        Some(Err(e)) => {
            log::error!("couldn't fetch stars from repository \"{repository_url}\". Error: {e}");

            None
        }
        None => empty_star_row
    }
}

async fn fetch_star_count(repository_url: String) -> Result<usize, String> {
    // Skip initial https://
    let mut segments = repository_url.split('/').skip(2);

    if segments.next() != Some("github.com") {
        return Err(String::from("repository URL is not from GitHub"));
    }

    let Some(owner) = segments.next() else {
        return Err(format!("couldn't get repository owner from the repository URL segments. Segments: {segments:?}"));
    };
    let Some(repo) = segments.next() else {
        return Err(format!("couldn't get repository name from the repository URL segments. Segments: {segments:?}"));
    };

    let get_url = format!("{GITHUB_API_BASE_URL}/repos/{owner}/{repo}");

    let Ok(response) = reqwest::get(&get_url).await else {
        return Err(format!("reqwest: Couldn't send request to {get_url:?}"));
    };
    let Ok(response) = response.text().await else {
        return Err(format!("reqwest: Couldn't parse response as text. Response URL: {get_url:?}"));
    };

    let Some(stargazers_prop_index) = response.find(STARGAZERS_PROPERTY_PATTERN) else {
        return Err(format!("couldn't find the pattern \"{STARGAZERS_PROPERTY_PATTERN}\" in response text. Response text: {response:?}"));
    };

    let response = response.as_bytes();
    let mut star_count: usize = 0;

    for byte in response
        .iter()
        .skip(stargazers_prop_index + STARGAZERS_PROPERTY_PATTERN.len())
    {
        let c = *byte as char;

        if c == ',' {
            break;
        }

        // Skip whitespace after "<PROP>":<WHITESPACE?><VALUE>
        if c.is_whitespace() {
            continue;
        }

        if c.is_numeric() {
            let Some(c_digit) = c.to_digit(10) else {
                return Err(format!("couldn't convert char {c:?} to a digit using a radix of 10"));
            };

            star_count = (star_count * 10) + (c_digit as usize);
        }
    }

    Ok(star_count)
}
