//! Conway's Game of Life library.
//!
//! This library provides the implementation of Conway's Game of Life, a cellular automaton devised by the mathematician John Conway.
//! The game consists of a grid of cells that can live, die, or multiply based on specific rules.
//! This library includes the core game logic and the rendering of the game board.

/// The `game` module contains the core functionality for the Game of Life.
/// It includes the definition of the game structure, methods for updating the game state,
/// and handling user input.
pub mod game;

/// The `board_renderer` module is responsible for rendering the game board and UI elements.
/// It includes methods for drawing the grid, cells, and menus, as well as displaying initial instructions.
pub mod board_renderer;
