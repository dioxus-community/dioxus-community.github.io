use crate::models::ProjectCategory;

#[derive(PartialEq, Clone, Debug)]
pub struct Guide {
    pub title: &'static str,
    pub name: &'static str,
    pub website: &'static str,
    pub description: &'static str,
    pub category: ProjectCategory,
}