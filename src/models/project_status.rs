use std::fmt::{Formatter, Display};

#[derive(PartialEq, Clone, Debug)]
#[allow(dead_code)]
pub enum ProjectStatus {
    Stable,
    Experimental,
}

impl ProjectStatus {
    fn to_static_string(&self) -> &'static str {
        match self {
            Self::Stable => "Stable ✔️",
            Self::Experimental => "Experimental ⚠️",
        }
    }
}

impl Display for ProjectStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_static_string())
    }
}