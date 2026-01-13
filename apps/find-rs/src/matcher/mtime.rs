use std::{fs::metadata, path::Path, time::SystemTime};

use crate::types::TimeSpec;

pub fn match_mtime(path: &Path, time_spec: &TimeSpec) -> bool {
    match metadata(path) {
        Ok(metadata) => {
            let modified_time = metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH);
            let now = SystemTime::now();

            match now.duration_since(modified_time) {
                Ok(age) => time_spec.matches(age),
                Err(_) => false,
            }
        },
        Err(_) => false,
    }
}