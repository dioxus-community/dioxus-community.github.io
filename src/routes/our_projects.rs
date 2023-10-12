use std::borrow::Cow;
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::{GITHUB_API_BASE_URL, ORGANIZATION_NAME, PROJECT_MARKER_FILE_NAME};
use crate::models::Project;
use serde::{Serialize, Deserialize};
use crate::components::{ProjectCard, ProjectGrid};

#[allow(non_snake_case)]
#[inline_props]
pub fn OurProjects(cx: Scope) -> Element {
    let fetched_projects = use_future(cx, (), |_| fetch_projects());

    render! {
        div { class: "text-white",
            p { class: "text-white w-2/3 text-center mx-auto pb-4",
                "This is an automatically generated list of projects made by us. How? Simple, there's a "
                code { ".dxcom-project" }
                " file in each repository that's supposed to be here. Then, this page looks for that file in each repository in the "
                Link { to: "https://github.com/dioxus-community", "Dioxus Community organization" }
                "."
            }
            // TODO: Cache the value
            match fetched_projects.value() {
                Some(Ok(projects)) => rsx! { ProjectGrid { projects: projects } },
                Some(Err(e)) => rsx! { "🤔 An error occurred while fetching projects: {e}" },
                None => rsx! { "Loading..." },
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
struct OrganizationRepositoryEntry {
    name: String,
    description: Option<String>,
    #[serde(rename = "html_url")]
    url: String,
    homepage: Option<String>,
    #[serde(rename = "stargazers_count")]
    star_count: usize,
    private: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
struct RepositoryContentsEntry {
    name: String,
    r#type: String,
}

async fn get_repository_contents(owner: &str, repository: &str, path: Option<&str>) -> reqwest::Result<Vec<RepositoryContentsEntry>> {
    let get_url = match path {
        None => format!("{GITHUB_API_BASE_URL}/repos/{owner}/{repository}/contents/"),
        Some(path) => format!("{GITHUB_API_BASE_URL}/repos/{owner}/{repository}/contents/{path}"),
    };

    reqwest::get(get_url).await?.json::<Vec<RepositoryContentsEntry>>().await
}

async fn get_organization_repositories(organization: &str) -> reqwest::Result<Vec<OrganizationRepositoryEntry>> {
    let get_url = format!("{GITHUB_API_BASE_URL}/orgs/{organization}/repos");

    reqwest::get(get_url).await?.json::<Vec<OrganizationRepositoryEntry>>().await
}

async fn fetch_projects<'a>() -> reqwest::Result<Vec<Project<'a>>> {
    let mut projects = Vec::new();

    let repositories = get_organization_repositories(ORGANIZATION_NAME).await?;

    for repository in repositories {
        if repository.private {
            continue;
        }

        let contents = get_repository_contents(ORGANIZATION_NAME, &repository.name, None).await?;

        if contents.iter().any(|c| c.name == PROJECT_MARKER_FILE_NAME && c.r#type == "file") {
            projects.push(Project {
                name: Cow::from(repository.name),
                description: repository.description.map(Cow::from),
                repository_url: Some(Cow::from(repository.url)),
                website: repository.homepage.map(Cow::from),
                star_count: Some(repository.star_count),
            });
        }
    }

    Ok(projects)
}