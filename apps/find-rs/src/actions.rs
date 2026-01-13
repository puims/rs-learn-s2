use std::path::PathBuf;
use crate::types::{Cli, FindResult};

pub mod delete;
pub mod exec;
pub mod print;

pub fn do_action(matches: &[PathBuf], cli: &Cli) -> FindResult<()> {
    // 检查是否指定了任何操作
    let has_action = cli.actions.print.is_some() || 
                     cli.actions.exec.is_some() || 
                     cli.actions.delete;
    
    if !has_action {
        // 如果没有指定动作，默认打印路径
        for path in matches {
            print::print_file(path, "%p")?;
        }
        return Ok(());
    }
    
    // 执行指定操作
    for path in matches {
        if let Some(format) = &cli.actions.print {
            print::print_file(path, format)?;
        }
        
        if let Some(cmd) = &cli.actions.exec {
            exec::execute_command(cmd, path)?;
        }
        
        if cli.actions.delete {
            delete::delete_file(path, cli.actions.force)?;
        }
    }
    
    Ok(())
}