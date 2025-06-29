use std::fs;
use std::process::{Command, Output};

use crate::{config::*, datetime::*, display::*, error::*, file_utils::*, phrases::*, weather::*};

/// 主显示函数，包括:
/// - 今日无记录时，显示:
///   * 日期
///   * 天气
///   * 天气问候语
/// - 今日有记录时，提示清除
/// - 显示待办事项
/// - 显示日记状态
pub async fn handle_main_display(
    app_paths: &AppPaths,
    now: &Now,
    config: &Config,
) -> Result<(), AppError> {
    // 获取语句和url
    let phrases = &config.phrases;
    let url = &config
        .url
        .replace("{0}", &config.api_key)
        .replace("{1}", &config.city);
    if app_paths.daily_file.exists() {
        // 检验文件不存在时
        display_clean(phrases)?;
    } else {
        // 文件存在时，获取天气
        fs::File::create(&app_paths.daily_file)?;
        let (weather, raw_weather) = get_weather(url).await?;
        display_greeting(phrases, now, &weather, &raw_weather)?;
    }

    // 显示待办事项和日记状态
    display_todos(app_paths, phrases)?;
    display_diary(app_paths, phrases)?;

    Ok(())
}

/// 处理命令行参数
pub fn handle_command(
    app_paths: &AppPaths,
    args: &[String],
    config: &Config,
) -> Result<(), AppError> {
    let phrases = &config.phrases;
    // 有命令行参数时
    if args[1] == "clean" {
        let Output {
            status,
            stdout: _,
            stderr: _,
        } = Command::new("sh")
            .arg("-c")
            .arg(format!("rm {}", app_paths.daily_file.display()))
            .output()?;
        match status {
            s if s.success() => println!("{}", get_random_phrase(phrases, "cleaned")?),
            _ => println!("{}", get_random_phrase(phrases, "nothing_to_clean")?),
        }
    } else {
        // ...なに？
        println!("{}", get_random_phrase(phrases, "nani")?);
    }
    Ok(())
}
