use walkdir::WalkDir;

use crate::types::Cli;

pub fn get_walker(cli: &Cli) -> WalkDir {
    let mut walker = WalkDir::new(&cli.path).min_depth(1);

    if let Some(depth) = cli.criteria.depth {
        walker = walker.max_depth(depth as usize)
    }
    
    if cli.criteria.follow_symlinks {
        walker = walker.follow_links(true)
    }

    walker
}
