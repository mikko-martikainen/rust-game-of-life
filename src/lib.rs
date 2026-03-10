mod engine;
mod renderer;
mod simulation;
mod ui;

use crossterm::{
    terminal,
};

use crate::{engine::engine::{Engine, EngineParams}, simulation::simulation::{Simulation, SimulationParams}};

pub fn run(fps: u64, tick_duration: u64) {
    let terminal = Terminal::init();

    let mut engine= Engine::init(
        EngineParams::new(terminal.width, terminal.height, fps, tick_duration)
    );
    
    let world_dimensions = engine.get_world_dimension();
    
    let mut simulation = Simulation::init(
        SimulationParams::new(world_dimensions.0, world_dimensions.1)
    );

    if let Err(_err) = engine.run(|inputs, frame| {
        simulation.tick(inputs, frame)
    }) {
        engine.cleanup();
    }
}

struct Terminal {
    width: usize,
    height: usize,
}

impl Terminal {
    pub fn init() -> Terminal {
        let (width, height) = terminal::size()
            .expect("Error during terminal initialization");

        Terminal { 
            width: width.into(), 
            height: height.into() 
        }
    }
}