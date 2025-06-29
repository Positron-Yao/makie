use chrono::Local;
use regex::{self, Regex};
use std::env;
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Output};

mod utils;
use utils::{Config, get_random_phrase, get_random_phrase_of_weather, get_weather};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args();
    // 获取日期
    let now = Local::now();
    let date = now.format("%Y-%m-%d").to_string();
    let got_time: u8 = now.format("%H").to_string().parse()?;
    // 每日检验文件位置
    let daily_file = PathBuf::from(env::var("HOME")?)
        .join(".daily")
        .join(date.clone() + ".daily");
    // 日记位置
    let diary_path = PathBuf::from(env::var("DNDIARY")?)
        .join("diary")
        .join(date.clone() + ".md");
    let config_file = PathBuf::from(env::var("HOME")?)
        .join(".config")
        .join("makie")
        .join("config.toml");
    let todo_file = PathBuf::from(env::var("HOME")?)
        .join("todo.md");
    // 将一天分成3个时间段，比较抽象因为这是我的作息()
    let time = String::from(match got_time {
        6..12 => "morning",
        12..19 => "noon",
        19..24 | 0..6 => "night",
        _ => "unknown",
    });
    let weekday_code = now.format("%u").to_string().parse::<u8>()?;
    // 读取配置文件
    // 包括api_key, city, url和所有语料库phrases
    let toml_str = fs::read_to_string(&config_file)?;
    let Config {
        api_key,
        city,
        url,
        phrases,
    } = toml::from_str(&toml_str)?;
    let url_rep = url.replace("{0}", &api_key).replace("{1}", &city);
    if args.len() == 1 {
        // 检测检验文件是否存在
        if daily_file.exists() {
            println!("{}", get_random_phrase(&phrases, "makie_clean")?);
        } else {
            // 检验文件不存在，则创建
            let _ = fs::File::create(daily_file);
            let weather = get_weather(&url_rep).await?;
            let weather_code = match &weather {
                w if w.contains("晴") => "clear",
                w if w.contains("云") => "cloudy",
                w if w.contains("阴") => "overcast",
                w if w.contains("雨") => "rainy",
                w if w.contains("雪") => "snowy",
                _ => "",
            };
            // ================== 下面是输出喵 ==================
            // 输出日期
            println!(
                "今天是 {} {}",
                now.format("%Y年%m月%d日"),
                match weekday_code {
                    1 => "星期一...",
                    2 => "星期二",
                    3 => "星期三",
                    4 => "星期四",
                    5 => "星期五",
                    6 => "星期六!",
                    7 => "星期日~",
                    _ => "",
                }
            );

            // 输出天气
            println!(
                "今日天气: {}{}",
                weather,
                match weather_code {
                    "clear" => {
                        if time == "night" { "🌙" } else { "☀️" }
                    }
                    "cloudy" => "⛅",
                    "overcast" => "☁️",
                    "rainy" => "🌧",
                    "snowy" => "🌨",
                    _ => "",
                }
            );

            // 天气问候语
            println!(
                "{}",
                get_random_phrase_of_weather(&phrases, &time, weather_code)?
            );

            // 星期/随机问候语
            if rand::random() {
                // 周期问候
                println!("{}", get_random_phrase(&phrases, "weekdays")?);
            } else {
                // 随机问候
                println!("{}", get_random_phrase(&phrases, "roasts")?);
            }
        }
        // TODO: 解析todo文件
        // byd不用glow做了架绷
        // 还是几把得自己弄
        let todo_content = fs::read_to_string(&todo_file)?;
        let re_todo = Regex::new(r"\[(TODO|ALRT)\].*")?;
        let mut got_todo = Vec::new();
        for cap in re_todo.captures_iter(&todo_content) {
            got_todo.push(String::from("  • "));
            got_todo.push(cap[0].to_string());
            got_todo.push(String::from("\n"));
        }
        println!("{}", get_random_phrase(&phrases, "todo")?.replace("%d", &(got_todo.len() / 3).to_string()));
        println!("\n{}", got_todo.join(""));

        // 检验日记文件是否存在
        if !diary_path.exists() {
            println!("{}", get_random_phrase(&phrases, "dn")?);
        }
    } else {
        // 有命令行参数时
        if args.nth(1).unwrap() == "clean" {
            let Output {
                status,
                stdout: _,
                stderr: _,
            } = Command::new("sh")
                .arg("-c")
                .arg(format!("rm {}", daily_file.display()))
                .output()
                ?;
            match status {
                s if s.success() => println!("{}", get_random_phrase(&phrases, "cleaned")?),
                _ => println!("{}", get_random_phrase(&phrases, "nothing_to_clean")?),
            }
        } else {
            // ...なに？
            println!("{}", get_random_phrase(&phrases, "nani")?);
        }
    }

    Ok(())
}
