use walkdir::WalkDir;
use crate::types::Cli;

pub fn get_walker(cli: &Cli) -> WalkDir {
    let mut walker = WalkDir::new(&cli.path);
    
    // 重要：设置 min_depth 为 0 以包含起始路径本身
    // 但要注意，如果起始路径是文件，min_depth(0) 会包含它
    // 如果起始路径是目录，min_depth(0) 会包含目录本身
    
    // 处理深度限制
    if let Some(depth) = cli.criteria.depth {
        // 如果指定了深度，设置最大深度
        // 注意：深度是相对于起始路径的
        // depth=0: 只包含起始路径本身
        // depth=1: 包含起始路径的直接子项
        walker = walker.max_depth(depth as usize);
    }
    
    // 注意：我们不设置 min_depth，默认就是 0
    
    if cli.criteria.follow_symlinks {
        walker = walker.follow_links(true);
    }
    
    // 添加内容排序，以便输出更可预测
    walker = walker.sort_by(|a, b| a.file_name().cmp(b.file_name()));

    walker
}