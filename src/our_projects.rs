use crate::models::Project;
use crate::models::ProjectCategory;

macro_rules! our_project {
    (
        name: $name:literal,
        description: $description:literal,
        repository_name: $repository_name:literal,
        category: $category:expr,
        status: $status:expr,
    ) => {
        Project {
            name: ::std::borrow::Cow::Borrowed($name),
            description: ::std::borrow::Cow::Borrowed($description),
            repository_url: ::std::borrow::Cow::Borrowed(concat!(
                "https://github.com/dioxus-community/",
                $repository_name
            )),
            repository_name: $repository_name,
            star_count: None,
            category: $category,
        }
    };

    (
        name: $name:literal,
        description: $description:literal,
        repository_name: $repository_name:literal,
        category: $category:expr,
    ) => {
        Project {
            name: ::std::borrow::Cow::Borrowed($name),
            description: ::std::borrow::Cow::Borrowed($description),
            repository_url: ::std::borrow::Cow::Borrowed(concat!(
                "https://github.com/dioxus-community/",
                $repository_name
            )),
            repository_name: ::std::borrow::Cow::Borrowed(concat!(
                "dioxus-community/",
                $repository_name
            )),
            star_count: None,
            category: $category,
        }
    };
}

pub const OUR_PROJECTS: &[Project<'static>] = &[
    our_project! {
        name: "Dioxus Free Icons",
        description: "Free icons for Dioxus Apps",
        repository_name: "dioxus-free-icons",
        category: ProjectCategory::Components,
    },
    our_project! {
        name: "Dioxus Radio",
        description: "State management",
        repository_name: "dioxus-radio",
        category: ProjectCategory::State,
    },
    our_project! {
        name: "Dioxus i18n",
        description: "i18n support for Dioxus apps",
        repository_name: "dioxus-i18n",
        category: ProjectCategory::Utility,
    },
    our_project! {
        name: "Dioxus Resize Observer",
        description: "Resize observer hooks for web renderers",
        repository_name: "dioxus-resize-observer",
        category: ProjectCategory::Utility,
    },
    our_project! {
        name: "Dioxus Charts",
        description: "A simple chart components library 📈",
        repository_name: "dioxus-charts",
        category: ProjectCategory::Components,
    },
    our_project! {
        name: "Dioxus Material",
        description: "Material You components for Dioxus",
        repository_name: "dioxus-material",
        category: ProjectCategory::Components,
    },
    our_project! {
        name: "Dioxus Use Mounted",
        description: "Mounted Hooks for Dioxus",
        repository_name: "dioxus-use-mounted",
        category: ProjectCategory::Utility,
    },
    our_project! {
        name: "Dioxus Lazy",
        description: "Virtualized components for Dioxus web renderers",
        repository_name: "dioxus-lazy",
        category: ProjectCategory::Utility,
    },
    our_project! {
        name: "Dioxus Use Computed",
        description: "Memoize expensive computations in Dioxus",
        repository_name: "dioxus-use-computed",
        category: ProjectCategory::Utility,
    },
    our_project! {
        name: "Dioxus Spring",
        description: "Animations library for Dioxus web apps",
        repository_name: "dioxus-spring",
        category: ProjectCategory::Styling,
    },
];
