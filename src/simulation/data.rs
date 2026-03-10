use std::time::{Instant};
use rand::random_range;
use crate::ui::menu::{Menu, MenuCommand, MenuOption};

pub enum State {
    MainMenu,
    Running
}

pub struct Data {
    pub state: State,
    pub world_width: usize,
    pub world_height: usize,
    pub world_center_x: usize,
    pub world_center_y: usize,

    pub timer: Option<Instant>,
    pub iteration: usize,
    pub cells: Vec<bool>,
    pub cells_alive: usize,

    // UI
    pub main_menu: Menu,
}

impl Data {
    pub fn new(world_width: usize, world_height: usize) -> Data {
        let initial_cell_count = (world_width * world_height) / 3;

        let cells = generate_initial_cells(initial_cell_count, world_width, world_height);
        let cells_alive = cells.len();

        Data {
            state: State::MainMenu,
            world_width,
            world_height,
            world_center_x: world_width / 2,
            world_center_y: world_height / 2,
            timer: None,
            iteration: 0,
            cells,
            cells_alive,
            main_menu: create_main_menu(),
        }
    }
}

fn generate_initial_cells(quantity: usize, world_width: usize, world_height: usize) -> Vec<bool> {
    let mut cells: Vec<bool> = vec![false; world_height * world_width];

    for _ in 0..quantity {
        let x :usize = random_range(0..world_width);
        let y :usize = random_range(0..world_height);

        let index = (y * world_width) + x;

        cells[index] = true;
    }

    cells
}

fn create_main_menu() -> Menu {
    Menu::with_options(
        2,
        vec![
            MenuOption::new("Start".into(), MenuCommand::Start),
            MenuOption::new("Quit".into(), MenuCommand::Quit),
        ],
    )
}
