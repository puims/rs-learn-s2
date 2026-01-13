use std::{path::PathBuf, time::Duration};

use clap::{Parser, ValueEnum, value_parser};
use once_cell::sync::Lazy;
use regex::Regex;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FindError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Invalid regex pattern: {0}")]
    Regex(#[from] regex::Error),

    #[error("Invalid size specification: {0}")]
    SizeSpec(String),

    #[error("Invalid time specification: {0}")]
    TimeSpec(String),

    #[error("No action specified")]
    NoAction,

    #[error("Path error: {0}")]
    PathError(String),

    #[error("Command execution failed: {0}")]
    CommandError(String),

    #[error("Invalid format string: {0}")]
    FormatError(String),
}

pub type FindResult<T> = std::result::Result<T, FindError>;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
// #[command(next_line_help(true))]
pub struct Cli {
    /// The directory to search in 
    #[arg(default_value = ".", value_parser = validate_path)]
    pub path: PathBuf,

    /// Search criteria
    #[command(flatten)]
    pub criteria: SearchCriteria,

    /// Actions to perform on matching files
    #[command(flatten)]
    pub actions: FileActions,
}

#[derive(clap::Args, Clone, Debug)]
pub struct SearchCriteria {
    /// Search for files with a given name (supports wildcard patterns like *.txt)
    #[arg(short, long, group = "matcher")]
    pub name: Option<String>,

    /// Search for files with a given regex pattern
    #[arg(short, long, group = "matcher")]
    pub regex: Option<String>,

    /// Case insensitive search
    #[arg(short, long, requires = "matcher")]
    pub insensitive: bool,

    /// Filter by file size (e.g., +1M, -500K, 100)
    #[arg(short, long, value_parser = parse_size_spec)]
    pub size: Option<SizeSpec>,

    /// Filter by file modification time (e.g., +7d, -30m, 24h)
    #[arg(short, long, value_parser = parse_time_spec)]
    pub mtime: Option<TimeSpec>,

    /// Filter by file type
    #[arg(short = 't', long, value_enum)]
    pub filter_type: Option<FilterType>,
    
    /// Search hidden files and directories
    #[arg(short, long)]
    pub all: bool,
    
    /// Maximum search depth (0 = only current directory)
    #[arg(short, long, value_parser = value_parser!(u8).range(0..=255))]
    pub depth: Option<u8>,

    /// Follow symbolic links
    #[arg(short = 'L', long)]
    pub follow_symlinks: bool,
}

#[derive(clap::Args, Clone, Debug)]
pub struct FileActions {
    /// Print matching files (supports format strings: %p=path, %f=filename, %s=size, %t=mod time)
    #[arg(short, long, default_value= "%p", value_name = "FORMAT")]
    pub print: Option<String>,

    /// Delete matching files (requires confirmation unless --force is used)
    #[arg(long)]
    pub delete: bool,

    /// Force deletion of matching files (no confirmation)
    #[arg(long, requires = "delete")]
    pub force: bool,

    /// Execute a command on matching files (use {} as placeholder for file path)
    #[arg(short = 'x', long, value_name = "COMMAND")]
    pub exec: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
#[allow(dead_code)]
pub enum SizeSpec {
    GreaterThan(u64),
    LessThan(u64),
    Equal(u64),
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub enum TimeSpec {
    NewerThan(Duration),
    OlderThan(Duration),
    Equal(Duration),
}

#[derive(Clone, Copy, Debug, ValueEnum, PartialEq)]
pub enum FilterType {
    #[clap(name = "f")]
    File,
    #[clap(name = "d")]
    Directory,
    #[clap(name = "l")]
    Symlink,
    #[clap(name = "b")]
    BlockDevice,
    #[clap(name = "c")]
    CharDevice,
    #[clap(name = "p")]
    Pipe,
    #[clap(name = "s")]
    Socket,
}

fn validate_path(s: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(s);
    if !path.is_dir() {
        return Err(format!("Path {} is not a directory", s));
    }

    Ok(path)
}

fn parse_size_spec(s: &str) -> Result<SizeSpec, String> {
    let s = s.trim();
    
    let size_pattern = Lazy::new(|| {
        Regex::new(r"^([+-=]?)(\d+)([kKmMgG]?)$").unwrap()
    });
    
    let caps = size_pattern.captures(s).ok_or_else(|| {
        format!("Invalid size specifier: {}", s)
    })?;

    let op = caps.get(1).map(|m| m.as_str()).unwrap_or("=");
    let num_str = caps.get(2).unwrap().as_str();
    let unit = caps.get(3).map(|m| m.as_str().to_ascii_lowercase());

    let num = num_str.parse::<u64>()
        .map_err(|_| "Invalid number format".to_string())?;

    let multiplier = match unit.as_deref() {
        Some("k") => 1024,
        Some("m") => 1024 * 1024,
        Some("g") => 1024 * 1024 * 1024,
        _ => 1,
    };

    let size = num.checked_mul(multiplier).ok_or_else(|| {
        "Size too large".to_string()
    })?;

    match op {
        "+" => Ok(SizeSpec::GreaterThan(size)),
        "-" => Ok(SizeSpec::LessThan(size)),
        "=" => Ok(SizeSpec::Equal(size)),
        _ => Err(format!("Invalid operator: {}", op)),
    }
}

fn parse_time_spec(s: &str) -> Result<TimeSpec, String> {
    let s = s.trim();

    let time_pattern = Lazy::new(|| {
        Regex::new(r"^([+-=]?)(\d+)([smhdwMy])$").unwrap()
    });
    
    let caps = time_pattern.captures(s).ok_or_else(|| {
        format!("Invalid time specifier: {}", s)
    })?;

    let op = caps.get(1).map(|m| m.as_str()).unwrap_or("=");
    let num_str = caps.get(2).unwrap().as_str();
    let unit = caps.get(3).map(|m| m.as_str().to_ascii_lowercase());

    let num = num_str.parse::<u64>()
        .map_err(|_| "Invalid number format".to_string())?;

    let duration = match unit.as_deref() {
        Some("s") => Duration::from_secs(num),
        Some("m") => Duration::from_secs(num * 60),
        Some("h") => Duration::from_secs(num * 60 * 60),
        Some("d") => Duration::from_secs(num * 60 * 60 * 24),
        Some("w") => Duration::from_secs(num * 60 * 60 * 24 * 7),
        _ => Duration::from_secs(num),
    };

    match op {
        "+" => Ok(TimeSpec::NewerThan(duration)),
        "-" => Ok(TimeSpec::OlderThan(duration)),
        "=" => Ok(TimeSpec::Equal(duration)),
        _ => Err(format!("Invalid operator: {}", op)),
    }
}

impl SizeSpec {
    pub fn matches(&self, size: u64) -> bool {
        match self {
            SizeSpec::GreaterThan(n) => size > *n,
            SizeSpec::LessThan(n) => size < *n,
            SizeSpec::Equal(n) => size == *n,
        }
    }
}

impl TimeSpec {
    pub fn matches(&self, age: Duration) -> bool {
        match self {
            TimeSpec::NewerThan(limit) => age < *limit,
            TimeSpec::OlderThan(limit) => age > *limit,
            TimeSpec::Equal(limit) => {
                let tolerance = Duration::from_secs(1);
                age >= *limit - tolerance && age <= *limit + tolerance
            }
        }
    }
}