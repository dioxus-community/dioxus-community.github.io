use std::fmt::{Display, Formatter};

#[derive(Clone, PartialEq, Debug)]
pub enum ProjectCategory {
    App,
    Miscellaneous,
    Utility,
    Components,
    Styling,
    Logging,
    Renderer,
    Deployment,
}

impl ProjectCategory {
    fn to_static_string(&self) -> &'static str {
        match self {
            Self::App => "🚀 App",
            Self::Miscellaneous => "📎 Miscellaneous",
            Self::Utility => "🧰 Utility",
            Self::Components => "📦 Components",
            Self::Styling => "🎨 Styling",
            Self::Logging => "📡 Logging",
            Self::Renderer => "🎥 Renderer",
            Self::Deployment => "⚙️ Deployment",
        }
    }
}

impl Display for ProjectCategory {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_static_string())
    }
}