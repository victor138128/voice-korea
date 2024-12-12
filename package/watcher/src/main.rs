use by_axum::log::root;
use chrono::{DateTime, FixedOffset, Utc};
use tokio::net::TcpListener;
use watcher::*;

#[tokio::main]
async fn main() -> Result<()> {
    let app = by_axum::new();
    let port = option_env!("PORT").unwrap_or("4000");
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    slog::info!(root(), "listening on {}", listener.local_addr().unwrap());
    by_axum::serve(listener, app).await.unwrap();
    let log = root();
    let client = Watcher::new()?;
    let utc_now: DateTime<Utc> = Utc::now();
    let kst_offset = FixedOffset::east_opt(9 * 3600).unwrap();
    let kst_time = utc_now.with_timezone(&kst_offset);
    let formatted_date = kst_time.format("%y-%m-%d").to_string();

    let res = client.finalize_survey(formatted_date.clone()).await;
    slog::info!(
        log,
        "Voice-Korea-Watcher({}) Result({:?})",
        formatted_date,
        res
    );
    Ok(())
}
