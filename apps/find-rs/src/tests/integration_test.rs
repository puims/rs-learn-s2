#[cfg(test)]
mod tests {
    use std::fs::{self, File};
    use std::io::Write;
    use tempfile::TempDir;
    use find_rs::execute;

    #[test]
    fn test_find_rs_files() {
        let temp_dir = TempDir::new().unwrap();
        
        // 创建测试文件
        let test_files = vec![
            "main.rs",
            "lib.rs",
            "test.txt",
            ".hidden.rs",
            "src/main.rs",
            "src/lib.rs",
        ];
        
        for file in test_files {
            let path = temp_dir.path().join(file);
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent).unwrap();
            }
            let mut f = File::create(&path).unwrap();
            writeln!(f, "test content").unwrap();
        }
        
        // 这里可以添加更多测试逻辑
        // 注意：由于 execute() 函数使用 clap 解析命令行参数，
        // 我们需要模拟命令行参数或重构代码以支持直接调用
    }
}