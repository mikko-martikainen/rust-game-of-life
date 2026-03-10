# Game of Life Demo

A Simple Rust implementation of **Conway's Game of Life**, demonstrating a 2D grid-based simulation of cells evolving according to the classic rules. 
Build using `crossterm`-crate for terminal ui rendering and an over-engineered (for this purpose) custom game/simulation engine.

## Engine Design

The simulation is built on a custom terminal-based game engine, it separates responsibilities to the following components:

- **Engine:** Manages the main loop, including fixed-step simulation ticks, frame timing, and input handling.
- **Renderer:** Responsible for drawing the output buffer to the terminal. Maintains a buffered representation of the screen and only updates the cells that have changed (dirty rendering).
- **Frame/Region builder:** Provides a higher-level abstraction for building frames. Supports regions for independent areas (e.g., world, debug overlay). Drawing operations only affect their assigned area, preventing overlaps and artifacts. Text and tiles are converted into dirty cells which are then flushed to the renderer.
- **Simulation / Data:** Keeps the simulation state entirely separate from rendering. The simulation modifies its internal state and marks only the cells that changed as “dirty.”

## Requirements
- Rust (installed via [rustup](https://rustup.rs))
- Cargo (comes with Rust)

## How to Run

Build and run the project:

```bash
cargo run
```