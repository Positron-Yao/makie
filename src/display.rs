use regex::{self, Regex};
use std::fs;

use crate::{config::*, datetime::*, error::*, file_utils::*, phrases::*, weather::*};

/// 主显示函数，包括
/// - 日期
/// - 天气
/// - 问候语
/// - 星期/随机问候语
pub fn display_greeting(
    phrases: &Phrases,
    now: &Now,
    weather: &Weather,
    raw_weather: &str,
) -> Result<String, AppError> {
    Ok([
        display_datetime(now),
        display_weather(weather, raw_weather, now),
        get_random_phrase_of_weather(phrases, now, weather)?,
        String::from("\n"),
        if rand::random() {
            // 周期问候
            get_random_phrase(phrases, "weekdays")?
        } else {
            // 随机问候
            get_random_phrase(phrases, "roasts")?
        },
    ].join(""))
}

/// 解析todo.md文件
pub fn display_todos(app_paths: &AppPaths, phrases: &Phrases) -> Result<String, AppError> {
    let mut output = Vec::new();
    // byd不用glow做了架绷
    // 还是几把得自己弄
    let todo_content = fs::read_to_string(&app_paths.todo_path)?;
    let re_todo = Regex::new(r"\[(TODO|ALRT)\].*")?;
    let mut got_todo = Vec::new();
    for cap in re_todo.captures_iter(&todo_content) {
        got_todo.push(String::from("  • "));
        got_todo.push(cap[0].to_string());
        got_todo.push(String::from("\n"));
    }
    output.push(get_random_phrase(phrases, "todo")?
        .replace("%d", &(got_todo.len() / 3).to_string())
    );
    output.push(String::new());
    output.push(got_todo.join(""));
    Ok(output.join("\n"))
}

/// 检验日记文件是否存在，并输出提示
pub fn display_diary(app_paths: &AppPaths, phrases: &Phrases) -> Result<String, AppError> {
    if !app_paths.diary_path.exists() {
        Ok(format!("{}\n", get_random_phrase(phrases, "dn")?))
    } else {
        Ok(String::from("\n"))
    }
}

/// 显示clean提示
pub fn display_clean(phrases: &Phrases) -> Result<String, AppError> {
    Ok(format!("{}\n", get_random_phrase(phrases, "makie_clean")?))
}

/// 显示时间
pub fn display_datetime(now: &Now) -> String {
    format!("今天是 {} {}\n", now.format("%Y年%m月%d日"), get_weekday(now))
}

/// 显示天气信息与emoji
pub fn display_weather(weather: &Weather, raw_weather: &str, now: &Now) -> String {
    format!(
        "今日天气: {}{}\n",
        raw_weather,
        match weather {
            Weather::Clear => {
                match get_time_section(now) {
                    TimeSection::Night => "🌙",
                    _ => "☀️",
                }
            }
            Weather::Cloudy => "⛅",
            Weather::Overcast => "☁️",
            Weather::Rainy => "🌧",
            Weather::Snowy => "🌨",
            _ => "",
        }
    )
}
