use iced::keyboard::{self, KeyCode, Modifiers};

/// Convert iced key events to terminal input bytes
pub fn key_to_terminal_input(key: KeyCode, modifiers: Modifiers) -> Option<Vec<u8>> {
    match key {
        // Basic ASCII characters
        KeyCode::Char(c) => {
            let mut bytes = Vec::new();
            
            // Handle control characters
            if modifiers.control() {
                let control_char = match c {
                    'a'..='z' => (c as u8 - b'a' + 1),
                    'A'..='Z' => (c as u8 - b'A' + 1),
                    // Add more control character mappings as needed
                    _ => return None,
                };
                bytes.push(control_char);
            } else {
                // Regular character
                bytes.extend_from_slice(c.to_string().as_bytes());
            }
            
            Some(bytes)
        },
        
        // Special keys
        KeyCode::Enter => Some(vec![b'\r']),
        KeyCode::Tab => Some(vec![b'\t']),
        KeyCode::Backspace => Some(vec![0x7F]), // Delete character
        KeyCode::Escape => Some(vec![0x1B]),    // ESC
        
        // Function keys (F1-F12)
        KeyCode::F(num) => {
            // Convert function keys to their typical escape sequences
            // This is a simplified implementation
            let seq = match num {
                1 => b"\x1BOP".to_vec(),
                2 => b"\x1BOQ".to_vec(),
                3 => b"\x1BOR".to_vec(),
                4 => b"\x1BOS".to_vec(),
                5 => b"\x1B[15~".to_vec(),
                6 => b"\x1B[17~".to_vec(),
                7 => b"\x1B[18~".to_vec(),
                8 => b"\x1B[19~".to_vec(),
                9 => b"\x1B[20~".to_vec(),
                10 => b"\x1B[21~".to_vec(),
                11 => b"\x1B[23~".to_vec(),
                12 => b"\x1B[24~".to_vec(),
                _ => return None,
            };
            
            Some(seq)
        },
        
        // Arrow keys and navigation
        KeyCode::Up => Some(b"\x1B[A".to_vec()),
        KeyCode::Down => Some(b"\x1B[B".to_vec()),
        KeyCode::Right => Some(b"\x1B[C".to_vec()),
        KeyCode::Left => Some(b"\x1B[D".to_vec()),
        KeyCode::Home => Some(b"\x1B[H".to_vec()),
        KeyCode::End => Some(b"\x1B[F".to_vec()),
        KeyCode::PageUp => Some(b"\x1B[5~".to_vec()),
        KeyCode::PageDown => Some(b"\x1B[6~".to_vec()),
        KeyCode::Delete => Some(b"\x1B[3~".to_vec()),
        KeyCode::Insert => Some(b"\x1B[2~".to_vec()),
        
        // Unhandled keys
        _ => None,
    }
}