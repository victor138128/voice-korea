use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod common;
mod utils;
mod v1;

#[tokio::main]
async fn main() {
    let _ = tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_env("LOG_LEVEL")
                .unwrap_or(tracing_subscriber::EnvFilter::new("debug")),
        )
        .with(tracing_subscriber::fmt::layer())
        .try_init();

    let app = by_axum::new().nest("/v1", v1::router());
    #[cfg(feature = "reload")]
    {
        use listenfd::ListenFd;
        let mut listenfd = ListenFd::from_env();
        let listener = match listenfd.take_tcp_listener(0).unwrap() {
            Some(listener) => {
                listener.set_nonblocking(true).unwrap();
                TcpListener::from_std(listener).unwrap()
            }
            None => {
                eprintln!("LISTENFD ERROR");
                return;
            }
        };
        tracing::info!(
            "[AUTO-RELOAD] listening on {}",
            listener.local_addr().unwrap()
        );
        by_axum::serve(listener, app).await.unwrap();
    }
    #[cfg(not(feature = "reload"))]
    {
        let port = option_env!("PORT").unwrap_or("3000");
        let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
            .await
            .unwrap();
        tracing::info!("listening on {}", listener.local_addr().unwrap());
        by_axum::serve(listener, app).await.unwrap();
    }
}
