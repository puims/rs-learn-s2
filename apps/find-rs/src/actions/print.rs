use std::{fs, path::Path, time::SystemTime};
use chrono::{DateTime, Local};
use crate::types::{FindResult};

pub fn print_file(path: &Path, format: &str) -> FindResult<()> {
    let metadata = fs::metadata(path)?;
    let filename = path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("");
    
    let parent = path.parent()
        .unwrap_or_else(|| Path::new("."))
        .to_str()
        .unwrap_or("");
    
    let size = metadata.len();
    let modified = metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH);
    
    // 将 SystemTime 转换为可读格式
    let datetime: DateTime<Local> = modified.into();
    let time_str = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
    
    // 支持的占位符映射
    let mut output = format.to_string();
    
    // 一次性处理所有占位符替换
    let replacements = [
        ("%p", path.display().to_string()),
        ("%f", filename.to_string()),
        ("%d", parent.to_string()),
        ("%s", format_size(size)),
        ("%t", time_str),
        ("%T", datetime.format("%H:%M:%S").to_string()),
        ("%D", datetime.format("%Y-%m-%d").to_string()),
        ("%h", format_human_size(size)),
        ("%n", path.file_name().and_then(|n| n.to_str()).unwrap_or("").to_string()),
    ];
    
    for (placeholder, replacement) in replacements {
        output = output.replace(placeholder, &replacement);
    }
    
    println!("{}", output);
    Ok(())
}

fn format_size(bytes: u64) -> String {
    bytes.to_string()
}

fn format_human_size(bytes: u64) -> String {
    const UNITS: [&str; 6] = ["B", "KB", "MB", "GB", "TB", "PB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;
    
    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }
    
    format!("{:.2} {}", size, UNITS[unit_index])
}