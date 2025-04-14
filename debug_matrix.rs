// Debug version of the Matrix Terminal application
// This helps diagnose issues with the application startup

use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::time::SystemTime;

fn main() {
    let log_file = PathBuf::from("/Users/joshkornreich/Documents/Projects/Terminal/Matrix/debug_log.txt");
    
    let mut file = match File::create(&log_file) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to create log file: {}", e);
            return;
        }
    };

    // Log basic information
    let _ = writeln!(file, "Matrix Terminal Debug Log");
    let _ = writeln!(file, "========================");
    let timestamp = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default().as_secs();
    let _ = writeln!(file, "Timestamp: {}", timestamp);
    
    // Log environment variables
    let _ = writeln!(file, "\nEnvironment Variables:");
    for (key, value) in std::env::vars() {
        let _ = writeln!(file, "  {} = {}", key, value);
    }
    
    // Log current directory
    let _ = writeln!(file, "\nCurrent Directory:");
    if let Ok(dir) = std::env::current_dir() {
        let _ = writeln!(file, "  {}", dir.display());
    } else {
        let _ = writeln!(file, "  Unable to determine current directory");
    }
    
    // Log terminal size
    let _ = writeln!(file, "\nTerminal Size:");
    if let Some(size) = term_size::dimensions() {
        let _ = writeln!(file, "  Width: {}, Height: {}", size.0, size.1);
    } else {
        let _ = writeln!(file, "  Unable to determine terminal size");
    }
    
    // Try to run the actual Matrix Terminal
    let _ = writeln!(file, "\nAttempting to start Matrix Terminal...");
    
    // Print to stderr so it shows up in the terminal
    eprintln!("Matrix Terminal Debug Mode");
    eprintln!("========================");
    eprintln!("Debug log written to: {}", log_file.display());
    eprintln!("Press Enter to continue...");
    
    // Wait for user input
    let mut buffer = String::new();
    let _ = std::io::stdin().read_line(&mut buffer);
    
    // Now try to run the real Matrix terminal
    let _ = writeln!(file, "Starting Matrix Terminal application...");
    
    match std::process::Command::new("./target/release/Matrix")
        .current_dir("/Users/joshkornreich/Documents/Projects/Terminal/Matrix")
        .status() {
        Ok(status) => {
            let _ = writeln!(file, "Matrix Terminal exited with status: {}", status);
        }
        Err(e) => {
            let _ = writeln!(file, "Failed to start Matrix Terminal: {}", e);
        }
    }
    
    let _ = writeln!(file, "Debug session complete");
}