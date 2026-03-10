use crossterm::style::Color;
use crate::renderer::cell::{Cell, CellPos, DirtyCells};

#[derive(Copy, Clone)]
pub struct Region {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

impl Region {
    #[inline]
    pub fn contains(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }

    #[inline]
    pub fn to_cell_pos(&self, x: usize, y: usize) -> CellPos {
        CellPos::new(self.x + x, self.y + y)
    }
}

pub struct RegionBuilder<'a> {
    region: Region,
    dirty: &'a mut DirtyCells,
}

impl<'a> RegionBuilder<'a> {
    pub fn new(region: Region, dirty: &'a mut DirtyCells) -> Self {
        Self {
            region,
            dirty
        }
    }

    pub fn dimensions(&self) -> (usize, usize) {
        (self.region.width, self.region.height)
    }
    
    #[inline]
    pub fn draw(&mut self, x: usize, y: usize, cell: Cell) {
        if !self.region.contains(x, y) {
            return;
        }

        self.dirty.insert(
            self.region.to_cell_pos(x, y),
            cell
        );
    }

    pub fn draw_text<S: Into<String>>(
        &mut self,
        text: S,
        color: Color,
        x: usize,
        y: usize,
        centered: bool,
    ) {
        let text = text.into();

        let start_x = if centered {
            x.saturating_sub(text.len() / 2)
        } else {
            x
        };

        for (i, ch) in text.chars().enumerate() {
            self.dirty.insert(
                self.region.to_cell_pos(start_x + i, y),
                Cell::new(ch, color),
            );
        }
    }

    pub fn clear(&mut self) {
        for y in self.region.y..self.region.y + self.region.height {
            for x in self.region.x..self.region.x + self.region.width {
                self.dirty.insert(
                    CellPos::new(x, y),
                    Cell::new(' ', Color::Reset)
                );
            }
        }
    }
}