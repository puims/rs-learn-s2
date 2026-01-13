use std::path::Path;

use crate::types::FindResult;

pub fn execute_command(cmd_template: &str, path: &Path) -> FindResult<()> {
    let cmd = cmd_template.replace("{}", &path.display().to_string());
    
    #[cfg(unix)]
    {
        use std::process::Command;
        
        if let Err(e) = Command::new("sh")
            .arg("-c")
            .arg(&cmd)
            .spawn()
            .and_then(|mut child| child.wait()) {
            eprintln!("Failed to execute command: {}", e);
        }
    }
    
    #[cfg(windows)]
    {
        use std::process::Command;
        
        if let Err(e) = Command::new("cmd")
            .arg("/C")
            .arg(&cmd)
            .spawn()
            .and_then(|mut child| child.wait()) {
            eprintln!("Failed to execute command: {}", e);
        }
    }

    Ok(())
}