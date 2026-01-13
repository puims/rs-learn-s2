use std::{fs::metadata, path::Path};

use crate::types::SizeSpec;

pub fn match_size(path: &Path, size_spec: &SizeSpec) -> bool {
    match metadata(path) {
        Ok(metadata) => {
            size_spec.matches(metadata.len())
        },
        Err(_) => false,
    }
}