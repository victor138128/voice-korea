use by_axum::logger::root;
use tokio::net::TcpListener;

mod common;
mod utils;
mod v1;

#[tokio::main]
async fn main() {
    let log = root();

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
        slog::info!(
            log,
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
        slog::info!(log, "listening on {}", listener.local_addr().unwrap());
        by_axum::serve(listener, app).await.unwrap();
    }
}
