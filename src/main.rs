mod config;
mod datetime;
mod file_utils;
mod phrases;
mod weather;
mod error;
mod handle;
mod display;

use crate::{config::*, datetime::*, file_utils::*, error::*, handle::*};

#[tokio::main]
#[allow(dead_code)]
async fn main() -> Result<(), AppError> {
    // 获取当前时间，构建路径，加载配置，命令行参数
    let now: Now = get_time();
    let app_paths = build_path(&now)?;
    let config = load_config(&app_paths)?;
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 1 {
        // 无命令行参数时
        handle_main_display(&app_paths, &now, &config).await?;
    } else {
        // 有命令行参数时
        handle_command(&app_paths, &args, &config)?;
    }

    Ok(())
}

