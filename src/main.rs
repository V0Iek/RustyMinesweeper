use ::lib::{
    check_if_win, clear_terminal, controls, generate_board, reveal_cell, show_board, Cell,
    GameState,
};
use std::io::{self, Write};

fn main() {
    let mut choose = String::new();
    let mut board: Vec<Vec<Cell>>;
    let mines: i32;
    let mut mines_generated = false;

    let mut game_state = GameState::InProgress;
    let mut pos_x: usize = 0;
    let mut pos_y: usize = 0;

    clear_terminal();

    println!("Choose the difficulty level:");
    println!("1. 8x8 board, 10 mines");
    println!("2. 16x16 board, 40 mines");
    println!("3. 30x16 board, 99 mines");

    io::stdout().flush().expect("Failed to flush stdout");

    io::stdin()
        .read_line(&mut choose)
        .expect("Failed to read line");

    let difficulty = choose.trim().parse::<usize>();

    match difficulty {
        Ok(1) => {
            board = generate_board(8, 8);
            mines = 10;
        }
        Ok(2) => {
            board = generate_board(16, 16);
            mines = 40;
        }
        Ok(3) => {
            board = generate_board(30, 16);
            mines = 99;
        }
        _ => {
            println!("Invalid choice. Exiting.");
            return;
        }
    }

    show_board(board.clone(), pos_x, pos_y);

    while game_state == GameState::InProgress {
        controls(
            &mut board,
            &mut pos_x,
            &mut pos_y,
            &mut game_state,
            mines,
            &mut mines_generated,
        );

        if check_if_win(&mut board) {
            game_state = GameState::Won;
        }

        show_board(board.clone(), pos_x, pos_y);
    }

    if game_state == GameState::Won {
        println!("You won!");
    }
    if game_state == GameState::Lost {
        reveal_cell(&mut board, pos_x, pos_y);
        show_board(board, pos_x, pos_y);
        println!("Boom! You loose");
    }

    crossterm::terminal::disable_raw_mode().expect("Failed to disable raw mode");
}
