pub mod other;
pub use other::{Cell, CellState, GameState};

pub mod visuals;
pub use visuals::{clear_terminal, reveal_cell, show_board};

pub mod generation;
pub use generation::{generate_board, generate_mines};

pub mod controls_mod;
pub use controls_mod::controls;

pub mod ui;
pub use ui::{init_game, next_round};

pub mod mechanics;
pub use mechanics::game_loop;
