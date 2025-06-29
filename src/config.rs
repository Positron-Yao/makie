use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use crate::{error::*, file_utils::*};

/// 语料库类型
pub type Phrases = HashMap<String, Vec<String>>;

#[derive(Deserialize)]
/// 配置文件结构
pub struct Config {
    pub api_key: String,
    pub city: String,
    pub url: String,
    pub phrases: Phrases,
}

/// 读取并解析配置文件
pub fn load_config(app_paths : &AppPaths) -> Result<Config, AppError> {
    // 读取配置文件
    // 包括api_key, city, url和所有语料库phrases
    let toml_str = fs::read_to_string(&app_paths.config_path)?;
    let config: Config = toml::from_str(&toml_str)?;
    Ok(config)
}
