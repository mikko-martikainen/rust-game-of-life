use std::collections::HashMap;
use crossterm::style::Color;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Cell {
    pub content: char,
    pub color: Color,
}

impl Cell {
    pub fn new(content: char, color: Color) -> Cell {
        Cell { content, color }
    }
}

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub struct CellPos {
    pub x: usize,
    pub y: usize,
}

impl CellPos {
    pub fn new(x: usize, y: usize) -> CellPos {
        CellPos { x, y }
    }
}
pub type DirtyCells = HashMap<CellPos, Cell>;

