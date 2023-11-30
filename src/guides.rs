use crate::models::{Guide, ProjectCategory};

macro_rules! guide {
    (
        title: $title:expr,
        name: $name:expr,
        description: $description:expr,
        category: $category:expr,
    ) => {
        crate::models::Guide {
            title: $title,
            name: $name,
            description: $description,
            website: concat!("https://dioxus-community.githhub.io/guides/", $name, "/index.html"),
            category: $category,
        }
    };
}

pub const GUIDES: [Guide; 1] = [
    guide! {
        title: "Learning Rust by making a website",
        name: "learning-rust-by-making-a-website",
        description: "",
        category: ProjectCategory::App,
    }
];
