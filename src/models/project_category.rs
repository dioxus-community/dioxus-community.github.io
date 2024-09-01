use std::fmt::{Display, Formatter};

#[derive(Clone, PartialEq, Debug)]
#[allow(dead_code)]
pub enum ProjectCategory {
    App,
    Miscellaneous,
    State,
    Utility,
    Components,
    Styling,
    Logging,
    Renderer,
    Deployment,
    Security,
}

impl ProjectCategory {
    fn to_static_string(&self) -> &'static str {
        match self {
            Self::App => "🚀 App",
            Self::Miscellaneous => "📎 Miscellaneous",
            Self::State => "📻 State",
            Self::Utility => "🧰 Utility",
            Self::Components => "📦 Components",
            Self::Styling => "🎨 Styling",
            Self::Logging => "📡 Logging",
            Self::Renderer => "🎥 Renderer",
            Self::Deployment => "⚙️ Deployment",
            Self::Security => "🔒 Security",
        }
    }
}

impl Display for ProjectCategory {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_static_string())
    }
}