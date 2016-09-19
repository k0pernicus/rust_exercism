extern crate chrono;
use chrono::*;

pub fn after<T: TimeZone>(start_date: DateTime<T>) -> DateTime<T> {
    start_date + Duration::seconds(1_000_000_000)
}
