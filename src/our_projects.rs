use crate::models::Project;
use crate::models::ProjectCategory;
use crate::models::ProjectStatus;

macro_rules! our_project {
    (
        name: $name:literal,
        description: $description:literal,
        repository_name: $repository_name:literal,
        website: $website:literal,
        category: $category:expr,
        status: $status:expr,
    ) => {
        Project {
            name: ::std::borrow::Cow::Borrowed($name),
            description: Some(::std::borrow::Cow::Borrowed($description)),
            repository_url: Some(::std::borrow::Cow::Borrowed(concat!(
                "https://github.com/dioxus-community/",
                $repository_name
            ))),
            website: Some(::std::borrow::Cow::Borrowed($website)),
            star_count: None,
            category: $category,
            status: $status:,
        }
    };

    (
        name: $name:literal,
        description: $description:literal,
        repository_name: $repository_name:literal,
        category: $category:expr,
        status: $status:expr,
    ) => {
        Project {
            name: ::std::borrow::Cow::Borrowed($name),
            description: Some(::std::borrow::Cow::Borrowed($description)),
            repository_url: Some(::std::borrow::Cow::Borrowed(concat!(
                "https://github.com/dioxus-community/",
                $repository_name
            ))),
            website: None,
            star_count: None,
            category: $category,
            status: $status,
        }
    };
}

pub const OUR_PROJECTS: [Project<'static>; 4] = [
    our_project! {
        name: "Dioxus Redux",
        description: "State management",
        repository_name: "dioxus-redux",
        category: ProjectCategory::Utility,
        status: ProjectStatus::Experimental,
    },
    our_project! {
        name: "Dioxus resize observer",
        description: "Resize observer hooks",
        repository_name: "dioxus-resize-observer",
        category: ProjectCategory::Utility,
        status: ProjectStatus::Experimental,
    },
    our_project! {
        name: "Dioxus Animations",
        description: "Animations library ⏯️",
        repository_name: "dioxus-animations",
        category: ProjectCategory::Utility,
        status: ProjectStatus::Experimental,
    },
    our_project! {
        name: "Dioxus Charts",
        description: "A simple chart components library 📈",
        repository_name: "dioxus-charts",
        category: ProjectCategory::Components,
        status: ProjectStatus::Beta,
    },
];
