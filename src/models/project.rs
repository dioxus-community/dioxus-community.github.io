use std::borrow::Cow;
use crate::models::ProjectCategory;


#[derive(PartialEq, Clone, Debug)]
pub struct Project<'a> {
    pub name: Cow<'a, str>,
    pub description: Cow<'a, str>,
    pub repository_url: Cow<'a, str>,
    pub repository_name: Cow<'a, str>,
    pub star_count: Option<usize>,
    pub category: ProjectCategory,
}