
# Dots and Boxes

Dots and Boxes is a digital implementation of the classic pen-and-paper game. This project is built using the Bevy engine and features interactive gameplay where two players can take turns to draw lines and earn points by completing boxes.

## Table of Contents

- [Installation](https://chat.openai.com/g/g-wpMtgVmzG-readme-generator/c/580ec197-cf3f-40a0-806f-e56fa17d32ac#installation)
- [Usage](https://chat.openai.com/g/g-wpMtgVmzG-readme-generator/c/580ec197-cf3f-40a0-806f-e56fa17d32ac#usage)
- [Features](https://chat.openai.com/g/g-wpMtgVmzG-readme-generator/c/580ec197-cf3f-40a0-806f-e56fa17d32ac#features)
- [Dependencies](https://chat.openai.com/g/g-wpMtgVmzG-readme-generator/c/580ec197-cf3f-40a0-806f-e56fa17d32ac#dependencies)
- [Configuration](https://chat.openai.com/g/g-wpMtgVmzG-readme-generator/c/580ec197-cf3f-40a0-806f-e56fa17d32ac#configuration)
- [Running Tests](https://chat.openai.com/g/g-wpMtgVmzG-readme-generator/c/580ec197-cf3f-40a0-806f-e56fa17d32ac#running-tests)
- [Contributors](https://chat.openai.com/g/g-wpMtgVmzG-readme-generator/c/580ec197-cf3f-40a0-806f-e56fa17d32ac#contributors)
- [License](https://chat.openai.com/g/g-wpMtgVmzG-readme-generator/c/580ec197-cf3f-40a0-806f-e56fa17d32ac#license)

## Installation

To set up this project locally, you will need to have Rust and Cargo installed on your machine.

1. Clone the repository:
`git clone https://github.com/yourusername/dots-and-boxes.git`
`cd dots-and-boxes`

2. Build the project:
`cargo build`

## Usage

To run the game, use the following command after installation:

`cargo run`

The game window should open, and you can start playing by clicking on the grid lines to form boxes.

## Features

- Interactive grid-based gameplay.
- Turn-based mechanics for two players.
- Score tracking and display.
- Customizable grid size and line colors.

## Dependencies

This project relies on the following Rust crates:

- `bevy`: A modern game engine built in Rust.
- `bevy_mod_picking`: A plugin for mouse-based interaction within Bevy games.

Make sure these dependencies are correctly listed in your `Cargo.toml` file.

## Configuration

You can adjust the game settings such as grid size and line width by modifying the constants at the beginning of the main source file:

``const GRID_SIZE: f32 = 6.0;
const WIDTH: f32 = 10.0;``

## Running Tests

This project includes tests for game functionality and utilities. Run the tests using:

`cargo test`

## Contributors

julianjjo

## License

This project is licensed under the MIT License - see the [LICENSE](https://github.com/julianjjo/dots-and-boxes-rust/blob/main/LICENSE) file for details.
