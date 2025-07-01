mod config;
mod datetime;
mod display;
mod error;
mod file_utils;
mod handle;
mod phrases;
mod weather;
mod app;

use std::io;

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use ratatui::prelude::CrosstermBackend;
use ratatui::Terminal;

use crate::{app::*, config::*, datetime::*, error::*, file_utils::*, handle::*};

#[tokio::main]
async fn main() -> Result<(), AppError> {
    // 获取当前时间，构建路径，加载配置，命令行参数
    let now: Now = get_time();
    let app_paths = build_path(&now)?;
    let config = load_config(&app_paths)?;
    let args: Vec<String> = std::env::args().collect();

    let output = handle_main_display(&app_paths, &now, &config, &args).await?;
    // print!("{}", output);
    //
    // ratatui渲染
    let app = App::new(&output);
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;
    terminal.draw(|frame| frame.render_widget(&app, frame.area()))?;
    ratatui::restore();
    println!();
    disable_raw_mode()?;
    Ok(())
}
