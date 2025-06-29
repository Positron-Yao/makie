use chrono::{DateTime, Datelike, Local, Timelike, Weekday};
use std::fmt::{Display, Formatter};

/// 本地时间类型
pub type Now = DateTime<Local>;

/// 时间段类型
pub enum TimeSection {
    Morning,
    Noon,
    Night,
}

impl Display for TimeSection {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            TimeSection::Morning => write!(f, "morning"),
            TimeSection::Noon => write!(f, "noon"),
            TimeSection::Night => write!(f, "night"),
        }
    }
}

/// 获取当前时间
pub fn get_time() -> Now {
    Local::now()
}

/// 获取当前日期
pub fn get_date(now: &Now) -> String {
    now.format("%Y-%m-%d").to_string()
}

/// 获取星期几
pub fn get_weekday(now: &Now) -> String {
    match now.weekday() {
        Weekday::Mon => "星期一...".to_string(),
        Weekday::Tue => "星期二".to_string(),
        Weekday::Wed => "星期三".to_string(),
        Weekday::Thu => "星期四".to_string(),
        Weekday::Fri => "星期五".to_string(),
        Weekday::Sat => "星期六!".to_string(),
        Weekday::Sun => "星期日~".to_string(),
    }
}

/// 获取时间段
pub fn get_time_section(now: &Now) -> TimeSection {
    match now.hour() {
        0..=5 => TimeSection::Night,
        6..=11 => TimeSection::Morning,
        12..=18 => TimeSection::Noon,
        19..=23 => TimeSection::Night,
        _ => TimeSection::Night,
    }
}

