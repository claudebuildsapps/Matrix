use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers, MouseEvent};
use std::time::{Duration, Instant};
use anyhow::Result;

pub enum AppEvent {
    Key(KeyEvent),
    Mouse(MouseEvent),
    Tick,
    Quit,
}

pub struct EventHandler {
    tick_rate: Duration,
    last_tick: Instant,
}

impl EventHandler {
    pub fn new(tick_rate: Duration) -> Self {
        Self {
            tick_rate,
            last_tick: Instant::now(),
        }
    }

    pub fn next(&mut self) -> Result<AppEvent> {
        let timeout = self.tick_rate
            .checked_sub(self.last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
            
        if event::poll(timeout)? {
            match event::read()? {
                Event::Key(key) => {
                    if key.kind == event::KeyEventKind::Press {
                        // Ctrl+C or q to quit
                        if (key.code == KeyCode::Char('c') && key.modifiers == KeyModifiers::CONTROL) ||
                           (key.code == KeyCode::Char('q')) {
                            return Ok(AppEvent::Quit);
                        }
                        return Ok(AppEvent::Key(key));
                    }
                },
                Event::Mouse(mouse) => {
                    return Ok(AppEvent::Mouse(mouse));
                },
                _ => {}
            }
        }
        
        if self.last_tick.elapsed() >= self.tick_rate {
            self.last_tick = Instant::now();
            return Ok(AppEvent::Tick);
        }
        
        // No event, wait for next tick
        std::thread::sleep(timeout);
        Ok(AppEvent::Tick)
    }
}
