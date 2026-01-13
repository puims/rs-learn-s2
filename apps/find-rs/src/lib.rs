use std::time::Instant;

use crate::types::FindResult;

mod matcher;
mod actions;
mod types;

pub fn execute() -> FindResult<()> {
    let start_time = Instant::now();

    let cli = matcher::parser::parse_cli();
    let walker = matcher::walker::get_walker(&cli);
    let matches = matcher::finder::find_files(walker, &cli)?;
    
    if matches.is_empty() {
        eprintln!("No files found matching the given criteria");
    }else {
        actions::do_action(&matches, &cli)?;
    }

    if std::env::var("FIND_AS_DEBUG").is_ok() {
        eprintln!("Execution time: {:?}", start_time.elapsed());
    }

    Ok(())
}