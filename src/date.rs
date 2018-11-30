use chrono::prelude::*;

pub fn now_utc() -> String {
    let utc: DateTime<Utc> = Utc::now();

    utc.to_rfc3339()
}

pub fn now_readable() -> String {
    let utc: DateTime<Utc> = Utc::now();

    utc.format("%d.%m.%Y").to_string()
}
