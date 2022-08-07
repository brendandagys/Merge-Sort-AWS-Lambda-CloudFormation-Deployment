use chrono::{DateTime, Utc};

pub fn print_time(prefix: &str) -> DateTime<Utc> {
    let now = Utc::now();
    println!("{prefix}: {} {}", now.date(), now.time());
    now
}
