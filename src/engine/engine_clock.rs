use std::time::{Duration, Instant};
use crossterm::style::Color;
use crate::renderer::frame_builder::FrameBuilder;

pub struct EngineClock {
    tick_duration: Duration,
    last_tick: Instant,
    accumulator: Duration,

    pub frame_count: u32,
    frame_duration: Duration,
    last_render: Instant,
    framerate_timer: Instant,
}

impl EngineClock {
    pub fn new(fps: u64, tick_duration: u64,) -> Self {
        let now = Instant::now();

        let tick_duration = Duration::from_millis(tick_duration);
        let frame_duration = Duration::from_millis(1000 / fps);

        Self {
            tick_duration,
            last_tick: now,
            accumulator: Duration::ZERO,
            last_render: now,
            frame_count: 0,
            framerate_timer: now,
            frame_duration,
        }
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        let frame_time = (now - self.last_tick).min(Duration::from_millis(250));
        self.last_tick = now;
        self.accumulator += frame_time;
    }

    pub fn should_tick(&self) -> bool {
        self.accumulator >= self.tick_duration
    }

    pub fn consume_tick(&mut self) {
        self.accumulator -= self.tick_duration;
    }

    pub fn reset_framerate_timer(&mut self) {
        self.framerate_timer = Instant::now();
        self.frame_count = 0;
    }

    pub fn should_render(&self) -> bool {
        let now = Instant::now();
        let time_since_last_frame = now.duration_since(self.last_render);

        time_since_last_frame >= self.frame_duration
    }

    pub fn get_sleep_duration(&self) -> Duration {
        let elapsed = Instant::now().duration_since(self.last_render);

        self.frame_duration.saturating_sub(elapsed)
    }

    pub fn add_render(&mut self) {
        self.frame_count += 1;
        self.last_render = Instant::now();
    }

    pub fn display_fps(&mut self, frame: &mut FrameBuilder) {
        if self.framerate_timer.elapsed() > Duration::from_secs(1) {
            frame.debug().draw_text(
                format!("FPS: {}", self.frame_count), Color::Grey, 0, 0, false
            );
            
            self.reset_framerate_timer();
        }
    }
}