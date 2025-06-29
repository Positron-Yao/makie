use std::error::Error;

// TODO: 定义AppError类型
/// AppError类型
pub type AppError = Box<dyn Error>;
