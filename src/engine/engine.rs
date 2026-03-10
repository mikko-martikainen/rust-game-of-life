use std::collections::{HashMap};
use std::error::Error;
use std::time::{Instant};

use crossterm::{
    event::{KeyCode},
};
use crate::engine::engine_clock::EngineClock;
use crate::engine::inputs::InputHandler;
use crate::renderer::frame_builder::{FrameBuilder, FrameBuilderParams};
use crate::renderer::renderer::{Renderer, RendererParams};

pub struct EngineParams {
    pub terminal_width: usize,
    pub terminal_height: usize,
    pub fps: u64,
    pub tick_duration: u64,
}

impl EngineParams {
    pub fn new(
        terminal_width: usize,
        terminal_height: usize,
        fps: u64,
        tick_duration: u64,
    ) -> Self {
        Self {
            terminal_width,
            terminal_height,
            fps,
            tick_duration,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EngineSignal {
    Continue,
    Quit,
}

pub struct Engine {
    renderer: Renderer,
    clock: EngineClock,
    frame_builder: FrameBuilder,
    input_handler: InputHandler
}

impl Engine {
    pub fn init(params: EngineParams) -> Engine {
        let renderer = Renderer::init(RendererParams {
            terminal_width: params.terminal_width,
            terminal_height: params.terminal_height
        });

        let frame_builder = FrameBuilder::init(
            FrameBuilderParams::new(
                params.terminal_width,
                params.terminal_height,
            )
        );

        Engine {
            renderer,
            clock: EngineClock::new(params.fps, params.tick_duration),
            frame_builder,
            input_handler: InputHandler::new()
        }
    }

    pub fn get_world_dimension(&mut self) -> (usize, usize) {
        self.frame_builder.world().dimensions()
    }

    pub fn run<F>(&mut self, mut tick: F) -> Result<(), Box<dyn Error>>
    where
        F: FnMut(&HashMap<KeyCode, Instant>, &mut FrameBuilder) -> EngineSignal,
    {
        'engine_loop: loop {
            self.clock.update();

            // Handle inputs
            self.input_handler.poll()?;

            // ---- Fixed game tick updates ----
            while self.clock.should_tick() {
                let signal = tick(&self.input_handler.inputs(), &mut self.frame_builder);

                self.clock.consume_tick();

                if signal == EngineSignal::Quit {
                    break 'engine_loop;
                }
            }

            // Render fps etc.
            self.clock.display_fps(&mut self.frame_builder);

            // Render changes in frame builder
            self.handle_rendering();
        }

        // Restore terminal when closing the game
        self.renderer.cleanup()?;

        Ok(())
    }
    
    pub fn handle_rendering(&mut self) {
        if self.clock.should_render() {
            // Render changes
            self.renderer.render(self.frame_builder.take_dirty());

            self.clock.add_render();
        } else {
            std::thread::sleep(self.clock.get_sleep_duration());
        }
    }

    pub fn cleanup(&mut self) {
        self.renderer.cleanup()
            .expect("Error during cleanup");
    }
}
