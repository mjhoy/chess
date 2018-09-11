extern crate nalgebra as na;

use std::io;
use na::{U2, Dynamic, MatrixArray, MatrixVec, Matrix, RowVector2};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Player {
    White,
    Black,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Piece {
    Pawn,
}

type Square = Option<(Player, Piece)>;

// start with a 2x2 board
type Board = Matrix<Square, U2, U2, MatrixArray<Square, U2, U2>>;

#[derive(Debug)]
struct Game {
    board: Board,
}

fn new_game() -> Game {
    Game {
        board: Board::from_rows(&[
            RowVector2::new(Some((Player::White, Piece::Pawn)), Some((Player::White,Piece::Pawn))),
            RowVector2::new(Some((Player::Black, Piece::Pawn)), Some((Player::Black,Piece::Pawn))),
        ])
    }
}

fn print_board(game: &Game) -> () {
    fn print_piece(square: &Square) -> () {
        let piece_str = match square {
            None => " ",
            Some((Player::White,Piece::Pawn)) => "♙",
            Some((Player::Black,Piece::Pawn)) => "♟",
        };
        print!("{}",piece_str);
    }

    for rowi in (0..game.board.nrows()).rev() {
        let row = &game.board.row(rowi);
        for piece in row.iter() {
            print_piece(piece)
        }
        print!("\n");
    }
}

fn main() {
    let game = new_game();

    // ♟♟
    // ♙♙
    print_board(&game);

    // panics at runtime
    // println!("{:?}", game.board[(2,2)]);
}
