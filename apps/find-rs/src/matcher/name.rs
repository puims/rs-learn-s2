pub fn match_name(file_name: &str, pattern: &str, case_insensitive: bool) -> bool {
    if case_insensitive {
        file_name.to_lowercase().contains(&pattern.to_lowercase())
    } else {
        file_name.contains(pattern)
    }
}