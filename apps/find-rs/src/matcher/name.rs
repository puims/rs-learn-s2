/// 匹配文件名，支持通配符 * 和 ?
pub fn match_name(file_name: &str, pattern: &str, case_insensitive: bool) -> bool {
    // 如果 pattern 包含通配符，使用通配符匹配
    if pattern.contains('*') || pattern.contains('?') {
        wildcard_match(file_name, pattern, case_insensitive)
    } else {
        // 否则使用简单的字符串匹配
        if case_insensitive {
            file_name.to_lowercase() == pattern.to_lowercase()
        } else {
            file_name == pattern
        }
    }
}

/// 简单的通配符匹配函数
fn wildcard_match(text: &str, pattern: &str, case_insensitive: bool) -> bool {
    let text = if case_insensitive {
        text.to_lowercase()
    } else {
        text.to_string()
    };
    
    let pattern = if case_insensitive {
        pattern.to_lowercase()
    } else {
        pattern.to_string()
    };
    
    // 简单的通配符匹配算法
    let mut text_index = 0;
    let mut pattern_index = 0;
    let mut star_index = -1;
    let mut text_temp_index = 0;
    
    let text_chars: Vec<char> = text.chars().collect();
    let pattern_chars: Vec<char> = pattern.chars().collect();
    
    while text_index < text_chars.len() {
        if pattern_index < pattern_chars.len() && 
           (pattern_chars[pattern_index] == '?' || 
            pattern_chars[pattern_index] == text_chars[text_index]) {
            text_index += 1;
            pattern_index += 1;
        } else if pattern_index < pattern_chars.len() && pattern_chars[pattern_index] == '*' {
            star_index = pattern_index as isize;
            text_temp_index = text_index;
            pattern_index += 1;
        } else if star_index != -1 {
            pattern_index = (star_index + 1) as usize;
            text_temp_index += 1;
            text_index = text_temp_index;
        } else {
            return false;
        }
    }
    
    // 处理剩余的 '*' 字符
    while pattern_index < pattern_chars.len() && pattern_chars[pattern_index] == '*' {
        pattern_index += 1;
    }
    
    pattern_index == pattern_chars.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exact_match() {
        assert!(match_name("main.rs", "main.rs", false));
        assert!(!match_name("main.rs", "main.py", false));
    }
    
    #[test]
    fn test_case_insensitive() {
        assert!(match_name("MAIN.RS", "main.rs", true));
        assert!(!match_name("MAIN.RS", "main.rs", false));
    }
    
    #[test]
    fn test_wildcard_star() {
        assert!(match_name("main.rs", "*.rs", false));
        assert!(match_name("test.txt", "*.txt", false));
        assert!(!match_name("test.rs", "*.txt", false));
        assert!(match_name("dx_1", "dx_*", false));
        assert!(match_name("dx_2", "dx_*", false));
        assert!(!match_name("tx_1", "dx_*", false));
    }
    
    #[test]
    fn test_wildcard_question() {
        assert!(match_name("a.txt", "?.txt", false));
        assert!(!match_name("ab.txt", "?.txt", false));
        assert!(match_name("ab.txt", "??.txt", false));
    }
    
    #[test]
    fn test_mixed_wildcards() {
        assert!(match_name("abc123.txt", "abc*.txt", false));
        assert!(match_name("abc123.txt", "abc???.txt", false));
        assert!(!match_name("abc12.txt", "abc???.txt", false));
    }
}