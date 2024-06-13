// use crate::constants::DATE_FORMAT;
use chrono::{DateTime, Local, NaiveDate, NaiveDateTime, TimeZone};
use regex::Regex;
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

    // Регулярное выражение для поиска дат и времен в строке
    let re = Regex::new(r"(?i)(\d{1,2}[./]\d{1,2}[./]\d{2,4}(?:\s+\d{1,2}:\d{2}:\d{2})?)|(\d{4}-\d{2}-\d{2}(?:\s+\d{2}:\d{2}:\d{2})?)|(\w+\s+\d{1,2},\s+\d{4}(?:\s+\d{2}:\d{2}:\d{2})?)").unwrap();

    if let Some(cap) = re.captures(date_str) {
        for i in 1..=cap.len() {
            if let Some(matched_str) = cap.get(i) {
                let matched_str = matched_str.as_str();

                // Проверка форматов даты без времени
                for &format in &date_formats {
                    if let Ok(naive_date) = NaiveDate::parse_from_str(matched_str, format) {
                        return Local
                            .from_local_datetime(&naive_date.and_hms(0, 0, 0))
                            .single()
                            .ok_or(io::Error::new(ErrorKind::Other, "date parsing error"));
                    }
                }

                // Проверка форматов даты с временем
                for &format in &date_time_formats {
                    if let Ok(naive_date_time) = NaiveDateTime::parse_from_str(matched_str, format)
                    {
                        return Local
                            .from_local_datetime(&naive_date_time)
                            .single()
                            .ok_or(io::Error::new(ErrorKind::Other, "date parsing error"));
                    }
                }
            }
        }
    }

    eprintln!("Error parsing date string: {}", date_str);
    Err(io::Error::new(ErrorKind::Other, "date parsing error"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_various_date_formats() {
        let date_strs = [
            "The meeting is scheduled for 16.12.1999 at 10:30:45.",
            "Conference date: 1999-12-16 14:00:00",
            "Event on 12/16/1999 19:45:30!",
        ];

        for date_str in &date_strs {
            assert!(parse_with_multiple_formats(date_str).is_ok())
        }
    }
}
