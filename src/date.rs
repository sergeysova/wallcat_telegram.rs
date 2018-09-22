use chrono::prelude::*;

pub fn now_utc() -> String {
    let utc: DateTime<Utc> = Utc::now();

    utc.to_rfc3339()
}
