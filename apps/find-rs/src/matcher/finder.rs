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

    // 在 Unix 系统上检查隐藏文件
    #[cfg(unix)]
    {
        if let Some(name) = entry.file_name().to_str() {
            !name.starts_with('.')
        } else {
            true
        }
    }
    
    #[cfg(not(unix))]
    {
        true // 在非 Unix 系统上，暂时不过滤隐藏文件
    }
}

fn matches_criteria(path: &Path, cli: &Cli) -> bool {
    if let Some(name) = &cli.criteria.name {
        if !matches_name(path, name, cli.criteria.insensitive) {
            return false;
        }
    }

    if let Some(regex) = &cli.criteria.regex {
        if !matches_regex(path, regex, cli.criteria.insensitive) {
            return false;
        }
    }

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
    
    if let Some(filter_type) = &cli.criteria.filter_type {
        if !filter_type::match_file_type(path, *filter_type) {
            return false;
        }
    }

    true
}

fn matches_name(path: &Path, pattern: &str, insensitive: bool) -> bool {
    if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
        name::match_name(file_name, pattern, insensitive)
    } else {
        false
    }
}

fn matches_regex(path: &Path, pattern: &str, insensitive: bool) -> bool {
    if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
        matcher::regex::simple_pattern_match(file_name, pattern, insensitive)
    } else {
        false
    }
}
