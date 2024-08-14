//! Conway's Game of Life library.
//!
//! This library provides the implementation of Conway's Game of Life, a cellular automaton devised by the mathematician John Conway.
//! The game consists of a grid of cells that can live, die, or multiply based on specific rules.
//! This library includes the core game logic, the rendering of the game board, and the management of user interactions.

/// The `game` module contains the core functionality for the Game of Life.
/// It includes the definition of the game structure and methods for updating the game state.
/// This module represents the model part of the application.
pub mod game;

/// The `board_renderer` module is responsible for rendering the game board and handling user interactions.
/// It includes methods for drawing the grid, cells, and UI elements, as well as processing user input.
/// This module represents both the view and controller parts of the application.
pub mod board_renderer;
