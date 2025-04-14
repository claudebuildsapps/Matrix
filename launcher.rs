use std::process::Command;
use std::io::Write;
use std::fs::File;

fn main() {
    // Set up logging first
    let log_path = "/Users/joshkornreich/Documents/Projects/Terminal/Matrix/launcher.log";
    
    if let Ok(mut file) = File::create(log_path) {
        let _ = write!(file, "Matrix Terminal Launcher starting at {:?}\n", 
                             std::time::SystemTime::now());
    }
    
    // Define the Matrix path
    let matrix_path = "/Users/joshkornreich/Documents/Projects/Terminal/Matrix";
    
    // Define the AppleScript to launch Matrix Terminal
    let script = format!(
        r#"
        tell application "Terminal"
            do script "cd '{}' && ./target/release/Matrix"
            set custom title of front window to "Matrix Terminal"
            activate
        end tell
        "#,
        matrix_path
    );
    
    // Log the script we're using
    if let Ok(mut file) = std::fs::OpenOptions::new().append(true).open(log_path) {
        let _ = write!(file, "Using AppleScript:\n{}\n", script);
    }
    
    // Execute the AppleScript
    match Command::new("osascript")
        .arg("-e")
        .arg(&script)
        .output() {
        Ok(output) => {
            // Log success
            if let Ok(mut file) = std::fs::OpenOptions::new().append(true).open(log_path) {
                let _ = write!(file, "Launch successful\nStdout: {}\nStderr: {}\n",
                                    String::from_utf8_lossy(&output.stdout),
                                    String::from_utf8_lossy(&output.stderr));
            }
        },
        Err(e) => {
            // Log failure
            if let Ok(mut file) = std::fs::OpenOptions::new().append(true).open(log_path) {
                let _ = write!(file, "Launch failed: {}\n", e);
            }
            
            // Try fallback direct method
            match Command::new("open")
                .arg("-a")
                .arg("Terminal")
                .arg(format!("{}/direct_launch.sh", matrix_path))
                .output() {
                Ok(_) => {
                    if let Ok(mut file) = std::fs::OpenOptions::new().append(true).open(log_path) {
                        let _ = write!(file, "Fallback launch successful\n");
                    }
                },
                Err(e) => {
                    if let Ok(mut file) = std::fs::OpenOptions::new().append(true).open(log_path) {
                        let _ = write!(file, "Fallback launch also failed: {}\n", e);
                    }
                }
            }
        }
    }
}