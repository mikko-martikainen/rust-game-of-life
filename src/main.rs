// Constants
const FPS: u64 = 100;
const TICK_DURATION: u64 = 100; // ms

fn main() {
    rust_game_of_life::run(FPS, TICK_DURATION);
}
