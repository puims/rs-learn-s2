use std::{fs, io, path::Path};

use crate::types::{FindError, FindResult};

pub fn delete_file(path: &Path, force: bool) -> FindResult<()>{
    if !force {
        print!("Delete {}? (y/N): ", path.display());
        let _ = io::Write::flush(&mut io::stdout());
        
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_ok() {
            if input.trim().to_lowercase() != "y" {
                return Err(FindError::NoAction);
            }
        }
    }
    
    if path.is_dir() {
        if let Err(e) = fs::remove_dir_all(path) {
            eprintln!("Failed to delete directory {}: {}", path.display(), e);
        }
    } else if let Err(e) = fs::remove_file(path) {
        eprintln!("Failed to delete file {}: {}", path.display(), e);
    }

    Ok(())
}