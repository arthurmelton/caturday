use std::thread::sleep;

use chrono::{DateTime, Datelike, Duration, NaiveDate, Utc};

pub fn wait_till_next_saturday() {
    let time = Utc::now();
    let start = time.timestamp();
    let day_of_the_week = time.weekday().num_days_from_sunday();
    let next_saturday = 6 - day_of_the_week as i32;
    let new_time = DateTime::<Utc>::from_utc(
        NaiveDate::from_ymd(time.year(), time.month(), time.day()).and_hms_milli(0, 0, 0, 0),
        Utc,
    ) + Duration::days(next_saturday as i64);
    sleep(std::time::Duration::from_secs(
        (new_time.timestamp() - start) as u64,
    ));
}
