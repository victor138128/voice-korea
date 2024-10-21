#![allow(non_snake_case)]

use dioxus_logger::tracing::{self, Level};

use dioxus::prelude::*;

use voice_korea::{routes::Route, utils::context::use_iitp_context_provider};

fn main() {
    dioxus_logger::init(match option_env!("LOG_LEVEL") {
        Some("trace") => Level::TRACE,
        Some("debug") => Level::DEBUG,
        Some("info") => Level::INFO,
        Some("warn") => Level::WARN,
        Some("error") => Level::ERROR,
        _ => Level::INFO,
    })
    .expect("failed to init logger");

    tracing::info!("starting app");
    dioxus_aws::launch(App);
}

fn App() -> Element {
    use_iitp_context_provider();

    rsx! {
        head {
            link {
                rel: "icon",
                r#type: "image/x-icon",
                href: "/favicon.ico",
            }
            link {
                rel: "stylesheet",
                href: "/main.css",
            }
            link {
                rel: "stylesheet",
                href: "/tailwind.css",
            }
            load_tailwindcss {}
        }
        Router::<Route> {}
    }
}

#[cfg(not(feature = "lambda"))]
#[allow(dead_code)]
fn load_tailwindcss() -> Element {
    rsx! {
        script {
            src: "https://cdn.tailwindcss.com/3.4.5",
        }
    }
}

#[cfg(feature = "lambda")]
#[allow(dead_code)]
fn load_tailwindcss() -> Element {
    rsx! {}
}
