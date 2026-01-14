use std::time::Instant;

mod matcher;
mod actions;
mod types;

pub use types::{Cli, FindError, FindResult};

pub fn execute() -> FindResult<()> {
    let start_time = Instant::now();
    
    let cli = matcher::parser::parse_cli();
    
    // 调试：打印解析后的参数
    if std::env::var("FIND_RS_DEBUG").is_ok() {
        eprintln!("Debug: CLI args: {:?}", cli);
        eprintln!("Debug: Searching in: {}", cli.path.display());
        if let Some(ref name) = cli.criteria.name {
            eprintln!("Debug: Name pattern: '{}'", name);
        }
        if let Some(ref filter_type) = cli.criteria.filter_type {
            eprintln!("Debug: Filter type: {:?}", filter_type);
        }
    }
    
    let walker = matcher::walker::get_walker(&cli);
    
    if std::env::var("FIND_RS_DEBUG").is_ok() {
        eprintln!("Debug: Walker created, starting search...");
    }
    
    let matches = matcher::finder::find_files(walker, &cli)?;
    
    if matches.is_empty() {
        eprintln!("No files found matching the given criteria");
        
        // 调试：提供一些建议
        if std::env::var("FIND_RS_DEBUG").is_ok() {
            eprintln!("Debug: Consider using --all to include hidden files");
            eprintln!("Debug: Consider checking the search path: {}", cli.path.display());
            eprintln!("Debug: Consider using a simpler pattern");
        }
    } else {
        eprintln!("Found {} matching file(s)", matches.len());
        actions::do_action(&matches, &cli)?;
    }
    
    if std::env::var("FIND_RS_DEBUG").is_ok() {
        eprintln!("Execution completed in {:?}", start_time.elapsed());
    }
    
    Ok(())
}