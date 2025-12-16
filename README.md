# Game of Life Demo

A simple Rust implementation of **Conway's Game of Life**, demonstrating a 2D grid-based simulation of cells evolving according to the classic rules. This demo uses a fixed-size grid and displays the simulation in the terminal using the `ruscii` crate.


## Requirements
- Rust (installed via [rustup](https://rustup.rs))
- Cargo (comes with Rust)
- **Linux only:** X11 development libraries for compiling `ruscii`

### Installing X11 Libraries on Linux

When using Linux, make sure you have the X11 development libraries installed:

- **Ubuntu / Debian:**

```bash
sudo apt install libx11-dev
```

- **Fedora / RHEL / CentOS:**

```bash
sudo dnf install xorg-x11-server-devel
```

## How to Run

Build and run the project:

```bash
cargo run
```