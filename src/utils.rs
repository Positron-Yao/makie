use rand::random_range;
use serde::Deserialize;
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::error::Error;

#[derive(Deserialize)]
pub struct Config {
    pub api_key: String,
    pub city: String,
    pub url: String,
    pub phrases: HashMap<String, Vec<String>>,
}

/// 获取天气信息
/// 从api.weatherapi.com获取信息并解析
///
/// 参数:
/// - `url`: 请求的url
///
/// 返回:
/// - `Result<String, Box<dyn std::error::Error>>`: 天气信息
pub async fn get_weather(url: &str) -> Result<String, Box<dyn Error>> {
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
/// - `key`: 关键字
///
/// 返回:
/// - `String`: 随机语句
pub fn get_random_phrase(phrases: &HashMap<String, Vec<String>>, key: &str) -> Result<String, Box<dyn Error>> {
    let arr = phrases.get(key).ok_or("Failed")?;
    Ok(String::from(&arr[random_range(..arr.len())]))
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
pub fn get_random_phrase_of_weather(
    phrases: &HashMap<String, Vec<String>>,
    time: &str,
    weather: &str,
) -> Result<String, Box<dyn Error>> {
    let key = format!("{}_{}", time, weather);
    Ok(get_random_phrase(phrases, &key)?)
}
