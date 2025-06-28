use chrono::Local;
use rand::random_range;
use serde_json::Value as JsonValue;
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use toml::{Table, Value};

#[tokio::main]
async fn main() {
    generate_makie().await;
}

async fn generate_makie() {
    // 获取日期
    let now = Local::now();
    let date = now.format("%Y-%m-%d").to_string();
    let got_time: u8 = now.format("%H").to_string().parse().expect("Failed");
    // 将一天分成3个时间段，比较抽象因为这是我的作息()
    let time = String::from(match got_time {
        6..12 => "morning",
        12..19 => "noon",
        19..24 | 0..6 => "night",
        _ => "unknown",
    });
    println!("{}", time);
    // 读取配置文件
    // 包括api_key, city, url和所有语料库phrases
    let mut file = File::open("src/config.toml").expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read file");
    let config = contents.parse::<Table>().unwrap();
    let api_key = config.get("api_key").unwrap().as_str().unwrap();
    let city = config.get("city").unwrap().as_str().unwrap();
    let url = config
        .get("url")
        .unwrap()
        .as_str()
        .unwrap()
        .replace("{0}", api_key)
        .replace("{1}", city);
    let phrases = config.get("phrases").unwrap();
    // .get("morning_clear").unwrap()
    // .get(0).unwrap();
    // 每日检验文件位置
    let dailyfile = PathBuf::from(env::var("HOME").unwrap())
        .join(".daily")
        .join(date.clone() + ".daily");
    // 日记位置
    let diary_path = PathBuf::from(env::var("DNDIARY").unwrap()).join("diary");
    let weather = get_weather(&url).await.unwrap();
    println!("六百六十六: {}", weather);
    println!("含有雨: {:?}", weather.contains("雨"));
    println!("日期检验文件位置: {}", dailyfile.display());
    println!("存在: {}", dailyfile.exists());
    println!("日记位置: {}", diary_path.display());
    println!("phrase: {}", get_random_phrase(phrases, "morning", "clear"));

    println!("================== 下面是输出喵 ==================");
    println!(
        "今天是 {} {}",
        now.format("%Y年%m月%d日"),
        match format!("{}", now.format("%A")).as_str() {
            "Monday" => "星期一...",
            "Tuesday" => "星期二",
            "Wednesday" => "星期三",
            "Thursday" => "星期四",
            "Friday" => "星期五",
            "Saturday" => "星期六!",
            "Sunday" => "星期日~",
            _ => "",
        }
    );
    println!(
        "今日天气: {}{}",
        weather,
        match &weather {
            w if w.contains("晴") => {
                if time == "night" { "🌙" } else { "☀️" }
            }
            w if w.contains("云") => "⛅",
            w if w.contains("阴") => "☁️",
            w if w.contains("雨") => "🌧",
            w if w.contains("雪") => "🌨",
            _ => "",
        }
    );
    println!(
        "{}",
        get_random_phrase(
            phrases,
            &time,
            match &weather {
                w if w.contains("晴") => "clear",
                w if w.contains("云") => "cloudy",
                w if w.contains("阴") => "overcast",
                w if w.contains("雨") => "rainy",
                w if w.contains("雪") => "snowy",
                _ => "unknown",
            }
        )
    );
}

/// 获取天气信息
/// 从api.weatherapi.com获取信息并解析
///
/// 参数:
/// - `url`: 请求的url
///
/// 返回:
/// - `Result<String, Box<dyn std::error::Error>>`: 天气信息
async fn get_weather(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let json: JsonValue = reqwest::get(url).await?.json().await?;
    let text = json["current"]["condition"]["text"]
        .as_str()
        .ok_or("Missing text field")?
        .to_string();
    Ok(text)
}

/// 获取随机语料库
///
/// 参数:
/// - `phrases`: 语料库
/// - `time`: 时间段: [morning, noon, night]
/// - `weather`: 天气: [clear, cloudy, overcast, rainy, snowy]
///
/// 返回:
/// - `String`: 随机语句
fn get_random_phrase(phrases: &Value, time: &str, weather: &str) -> String {
    let words = phrases
        .get(time.to_owned() + "_" + weather)
        .unwrap()
        .as_array()
        .unwrap();
    String::from(
        words
            .get(random_range(..words.len()))
            .unwrap()
            .as_str()
            .unwrap(),
    )
}
