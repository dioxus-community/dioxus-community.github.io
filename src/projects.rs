use crate::models::project_static;
use crate::models::Project;

pub const PROJECTS: &'static [Project<'static>] = &[
    project_static! {
        name: "Uplink",
        description: "Secure, encrypted, P2P chat app",
        repository_url: "https://github.com/Satellite-im/Uplink",
        website: "https://uplink.satellite.im",
    },
    project_static! {
        name: "Ebou",
        description: "Cross platform Mastodon Client",
        repository_url: "https://github.com/terhechte/Ebou",
        website: "https://terhech.de/ebou",
    },
    project_static! {
        name: "Ambient",
        description: "Multiplayer game engine that runs in the browser",
        repository_url: "https://github.com/ambientrun/ambient",
        website: "https://ambient.run",
    },
    project_static! {
        name: "Cloak",
        description: "Secrets automation for developers",
        repository_url: "https://github.com/purton-tech/cloak",
        website: "https://cloak.software",
    },
    project_static! {
        name: "Hemi",
        description: "Typing tutor for training hands separately",
        repository_url: "https://github.com/kualta/hemi",
        website: "https://hemi.kualta.dev",
    },
    project_static! {
        name: "InterSpace",
        description: "The graphical interface pipeline visualized",
        repository_url: "https://github.com/erithax/interspace",
        website: "https://erithax.com",
    },
    project_static! {
        name: "Patina",
        description: "Simple text editor with Rust syntax highlighting",
        repository_url: "https://github.com/ealmloff/Patina",
        website: "https://ealmloff.github.io/Patina",
    },
    project_static! {
        name: "Pomo",
        description: "Minimalistic Pomodoro timer",
        repository_url: "https://github.com/kualta/pomo",
        website: "https://pomo.kualta.dev",
    },
];
