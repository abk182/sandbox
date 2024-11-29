// use crate::constants::DATE_FORMAT;
use chrono::{DateTime, Local, NaiveDate, NaiveDateTime, TimeZone};
use std::io::{self, ErrorKind};

pub fn parse_with_multiple_formats(date_str: &str) -> Result<DateTime<Local>, io::Error> {
    let date_formats = [
        "%d.%m.%Y",
        "%Y-%m-%d",
        "%m/%d/%Y",
        "%B %d, %Y",
        "%d %B %Y",
        "%d %b %Y",
    ];

    let date_time_formats = [
        "%d.%m.%Y %H:%M:%S",
        "%Y-%m-%d %H:%M:%S",
        "%m/%d/%Y %H:%M:%S",
        "%B %d, %Y %H:%M:%S",
        "%d %B %Y %H:%M:%S",
        "%d %b %Y %H:%M:%S",
    ];

    // Проверка форматов даты без времени
    for &format in &date_formats {
        if let Ok(naive_date) = NaiveDate::parse_from_str(date_str, format) {
            return Local
                .from_local_datetime(&naive_date.and_hms(0, 0, 0))
                .single()
                .ok_or(io::Error::new(ErrorKind::Other, "date parsing error"));
        }
    }

    // Проверка форматов даты с временем
    for &format in &date_time_formats {
        if let Ok(naive_date_time) = NaiveDateTime::parse_from_str(date_str, format) {
            return Local
                .from_local_datetime(&naive_date_time)
                .single()
                .ok_or(io::Error::new(ErrorKind::Other, "date parsing error"));
        }
    }

    Err(io::Error::new(
        ErrorKind::Other,
        format!("Error parsing date string: {date_str}"),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_various_date_formats() {
        let date_strs = [
            "16.12.1999",
            "16.12.1999 10:30:45",
            "1999-12-16 14:00:00",
            "12/16/1999 19:45:30",
        ];

        for date_str in &date_strs {
            assert!(parse_with_multiple_formats(date_str).is_ok())
        }
    }
}
