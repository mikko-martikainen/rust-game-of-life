use std::collections::HashMap;
use std::error::Error;
use std::time::{Duration, Instant};
use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};

static INPUT_HOLD_TIMEOUT: Duration = Duration::from_millis(100);

pub struct InputHandler {
    inputs: HashMap<KeyCode, Instant>,
}

impl InputHandler {
    pub fn new() -> InputHandler {
        InputHandler {
            inputs: HashMap::new(),
        }
    }

    pub fn inputs(&self) -> &HashMap<KeyCode, Instant> {
        &self.inputs
    }

    pub fn poll(&mut self) -> Result<(), Box<dyn Error>> {
        // Read inputs
        while event::poll(Duration::from_secs(0))? {
            if let Event::Key(KeyEvent { code, kind, .. }) = event::read()? {
                match kind {
                    KeyEventKind::Press => {
                        self.inputs.insert(code, Instant::now());
                    }
                    KeyEventKind::Repeat => {
                        self.inputs.insert(code, Instant::now());
                    }
                    _ => {}
                }
            }
        }

        self.inputs.retain(|_, last_seen| Instant::now().duration_since(*last_seen) < INPUT_HOLD_TIMEOUT);

        Ok(())
    }
}