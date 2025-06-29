use regex::{self, Regex};
use std::fs;

use crate::{config::*, datetime::*, file_utils::*, error::*, weather::*, phrases::*};

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
) -> Result<(), AppError> {
    display_datetime(now);
    display_weather(weather, raw_weather, now);
    println!(
        "{}",
        get_random_phrase_of_weather(phrases, now, weather)?
    );

    // 星期/随机问候语
    if rand::random() {
        // 周期问候
        println!("{}", get_random_phrase(phrases, "weekdays")?);
    } else {
        // 随机问候
        println!("{}", get_random_phrase(phrases, "roasts")?);
    }

    Ok(())
}

/// 解析todo.md文件
pub fn display_todos(
    app_paths: &AppPaths,
    phrases: &Phrases,
) -> Result<(), AppError> {
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
    println!("{}", get_random_phrase(phrases, "todo")?.replace("%d", &(got_todo.len() / 3).to_string()));
    println!("\n{}", got_todo.join(""));
    Ok(())
}

/// 检验日记文件是否存在，并输出提示
pub fn display_diary(
    app_paths: &AppPaths,
    phrases: &Phrases,
) -> Result<(), AppError> {
    if !app_paths.diary_path.exists() {
        println!("{}", get_random_phrase(phrases, "dn")?);
    }
    Ok(())
}

/// 显示clean提示
pub fn display_clean(phrases: &Phrases) -> Result<(), AppError> {
    println!("{}", get_random_phrase(phrases, "makie_clean")?);
    Ok(())
}

/// 显示时间
pub fn display_datetime(now: &Now) {
    println!(
        "今天是 {} {}",
        now.format("%Y年%m月%d日"),
        get_weekday(now)
    )
}
