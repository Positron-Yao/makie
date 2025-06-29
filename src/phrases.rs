use rand::random_range;

use crate::{config::*, datetime::*, error::*, weather::*};

/// 获取随机语料库
///
/// 参数:
/// - `arr`: 语料库
/// - `key`: 关键字
///
/// 返回:
/// - `String`: 随机语句
pub fn get_random_phrase(phrases: &Phrases, key: &str) -> Result<String, AppError> {
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
    phrases: &Phrases,
    now: &Now,
    weather: &Weather,
) -> Result<String, AppError> {
    let key = match weather {
        Weather::Unknown => "unknown_weather".to_string(),
        _ => format!("{}_{}", get_time_section(now), weather),
    };
    get_random_phrase(phrases, &key)
}
