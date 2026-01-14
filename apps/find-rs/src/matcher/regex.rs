use regex::Regex;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;

// 线程安全的正则表达式缓存
static REGEX_CACHE: Lazy<Mutex<HashMap<String, Regex>>> = Lazy::new(|| {
    Mutex::new(HashMap::new())
});

/// 正则表达式匹配（用于 -regex 选项）
pub fn regex_match(text: &str, pattern: &str, case_insensitive: bool) -> bool {
    let cache_key = if case_insensitive {
        format!("(?i){}", pattern)
    } else {
        pattern.to_string()
    };
    
    let mut cache = REGEX_CACHE.lock().unwrap();
    
    let regex = cache.entry(cache_key.clone()).or_insert_with(|| {
        match Regex::new(&cache_key) {
            Ok(re) => re,
            Err(e) => {
                eprintln!("Warning: Failed to compile regex '{}': {}", cache_key, e);
                // 回退到匹配所有
                Regex::new(".*").unwrap()
            }
        }
    });
    
    regex.is_match(text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regex_match() {
        // 基本正则表达式匹配
        assert!(regex_match("test123", r"\w+\d+", false));
        assert!(regex_match("test.txt", r".*\.txt", false));
        assert!(!regex_match("test.py", r".*\.txt", false));
        
        // 大小写不敏感
        assert!(regex_match("TEST.RS", r".*\.rs", true));
        assert!(!regex_match("TEST.RS", r".*\.rs", false));
        
        // 字符类
        assert!(regex_match("abc123", r"[a-z]+\d+", false));
        assert!(!regex_match("ABC123", r"[a-z]+\d+", false));
    }
}