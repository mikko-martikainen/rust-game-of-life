use std::{collections::{HashMap}, time::Instant};
use std::time::Duration;
use crossterm::{
    event::{KeyCode},
    style::Color
};
use crate::engine::engine::EngineSignal;
use crate::renderer::cell::{Cell};
use crate::renderer::frame_builder::FrameBuilder;
use crate::simulation::data::{Data, State};
use crate::ui::menu::MenuCommand;

static ITERATION_DURATION: Duration = Duration::from_millis(250);

pub struct SimulationParams {
    world_width: usize,
    world_height: usize
}

impl SimulationParams {
    pub fn new(world_width: usize, world_height: usize) -> SimulationParams {
        SimulationParams {
            world_width,
            world_height
        }
    }
}

pub struct Simulation {
    data: Data,
    last_iteration: Option<Instant>,
}

impl Simulation {
    pub fn init(params: SimulationParams) -> Simulation {
        let data = Data::new(
            params.world_width,
            params.world_height,
        );

        Simulation {
            data,
            last_iteration: None,
        }
    }

    pub fn tick(&mut self, inputs: &HashMap<KeyCode, Instant>, frame: &mut FrameBuilder) -> EngineSignal {
        match self.data.state {
            State::MainMenu => {
                if let Some (EngineSignal::Quit) =self.main_menu(inputs, frame) {
                    return EngineSignal::Quit
                }
            },
            State::Running => {
                self.running(inputs, frame);
            }
        }

        EngineSignal::Continue
    }

    fn main_menu(&mut self, inputs: &HashMap<KeyCode, Instant>, frame: &mut FrameBuilder) -> Option<EngineSignal> {
        frame.world().draw_text(
            "Press enter to start a game of life", Color::Green, self.data.world_center_x, self.data.world_center_y - 10, true
        );

        self.data.main_menu.draw(frame, self.data.world_center_x, self.data.world_center_y);

        if let Some(command) = self.data.main_menu.handle_input(inputs) {
            match command {
                MenuCommand::Start => {
                    self.start(frame);
                    None
                },
                MenuCommand::Quit => {
                    self.quit(frame);
                    Some(EngineSignal::Quit)
                },
            }
        } else {
            None
        }
    }

    fn start(&mut self, frame: &mut FrameBuilder) {
        let now = Instant::now();

        self.last_iteration = Some(now);
        self.data.timer = Some(now);

        frame.world().clear();
        self.data.state = State::Running;
    }

    fn quit(&mut self, frame: &mut FrameBuilder) {
        self.last_iteration = None;
        self.data.timer = None;

        frame.data().clear();
        self.data.state = State::MainMenu;
    }

    fn running(&mut self, inputs: &HashMap<KeyCode, Instant>, frame: &mut FrameBuilder) {
        for key_code in inputs.keys() {
            match key_code {
                KeyCode::Esc => {
                    self.data.state = State::MainMenu;
                    frame.world().clear();

                    return;
                },
                _ => {}
            }
        }

        if self.last_iteration.is_some_and(|last| Instant::now().duration_since(last) > ITERATION_DURATION) {
            self.iterate(frame);
        }

        frame.data().clear();
        frame.data().draw_text(
            format!("Iteration: {}", self.data.iteration), Color::Yellow,
            0, 0, false
        );

        frame.data().draw_text(
            format!("Cells alive: {}", self.data.cells_alive), Color::Yellow,
            0, 1, false
        );

        if let Some(timer) = self.data.timer {
            frame.data().draw_text(
                format!("Elapsed time: {}s", timer.elapsed().as_secs()), Color::Yellow,
                0, 2, false
            );
        }
    }

    fn iterate(&mut self, frame: &mut FrameBuilder) {
        self.data.iteration += 1;

        let mut new_cells: Vec<bool> = vec![false; self.data.world_width * self.data.world_height];
        let mut count = 0;

        for x in 0..self.data.world_width {
            for y in 0..self.data.world_height {
                let index = index(x, y , self.data.world_width);

                let is_alive = self.data.cells[index];
                let neighbour_count = self.get_neighbor_count(x, y);

                if neighbour_count == 3 || (is_alive && neighbour_count == 2) {
                    new_cells[index] = true;
                    count = count + 1;
                    frame
                        .world()
                        .draw(x, y, Cell::new('0', Color::Green))
                } else {
                    frame
                        .world()
                        .draw(x, y, Cell::new(' ', Color::Reset))
                }
            }
        }

        self.data.cells = new_cells;
        self.data.cells_alive = count;
        self.last_iteration = Some(Instant::now());
    }

    fn get_neighbor_count(&self, x: usize, y: usize) -> usize {
        let mut alive_neighbour_count = 0;

        for i in x.saturating_sub(1)..=(x + 1).min(self.data.world_width - 1) {
            for j in y.saturating_sub(1)..=(y + 1).min(self.data.world_height - 1) {
                if i == x && j == y {
                    continue;
                }

                let index = index(i, j, self.data.world_width);

                if self.data.cells[index] == true {
                    alive_neighbour_count = alive_neighbour_count + 1;
                }
            }
        }

        alive_neighbour_count
    }
}

#[inline]
fn index(x: usize, y: usize, row_size: usize) -> usize {
    y * row_size + x
}
