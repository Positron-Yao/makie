use chrono::Local;
use rand::random_range;
use serde_json::Value as JsonValue;
use std::{env, iter::Map};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use toml::{Table, Value};

#[allow(dead_code)]
#[derive(Debug)]
struct Config {
    api_key: String,
    city: String,
    url: String,
    phrases: Map<String, Vec<String>>,
}

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
    let weekday_code = now.format("%u").to_string().parse::<u8>().expect("Failed");
    // println!("{}", time);
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
    let weather_code = match &weather {
        w if w.contains("晴") => "clear",
        w if w.contains("云") => "cloudy",
        w if w.contains("阴") => "overcast",
        w if w.contains("雨") => "rainy",
        w if w.contains("雪") => "snowy",
        _ => "",
    };
    println!("六百六十六: {}", weather);
    println!("含有雨: {:?}", weather.contains("雨"));
    println!("日期检验文件位置: {}", dailyfile.display());
    println!("存在: {}", dailyfile.exists());
    println!("日记位置: {}", diary_path.display());
    println!("phrase: {}", get_random_phrase_of_weather(phrases, "morning", "clear"));

    println!("================== 下面是输出喵 ==================");
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
    println!(
        "{}",
        get_random_phrase_of_weather(
            phrases,
            &time,
            weather_code
        )
    );
    if rand::random() {
        // 周期问候
        println!("{}", get_random_phrase(phrases, "weekdays"));
    } else {
        // 随机问候
        println!("{}", get_random_phrase(phrases, "roasts"));
    }
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
/// - `arr`: 语料库
///
/// 返回:
/// - `String`: 随机语句
fn get_random_phrase(phrases: &Value, key: &str) -> String {
    let arr = 
        phrases
            .get(key)
            .unwrap()
            .as_array()
            .unwrap();
    String::from(
        arr
            .get(random_range(..arr.len()))
            .unwrap()
            .as_str()
            .unwrap()
        
    )
}

/// 解析天气&时段的特定版本
///
/// 参数:
/// - `phrases`: 语料库
/// - `time`: 时间段: [morning, noon, night]
/// - `weather`: 天气: [clear, cloudy, overcast, rainy, snowy]
///
/// 返回:
/// - `String`: 随机语句
fn get_random_phrase_of_weather(phrases: &Value, time: &str, weather: &str) -> String {
    let key = time.to_owned() + "_" + weather;
    get_random_phrase(phrases, &key)
}

