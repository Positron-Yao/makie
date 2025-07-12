use std::fs;
use std::process::{Command, Output};

use crate::{config::*, datetime::*, display::*, error::*, file_utils::*, phrases::*, weather::*, cli::*};

pub async fn handle_main_display(
    app_paths: &AppPaths,
    now: &Now,
    config: &Config,
    args: &Cli,
) -> Result<String, AppError> {
    if args.command.is_some() {
        // 有命令行参数时
        handle_command(app_paths, args, config)
    } else {
        // 无命令行参数时
        handle_greeting(app_paths, now, config).await
    }
}

/// 主显示函数，包括:
/// - 今日无记录时，显示:
///   * 日期
///   * 天气
///   * 天气问候语
/// - 今日有记录时，提示清除
/// - 显示待办事项
/// - 显示日记状态
pub async fn handle_greeting(
    app_paths: &AppPaths,
    now: &Now,
    config: &Config,
) -> Result<String, AppError> {
    let mut output = Vec::new();
    // 获取语句和url
    let phrases = &config.phrases;
    let url = &config
        .url
        .replace("{0}", &config.api_key)
        .replace("{1}", &config.city);
    if app_paths.daily_file.exists() {
        // 检验文件不存在时
        output.push(display_clean(phrases)?);
    } else {
        // 文件存在时，获取天气
        fs::File::create(&app_paths.daily_file)?;
        let (weather, raw_weather) = get_weather(url).await?;
        output.push(display_greeting(phrases, now, &weather, &raw_weather)?);
        output.push(String::from("\n"));
    }

    // 显示待办事项和日记状态
    output.push(display_todos(app_paths, phrases)?);
    output.push(display_diary(app_paths, phrases)?);

    Ok(output.join(""))
}

/// 处理命令行参数
pub fn handle_command(
    app_paths: &AppPaths,
    args: &Cli,
    config: &Config,
) -> Result<String, AppError> {
    let mut output = Vec::new();
    let phrases = &config.phrases;
    // 有命令行参数时
    if let Some(CliCommand::Clean) = &args.command {
        let Output {
            status,
            stdout: _,
            stderr: _,
        } = Command::new("sh")
            .arg("-c")
            .arg(format!("rm {}", app_paths.daily_file.display()))
            .output()?;
        match status {
            s if s.success() => output.push(get_random_phrase(phrases, "cleaned")?),
            _ => output.push(get_random_phrase(phrases, "nothing_to_clean")?),
        }
    } else {
        // ...なに？
        output.push(get_random_phrase(phrases, "nani")?);
    }
    Ok(output.join("\n"))
}
