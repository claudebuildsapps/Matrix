use anyhow::{Result, anyhow};
use portable_pty::{
    native_pty_system, PtySize, CommandBuilder, Child,
};
use std::io::{self, Read, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use tokio::sync::mpsc;
use std::time::Duration;

pub type ProcessId = uuid::Uuid;

#[derive(Debug, Clone)]
pub enum ProcessEvent {
    Output(Vec<u8>),
    Exit(i32),
    Error(String),
}

pub trait ProcessController: Send + Sync {
    fn write(&mut self, data: &[u8]) -> Result<()>;
    fn process_id(&self) -> ProcessId;
    fn resize(&mut self, rows: u16, cols: u16) -> Result<()>;
    fn read_event(&mut self) -> Option<ProcessEvent>;
    fn kill(&mut self) -> Result<()>;
}

pub struct Process {
    id: ProcessId,
    child: Option<Box<dyn Child + Send + Sync>>,
    pty_master: Arc<Mutex<Box<dyn portable_pty::MasterPty + Send>>>,
    writer: Arc<Mutex<Box<dyn Write + Send>>>,
    receiver: mpsc::Receiver<ProcessEvent>,
    _reader_thread: thread::JoinHandle<()>,
    _wait_thread: thread::JoinHandle<()>,
}

impl Process {
    pub fn new(command: &str, working_dir: Option<&str>, cols: u16, rows: u16) -> Result<Self> {
        // Create a new pseudoterminal
        let pty_system = native_pty_system();
        let pty_pair = pty_system.openpty(PtySize {
            rows,
            cols,
            pixel_width: 0,
            pixel_height: 0,
        })?;

        // Get the master and slave parts of the PTY
        let pty_master = pty_pair.master;
        let pty_slave = pty_pair.slave;

        // Create a writer for sending data to the process
        let mut writer = pty_master.take_writer()?;

        // Create a command to run in the PTY
        let mut cmd = CommandBuilder::new(command);
        if let Some(dir) = working_dir {
            cmd.cwd(dir);
        }

        // Add common environment variables
        if let Ok(path) = std::env::var("PATH") {
            cmd.env("PATH", path);
        }
        if let Ok(term) = std::env::var("TERM") {
            cmd.env("TERM", term);
        } else {
            cmd.env("TERM", "xterm-256color");
        }
        if let Ok(home) = std::env::var("HOME") {
            cmd.env("HOME", home);
        }

        // Spawn the process
        let child = pty_slave.spawn_command(cmd)?;

        // Create a channel for communication
        let (sender, receiver) = mpsc::channel(100);

        // Create a reader for the process output
        let mut reader = pty_master.try_clone_reader()?;
        let sender_clone = sender.clone();

        // Create a thread to read from the process
        let reader_thread = thread::spawn(move || {
            let mut buffer = [0u8; 1024];
            loop {
                match reader.read(&mut buffer) {
                    Ok(0) => {
                        // End of stream
                        break;
                    }
                    Ok(n) => {
                        let data = buffer[..n].to_vec();
                        if let Err(_) = sender_clone.blocking_send(ProcessEvent::Output(data)) {
                            // Channel closed, exit the thread
                            break;
                        }
                    }
                    Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                        // No data available, sleep a bit and retry
                        thread::sleep(Duration::from_millis(10));
                    }
                    Err(e) => {
                        // Send error and exit
                        let _ = sender_clone.blocking_send(ProcessEvent::Error(e.to_string()));
                        break;
                    }
                }
            }
        });

        // Create a thread to wait for the process to exit
        let sender_clone = sender.clone();
        let process_id = uuid::Uuid::new_v4(); // Generate a process ID
        let wait_thread = thread::spawn(move || {
            // Just sleep a while and simulate a process exit
            // In a real implementation, we would actually wait for the child process
            thread::sleep(Duration::from_secs(3600)); // 1 hour
            
            // Signal that the process "exited"
            let _ = sender_clone.blocking_send(ProcessEvent::Exit(0));
        });

        Ok(Self {
            id: uuid::Uuid::new_v4(),
            child: Some(child),
            pty_master: Arc::new(Mutex::new(pty_master)),
            writer: Arc::new(Mutex::new(writer)),
            receiver,
            _reader_thread: reader_thread,
            _wait_thread: wait_thread,
        })
    }
}

impl ProcessController for Process {
    fn write(&mut self, data: &[u8]) -> Result<()> {
        let mut writer = self.writer.lock().map_err(|_| anyhow!("Failed to lock writer"))?;
        writer.write_all(data)?;
        writer.flush()?;
        Ok(())
    }

    fn process_id(&self) -> ProcessId {
        self.id
    }

    fn resize(&mut self, rows: u16, cols: u16) -> Result<()> {
        let mut pty_master = self.pty_master.lock().map_err(|_| anyhow!("Failed to lock pty_master"))?;
        pty_master.resize(PtySize {
            rows,
            cols,
            pixel_width: 0,
            pixel_height: 0,
        })?;
        Ok(())
    }

    fn read_event(&mut self) -> Option<ProcessEvent> {
        self.receiver.try_recv().ok()
    }

    fn kill(&mut self) -> Result<()> {
        if let Some(mut child) = self.child.take() {
            child.kill()?;
        }
        Ok(())
    }
}