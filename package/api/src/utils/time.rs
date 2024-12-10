use chrono::{DateTime, Utc};

pub fn convert_rfc3339_to_timestamp_milllis(iso_str: &str) -> Option<i64> {
    DateTime::parse_from_rfc3339(iso_str)
        .ok()
        .map(|dt| dt.with_timezone(&Utc).timestamp_millis())
}
