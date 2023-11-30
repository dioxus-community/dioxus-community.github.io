use std::borrow::Cow;
use crate::models::ProjectCategory;

use super::ProjectStatus;

#[derive(PartialEq, Clone, Debug)]
pub struct Project<'a> {
    pub name: Cow<'a, str>,
    pub description: Option<Cow<'a, str>>,
    pub repository_url: Option<Cow<'a, str>>,
    pub website: Option<Cow<'a, str>>,
    pub star_count: Option<usize>,
    pub category: ProjectCategory,
    pub status: ProjectStatus,
}