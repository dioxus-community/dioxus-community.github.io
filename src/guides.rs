use crate::models::{Guide, ProjectCategory};

macro_rules! book_guide {
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
            website: concat!("guides/books/", $name, "/index.html"),
            category: $category,
        }
    };
}

pub const GUIDES: &[Guide] = &[book_guide! {
    title: "Learning Rust by making a website",
    name: "learning-rust-by-making-a-website",
    description: "Description",
    category: ProjectCategory::App,
}];
