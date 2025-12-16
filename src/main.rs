use std::time::{Instant};
use rand::{thread_rng, Rng};
use ruscii::app::{App, State, Config};
use ruscii::drawing::Pencil;
use ruscii::gui::FPSCounter;
use ruscii::keyboard::{Key, KeyEvent};
use ruscii::spatial::{Vec2};
use ruscii::terminal::Window;

// Generator variables
const RANGE_X: usize = 200;
const RANGE_Y: usize = 50;
static INITIAL_CELL_QUANTITY: usize = 2000;

struct SimulationState {
    pub timer: Instant,
    pub cells: [[bool; RANGE_Y]; RANGE_X],
    pub cells_alive: usize,
    pub iteration: usize
}

impl SimulationState {
    pub fn init() -> SimulationState {
        let cells = generate_initial_cells(INITIAL_CELL_QUANTITY);

        SimulationState {
            timer: Instant::now(),
            cells,
            iteration: 0,
            cells_alive: 0
        }
    }

    pub fn update(&mut self) {
        self.iteration += 1;
        let mut new_active_cells = [[false; RANGE_Y]; RANGE_X];
        let mut count = 0;

        for x in 0..RANGE_X {
            for y in 0..RANGE_Y {
                let is_alive = self.cells[x][y];
                let neighbour_count = Self::get_neighbor_count(x, y, &self.cells);

                if (is_alive && neighbour_count == 2) || neighbour_count == 3 {
                    new_active_cells[x][y] = true;
                    count += 1;
                }
            }
        }

        self.cells = new_active_cells;
        self.cells_alive = count;
    }

    fn get_neighbor_count(x: usize, y: usize, cells: &[[bool; RANGE_Y]; RANGE_X]) -> usize {
        let mut count = 0;

        let x_min = x.saturating_sub(1);
        let x_max = (x + 1).min(RANGE_X - 1);
        let y_min = y.saturating_sub(1);
        let y_max = (y + 1).min(RANGE_Y - 1);

        for i in x_min..=x_max {
            for j in y_min..=y_max {
                // Skip self
                if i == x && j == y {
                    continue;
                }

                if cells[i][j] == true {
                    count = count + 1;
                }
            }
        }

        count
    }
}

fn main() {
    let mut app = App::config(Config { fps: 120 });
    let mut state = SimulationState::init();
    let mut fps_counter = FPSCounter::default();

    app.run(|app_state: &mut State, window: &mut Window| {
        for key_event in app_state.keyboard().last_key_events() {
            match key_event {
                KeyEvent::Pressed(Key::Esc) => app_state.stop(),
                _ => (),
            }
        }

        fps_counter.update();
        if app_state.step() % 60 == 0 {
            state.update();
        }

        let mut pencil = Pencil::new(window.canvas_mut());

        pencil.draw_text(&format!("Iteration: {}", state.iteration), Vec2::xy(RANGE_X, RANGE_Y + 1));
        pencil.draw_text(&format!("Cells alive: {:?}", state.cells_alive), Vec2::xy(RANGE_X, RANGE_Y + 2));
        pencil.draw_text(&format!("Elapsed time: {:?}", state.timer.elapsed()), Vec2::xy(RANGE_X, RANGE_Y + 3));
        pencil.draw_text(&format!("FPS: {}", fps_counter.count()), Vec2::xy(RANGE_X, RANGE_Y + 4));


        for x in 0..RANGE_X {
            for y in 0..RANGE_Y {
                if state.cells[x][y] {
                    pencil.draw_char('0', Vec2 { x: x as i32, y: y as i32 });
                }
            }
        }
    });
}

fn generate_initial_cells(quantity: usize) -> [[bool; RANGE_Y]; RANGE_X] {
    let mut blocks = [[false; RANGE_Y]; RANGE_X];

    for _ in 0..quantity {
        let x :usize = thread_rng().gen_range(0..RANGE_X);
        let y :usize = thread_rng().gen_range(0..RANGE_Y);

        blocks[x][y] = true;
    }

    blocks
}
