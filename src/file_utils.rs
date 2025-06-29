use std::path::PathBuf;
use std::env;
use crate::{error::*, datetime::*};

/// 应用程序路径结构
pub struct AppPaths {
    /// 配置文件路径
    pub config_path: PathBuf,
    /// 每日检验文件位置
    pub daily_file: PathBuf,
    /// 日记文件路径
    pub diary_path: PathBuf,
    /// 待办文件路径
    pub todo_path: PathBuf,
}

/// 构建项目路径
pub fn build_path(now: &Now) -> Result<AppPaths, AppError> {
    let date = get_date(now);

    Ok( AppPaths {
        config_path: PathBuf::from(env::var("HOME")?)
            .join(".config")
            .join("makie")
            .join("config.toml"),
        daily_file: PathBuf::from(env::var("HOME")?)
            .join(".daily")
            .join(date.clone() + ".daily"),
        diary_path: PathBuf::from(env::var("DNDIARY")?)
            .join("diary")
            .join(date.clone() + ".md"),
        todo_path: PathBuf::from(env::var("HOME")?)
            .join("todo.md"),
    })
}
