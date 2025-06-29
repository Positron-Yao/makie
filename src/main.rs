mod config;
mod datetime;
mod display;
mod error;
mod file_utils;
mod handle;
mod phrases;
mod weather;

use crate::{config::*, datetime::*, error::*, file_utils::*, handle::*};

#[tokio::main]
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
