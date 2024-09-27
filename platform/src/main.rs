#![allow(non_snake_case)]

pub mod prelude {
    pub use crate::layouts::root_layout::*;
    pub use crate::routes::*;
    pub use crate::utils::context::*;

    pub use crate::pages::not_found::NotFoundPage;
}
use dioxus_logger::tracing;

pub mod pages {
    pub mod not_found;
}

pub mod presentations {
    pub mod create;
    pub mod dashboard;
    pub mod login;
}

pub mod models {
    pub mod survey;
}

pub mod utils {
    pub mod context;
}

pub mod layouts {
    pub mod root_layout;
}

pub mod components {
    pub mod bottom;
    pub mod input;
}

pub mod api;
pub mod routes;
use dioxus::prelude::*;

use crate::{routes::Route, utils::context::use_iitp_context_provider};

fn main() {
    rust_common::base::launch(App);
}

fn App() -> Element {
    use_iitp_context_provider();

    rsx! {
        ErrorBoundary {
            handle_error: |error| {
                tracing::error!("Error: {:?}", error);
                rsx! {
                    div { "Hmm, something went wrong. Prease report {error}" }
                }
            },
            Router::<Route> {}
        }
    }
}
