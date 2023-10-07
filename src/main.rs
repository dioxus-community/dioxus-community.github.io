#![allow(non_snake_case)]

use dioxus_router::prelude::*;

use dioxus::prelude::*;
use log::LevelFilter;

mod navbar;
mod routes;

pub use routes::*;

#[cfg(not(debug_assertions))]
pub const BASE_PATH: &str = "/website";

#[cfg(debug_assertions)]
pub const BASE_PATH: &str = "";

fn main() {
    // Init debug
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    console_error_panic_hook::set_once();

    log::info!("starting app");
    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
    render! {
        Router::<Route> {}
    }
}

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[cfg(not(debug_assertions))]
    #[route("/website")]
    Home {},
    #[cfg(debug_assertions)]
    #[route("/")]
    Home {},
    // #[route("/made-with-dioxus")]
    // MadeWithDioxus { },
}
