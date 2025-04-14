#!/usr/bin/swift

import Foundation

// Set up logging
let logPath = "/Users/joshkornreich/Documents/Projects/Terminal/Matrix/launcher.log"
let matrixPath = "/Users/joshkornreich/Documents/Projects/Terminal/Matrix"

// Log start
let dateFormatter = DateFormatter()
dateFormatter.dateFormat = "yyyy-MM-dd HH:mm:ss"
let dateString = dateFormatter.string(from: Date())

do {
    try "Matrix Terminal Launcher starting at \(dateString)\n".write(
        toFile: logPath, 
        atomically: true, 
        encoding: .utf8
    )
} catch {
    print("Error writing to log: \(error)")
}

// Define the AppleScript
let script = """
tell application "Terminal"
    do script "cd '\(matrixPath)' && ./target/release/Matrix"
    set custom title of front window to "Matrix Terminal"
    activate
end tell
"""

// Create a process to run osascript
let process = Process()
process.executableURL = URL(fileURLWithPath: "/usr/bin/osascript")
process.arguments = ["-e", script]

// Set up pipes for stdout and stderr
let outputPipe = Pipe()
let errorPipe = Pipe()
process.standardOutput = outputPipe
process.standardError = errorPipe

do {
    // Launch the process
    try process.run()
    
    // Get outputs
    let outputData = outputPipe.fileHandleForReading.readDataToEndOfFile()
    let errorData = errorPipe.fileHandleForReading.readDataToEndOfFile()
    let output = String(data: outputData, encoding: .utf8) ?? ""
    let error = String(data: errorData, encoding: .utf8) ?? ""
    
    // Wait for the process to complete
    process.waitUntilExit()
    
    // Log results
    let successStatus = process.terminationStatus == 0 ? "Success" : "Failed"
    let logMessage = """
    
    Launch \(successStatus) with status \(process.terminationStatus)
    Output: \(output)
    Error: \(error)
    
    """
    
    if let fileHandle = FileHandle(forWritingAtPath: logPath) {
        fileHandle.seekToEndOfFile()
        fileHandle.write(logMessage.data(using: .utf8)!)
        fileHandle.closeFile()
    }
    
    // If failed, try fallback method
    if process.terminationStatus != 0 {
        let fallbackProcess = Process()
        fallbackProcess.executableURL = URL(fileURLWithPath: "/usr/bin/open")
        fallbackProcess.arguments = ["-a", "Terminal", "\(matrixPath)/direct_launch.sh"]
        try fallbackProcess.run()
        
        if let fileHandle = FileHandle(forWritingAtPath: logPath) {
            fileHandle.seekToEndOfFile()
            fileHandle.write("\nFallback method attempted\n".data(using: .utf8)!)
            fileHandle.closeFile()
        }
    }
} catch {
    // Log any errors
    if let fileHandle = FileHandle(forWritingAtPath: logPath) {
        fileHandle.seekToEndOfFile()
        fileHandle.write("\nError launching: \(error)\n".data(using: .utf8)!)
        fileHandle.closeFile()
    }
}