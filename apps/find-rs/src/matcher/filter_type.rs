use std::path::Path;
use crate::types::FilterType;

pub fn match_file_type(path: &Path, filter_type: FilterType) -> bool {
    match std::fs::symlink_metadata(path) {
        Ok(metadata) => {
            let file_type = metadata.file_type();
            
            match filter_type {
                FilterType::File => file_type.is_file(),
                FilterType::Directory => file_type.is_dir(),
                FilterType::Symlink => file_type.is_symlink(),
                FilterType::BlockDevice => {
                    #[cfg(unix)]
                    {
                        use std::os::unix::fs::FileTypeExt;
                        file_type.is_block_device()
                    }
                    #[cfg(not(unix))]
                    false
                }
                FilterType::CharDevice => {
                    #[cfg(unix)]
                    {
                        use std::os::unix::fs::FileTypeExt;
                        file_type.is_char_device()
                    }
                    #[cfg(not(unix))]
                    false
                }
                FilterType::Pipe => {
                    #[cfg(unix)]
                    {
                        use std::os::unix::fs::FileTypeExt;
                        file_type.is_fifo()
                    }
                    #[cfg(not(unix))]
                    false
                }
                FilterType::Socket => {
                    #[cfg(unix)]
                    {
                        use std::os::unix::fs::FileTypeExt;
                        file_type.is_socket()
                    }
                    #[cfg(not(unix))]
                    false
                }
            }
        }
        Err(_) => false,
    }
}