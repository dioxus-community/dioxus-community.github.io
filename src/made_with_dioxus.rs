use crate::models::Project;

macro_rules! made_with_dioxus {
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
            category: $crate::models::ProjectCategory::App,
        }
    };
}

pub const MADE_WITH_DIOXUS: &[Project<'static>] = &[
    made_with_dioxus! {
        name: "Uplink",
        description: "Secure, encrypted, P2P chat app",
        repository_url: "https://github.com/Satellite-im/Uplink",
        website: "https://uplink.satellite.im",
    },
    made_with_dioxus! {
        name: "Ebou",
        description: "Cross platform Mastodon Client",
        repository_url: "https://github.com/terhechte/Ebou",
        website: "https://terhech.de/ebou",
    },
    made_with_dioxus! {
        name: "Ambient",
        description: "Multiplayer game engine that runs in the browser",
        repository_url: "https://github.com/ambientrun/ambient",
        website: "https://ambient.run",
    },
    made_with_dioxus! {
        name: "Cloak",
        description: "Secrets automation for developers",
        repository_url: "https://github.com/purton-tech/cloak",
        website: "https://cloak.software",
    },
    made_with_dioxus! {
        name: "Hemi",
        description: "Typing tutor for training hands separately",
        repository_url: "https://github.com/kualta/hemi",
        website: "https://hemi.kualta.dev",
    },
    made_with_dioxus! {
        name: "InterSpace",
        description: "The graphical interface pipeline visualized",
        repository_url: "https://github.com/erithax/interspace",
        website: "https://erithax.com",
    },
    made_with_dioxus! {
        name: "Patina",
        description: "Simple text editor with Rust syntax highlighting",
        repository_url: "https://github.com/ealmloff/Patina",
        website: "https://ealmloff.github.io/Patina",
    },
    made_with_dioxus! {
        name: "Pomo",
        description: "Minimalistic Pomodoro timer",
        repository_url: "https://github.com/kualta/pomo",
        website: "https://pomo.kualta.dev",
    },
];
