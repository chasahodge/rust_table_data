extern crate chrono;

use chrono::prelude::*;

pub fn get_a_date(some_date: String) -> Date<Local> {
    let sdate: Vec<&str> = some_date.split("/").collect();
    let month: String = sdate[0].to_string();
    let day: String = sdate[1].to_string();
    let year: String = sdate[2].to_string();
    let mm: u32 = month.parse().unwrap();
    let dd: u32 = day.parse().unwrap();
    let yyyy: i32 = year.parse().unwrap();
    let new_date = Local.ymd(yyyy, mm, dd);

    return new_date
}
