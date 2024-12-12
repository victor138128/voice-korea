use chrono::{DateTime, FixedOffset, Utc};
use lambda_http::{
    lambda_runtime::streaming::Body, run, service_fn, Error, Request, RequestPayloadExt, Response,
};
use reqwest::StatusCode;
use serde::Deserialize;
use serde_json::Value;
use watcher::*;

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(handler)).await
}

#[derive(Deserialize, Debug)]
struct EventBridgeEvent {
    source: Option<String>,
}

async fn handler(event: Request) -> Result<Response<Body>, Error> {
    let payload = event.payload::<Value>().unwrap_or_default();

    // Cron Event
    if let Some(payload) = payload {
        if let Ok(event_bridge_event) = serde_json::from_value::<EventBridgeEvent>(payload.clone())
        {
            if event_bridge_event.source.as_deref() == Some("aws.events") {
                let utc_now: DateTime<Utc> = Utc::now();
                let kst_offset = FixedOffset::east_opt(9 * 3600).unwrap();
                let kst_time = utc_now.with_timezone(&kst_offset);
                let formatted_date = kst_time.format("%y-%m-%d").to_string();
                match Watcher::new() {
                    Ok(v) => match v.finalize_survey(formatted_date.clone()).await {
                        Ok(v) => {
                            return Ok(response(
                                StatusCode::OK,
                                format!("Finalized({}) {:?}", formatted_date, v),
                            ));
                        }
                        Err(e) => {
                            return Ok(response(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()));
                        }
                    },
                    Err(e) => {
                        return Ok(response(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()));
                    }
                };
            }
        }
    }

    //Version
    let method = event.method().to_string();
    let path = event.uri().path().to_string();
    if path == "/version" && method == "GET" {
        let version = match option_env!("VERSION") {
            Some(version) => match option_env!("COMMIT") {
                Some(commit) => format!("{}-{}", version, commit),
                None => version.to_string(),
            },
            None => match option_env!("DATE") {
                Some(date) => date.to_string(),
                None => "unknown".to_string(),
            },
        }
        .to_string();
        return Ok(response(StatusCode::OK, version));
    }

    Ok(response(StatusCode::NOT_FOUND, String::default()))
}

fn response(status_code: StatusCode, message: String) -> Response<Body> {
    Response::builder()
        .status(status_code)
        .body(Body::from(message))
        .unwrap()
}
