use std::borrow::Cow;

#[derive(PartialEq, Clone, Debug)]
pub struct Project<'a> {
    pub name: Cow<'a, str>,
    pub description: Option<Cow<'a, str>>,
    pub repository_url: Option<Cow<'a, str>>,
    pub website: Option<Cow<'a, str>>,
    pub star_count: Option<usize>,
}

macro_rules! project_static {
    (
        name: $name:literal,
        description: $description:literal,
        repository_url: $repository_url:literal,
        website: $website:literal,
    ) => {
        Project {
            name: ::std::borrow::Cow::Borrowed($name),
            description: Some(::std::borrow::Cow::Borrowed($description)),
            repository_url: Some(::std::borrow::Cow::Borrowed($repository_url)),
            website: Some(::std::borrow::Cow::Borrowed($website)),
            star_count: None,
        }
    };
}

pub(crate) use project_static;