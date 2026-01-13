use regex::Regex;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;

// 线程安全的正则表达式缓存
static REGEX_CACHE: Lazy<Mutex<HashMap<(String, bool), Regex>>> = Lazy::new(|| {
    Mutex::new(HashMap::new())
});

pub fn simple_pattern_match(text: &str, pattern: &str, case_insensitive: bool) -> bool {
    let key = (pattern.to_string(), case_insensitive);
    
    let mut cache = REGEX_CACHE.lock().unwrap();
    
    let regex = cache.entry(key.clone()).or_insert_with(|| {
        // 将简单通配符模式转换为正则表达式
        let regex_pattern = convert_wildcard_to_regex(pattern);
        let pattern_with_anchors = format!("^{}$", regex_pattern);
        
        if case_insensitive {
            Regex::new(&format!("(?i){}", pattern_with_anchors))
                .unwrap_or_else(|_| Regex::new(".*").unwrap())
        } else {
            Regex::new(&pattern_with_anchors)
                .unwrap_or_else(|_| Regex::new(".*").unwrap())
        }
    });
    
    regex.is_match(text)
}

fn convert_wildcard_to_regex(pattern: &str) -> String {
    // 转义正则表达式特殊字符
    let escaped = regex::escape(pattern);
    
    // 将通配符 * 和 ? 转换回正则表达式形式
    let mut result = String::new();
    let mut chars = escaped.chars().peekable();
    
    while let Some(ch) = chars.next() {
        if ch == '\\' {
            if let Some(&next_ch) = chars.peek() {
                if next_ch == '*' {
                    result.push_str(".*");
                    chars.next(); // 跳过 *
                } else if next_ch == '?' {
                    result.push('.');
                    chars.next(); // 跳过 ?
                } else {
                    result.push(ch);
                    result.push(next_ch);
                    chars.next(); // 跳过转义字符
                }
            } else {
                result.push(ch);
            }
        } else {
            result.push(ch);
        }
    }
    
    result
}