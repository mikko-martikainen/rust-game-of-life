use std::collections::HashMap;
use std::time::Instant;
use crossterm::event::KeyCode;
use crossterm::style::Color;
use crate::renderer::frame_builder::FrameBuilder;

#[derive(Debug, Clone)]
pub enum MenuCommand {
    Start,
    Quit,
}
#[derive(Debug, Clone)]
pub struct MenuOption {
    pub label: String,
    pub command: MenuCommand,
}

impl MenuOption {
    pub fn new(label: String, command: MenuCommand) -> Self {
        Self {
            label,
            command,
        }
    }
}

pub struct Menu {
    height: usize,
    options: Vec<MenuOption>,
    selected: usize,
}

impl Menu {
    pub fn with_options(height: usize, options: Vec<MenuOption>) -> Self {
        Self {
            height,
            options: options.clone(),
            selected: 0,
        }
    }

    pub fn handle_input(&mut self, inputs: &HashMap<KeyCode, Instant>) -> Option<MenuCommand> {
        if inputs.contains_key(&KeyCode::Up) {
            self.selected = self.selected.saturating_sub(1);
        }

        if inputs.contains_key(&KeyCode::Down) {
            self.selected = (self.selected + 1).min(self.options.len() - 1);
        }

        if inputs.contains_key(&KeyCode::Enter) {
            return Some(self.options[self.selected].command.clone());
        }

        if inputs.contains_key(&KeyCode::Esc) {
            return Some(MenuCommand::Quit);
        }

        None
    }

    pub fn draw(&self, frame: &mut FrameBuilder, x: usize, y: usize) {
        let y = y.saturating_sub(self.height / 2);
        let padding = (self.height - self.options.len()).max(1);

        for (i, option) in self.options.iter().enumerate() {
            let option_y = y + (i * padding);
            let color = if i == self.selected {
                Color::Blue
            } else {
                Color::White
            };

            frame.world().draw_text(
                &option.label, color, x, option_y, true
            );
        }
    }
}