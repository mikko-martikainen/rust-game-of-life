use std::error::Error;
use std::io::{Stdout, stdout, Write};

use crossterm::{
    cursor,
    execute, queue,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{self},
};
use crate::renderer::cell::{Cell, DirtyCells};

pub struct RendererParams {
    pub terminal_width: usize,
    pub terminal_height: usize,
}

pub struct Renderer {
    stdout: Stdout,
    terminal_width: usize,
    terminal_height: usize,
    buffer: Vec<Cell>,
}

impl Renderer {
    pub fn init(params: RendererParams) -> Renderer {
        let mut stdout = stdout();

        // Setup terminal
        terminal::enable_raw_mode()
            .expect("Error during terminal initialization");
        execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide)
            .expect("Error during terminal initialization");

        Renderer {
            stdout,
            terminal_width: params.terminal_width,
            terminal_height: params.terminal_height,
            buffer: vec![Cell::new(' ', Color::Reset ); params.terminal_width * params.terminal_height],
        }
    }

    pub fn render(&mut self, dirty_cells: DirtyCells) {
        for (pos, cell) in dirty_cells {
            if pos.x > self.terminal_width || pos.y > self.terminal_height {
                continue;
            }

            let index = index(pos.x, pos.y, self.terminal_width);

            if self.buffer[index] != cell {
                queue!(
                    self.stdout,
                    cursor::MoveTo(pos.x as u16, pos.y as u16),
                    SetForegroundColor(cell.color),
                    Print(cell.content),
                    ResetColor
                ).expect("queue failed");

                self.buffer[index] = cell;
            }
        }

        self.stdout.flush()
            .expect("flush failed");
    }

    pub fn cleanup(&mut self) -> Result<(), Box<dyn Error>> {
        execute!(self.stdout, cursor::Show, terminal::LeaveAlternateScreen)?;
        terminal::disable_raw_mode()?;
        
        Ok(())
    }
}

#[inline]
fn index(x: usize, y: usize, row_size: usize) -> usize {
    y * row_size + x
}