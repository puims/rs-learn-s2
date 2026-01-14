use std::{path::{Path, PathBuf}};

use walkdir::{DirEntry, WalkDir};

use crate::{matcher, matcher::{filter_type, mtime::match_mtime, name, size::match_size}, types::{Cli, FindResult}};

pub fn find_files(walker: WalkDir, cli: &Cli) -> FindResult<Vec<PathBuf>> {
    let matches = walker
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|entry| filter_hidden(entry, cli))
        .map(|entry| entry.path().to_path_buf())
        .filter(|path| matches_criteria(path, cli))
        .collect();

    Ok(matches)
}

fn filter_hidden(entry: &DirEntry, cli: &Cli) -> bool {
    if cli.criteria.all {
        return true;
    }
    
    // 检查是否是隐藏文件/目录
    if let Some(file_name) = entry.file_name().to_str() {
        // 在Unix系统上，以点开头的文件是隐藏文件
        if file_name.starts_with('.') {
            return false;
        }
        
        // 检查文件所在的目录是否有隐藏的父目录
        // 注意：这里我们只检查当前条目本身，不递归检查父目录
        // 如果需要，可以添加递归检查
        true
    } else {
        // 如果文件名不是有效的UTF-8，我们不过滤它
        true
    }
}

// 优化的匹配函数
fn matches_criteria(path: &Path, cli: &Cli) -> bool {
    // 调试：打印正在检查的路径
    if std::env::var("FIND_RS_DEBUG").is_ok() {
        eprintln!("Debug: Checking path: {}", path.display());
    }
    
    // 先检查文件类型（如果指定了的话）
    if let Some(filter_type) = &cli.criteria.filter_type {
        if !filter_type::match_file_type(path, *filter_type) {
            if std::env::var("FIND_RS_DEBUG").is_ok() {
                eprintln!("Debug: Failed file type filter");
            }
            return false;
        }
    }
    
    // 然后检查名称匹配（如果指定了的话）
    if let Some(name) = &cli.criteria.name {
        if !matches_name(path, name, cli.criteria.insensitive) {
            if std::env::var("FIND_RS_DEBUG").is_ok() {
                eprintln!("Debug: Failed name filter: pattern={}, path={}", name, path.display());
            }
            return false;
        }
    }
    
    // 检查正则表达式匹配
    if let Some(regex) = &cli.criteria.regex {
        if !matches_regex(path, regex, cli.criteria.insensitive) {
            return false;
        }
    }
    
    // 最后检查需要文件系统操作的条件
    if let Some(size_spec) = &cli.criteria.size {
        if !match_size(path, size_spec) {
            return false;
        }
    }
    
    if let Some(time_spec) = &cli.criteria.mtime {
        if !match_mtime(path, time_spec) {
            return false;
        }
    }
    
    true
}

fn matches_name(path: &Path, pattern: &str, insensitive: bool) -> bool {
    // 首先检查路径是否有文件名
    let Some(file_name) = path.file_name() else {
        return false;
    };
    
    // 尝试转换为字符串
    let Some(file_name_str) = file_name.to_str() else {
        // 如果文件名不是有效的UTF-8，使用原始字节进行比较
        return matches_name_bytes(file_name, pattern, insensitive);
    };
    
    use crate::matcher::name::match_name;
    match_name(file_name_str, pattern, insensitive)
}

/// 处理非UTF-8文件名的匹配
fn matches_name_bytes(file_name: &std::ffi::OsStr, pattern: &str, insensitive: bool) -> bool {
    // 如果大小写敏感，直接比较字节
    if !insensitive {
        return file_name == pattern;
    }
    
    // 对于大小写不敏感，我们进行简单处理
    // 注意：这是一个简化的实现，不处理所有Unicode情况
    let pattern_lower = pattern.to_lowercase();
    if let Some(file_name_str) = file_name.to_str() {
        file_name_str.to_lowercase().contains(&pattern_lower)
    } else {
        false
    }
}

fn matches_regex(path: &Path, pattern: &str, insensitive: bool) -> bool {
    if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
        matcher::regex::regex_match(file_name, pattern, insensitive)
    } else {
        false
    }
}
