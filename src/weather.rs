use serde_json::Value as JsonValue;
use std::fmt::{Display, Formatter};

use crate::error::*;

pub enum Weather {
    Clear,
    Cloudy,
    Overcast,
    Rainy,
    Snowy,
    Unknown,
}

impl Display for Weather {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Weather::Clear => write!(f, "clear"),
            Weather::Cloudy => write!(f, "cloudy"),
            Weather::Overcast => write!(f, "overcast"),
            Weather::Rainy => write!(f, "rainy"),
            Weather::Snowy => write!(f, "snowy"),
            Weather::Unknown => write!(f, "unknown"),
        }
    }
}

/// 获取天气信息
/// 从api.weatherapi.com获取信息并解析
///
/// 参数:
/// - `url`: 请求的url
///
/// 返回:
/// - `Result<String, AppError>`: 天气信息
pub async fn get_weather(url: &str) -> Result<(Weather, String), AppError> {
    let json: JsonValue = reqwest::get(url).await?.json().await?;
    let text = json["current"]["condition"]["text"]
        .as_str()
        .ok_or("Missing text field")?
        .to_string();

    Ok((
        match &text.as_str() {
            w if w.contains("晴") => Weather::Clear,
            w if w.contains("云") => Weather::Cloudy,
            w if w.contains("阴") => Weather::Overcast,
            w if w.contains("雨") => Weather::Rainy,
            w if w.contains("雪") => Weather::Snowy,
            _ => Weather::Unknown,
        },
        text.to_string(),
    ))
}

