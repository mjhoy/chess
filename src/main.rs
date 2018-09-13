extern crate nalgebra as na;
#[macro_use] extern crate itertools;

use na::{U2, MatrixArray, Matrix, RowVector2};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Player {
    White,
    Black,
}

use Player::*;

impl Player {
    fn other(&self) -> Player {
        match &self {
            White => Black,
            Black => White,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Piece {
    Pawn,
}

use Piece::*;

type Square = Option<(Player, Piece)>;

#[derive(Debug, Clone, Copy)]
struct Pos {
    rank: u8,
    file: u8
}

// start with a 2x2 board
type Board = Matrix<Square, U2, U2, MatrixArray<Square, U2, U2>>;

#[derive(Debug, Clone, Copy)]
struct GameState {
    board: Board,
    current_player: Player,
}

struct Game {
    state: GameState,
    moves: Vec<Move>,
}

struct Move {
    index: (Pos,Pos),
    next: Game
}

fn initial_board() -> Board {
    Board::from_rows(&[
        RowVector2::new(Some((White, Pawn)), Some((White,Pawn))),
        RowVector2::new(Some((Black, Pawn)), Some((Black,Pawn))),
    ])
}

fn coords(board: &Board) -> Vec<Pos> {
    iproduct!(0..board.ncols(), 0..board.nrows()).map(|(rank, file)| {
        Pos { rank: rank as u8, file: file as u8 }
    }).collect()
}

fn piece_at(pos: &Pos, board: &Board) -> Option<(Player,Piece)> {
    board.row(pos.file as usize)[pos.rank as usize]
}

fn new_game() -> Game {
    let board = initial_board();
    let current_player = White;
    let state = GameState { board, current_player, };
    Game {
        state: state,
        moves: gen_moves(state),
    }
}

fn can_move(board: &Board, player: &Player, from_pos: &Pos, to_pos: &Pos) -> bool {
    let from = piece_at(from_pos, board);
    let to   = piece_at(to_pos, board);

    match from {
        Some((from_player, piece)) if from_player == *player => {
            match to {
                Some((to_player, _)) if to_player == *player => false,
                _ => { match piece {
                    Pawn => {
                        // simplified pawn moves, just one ahead
                        if from_pos.file == to_pos.file {
                            let next_rank = if *player == White { 1 } else { -1 };
                            to_pos.file == ((from_pos.file as i32) + next_rank) as u8
                        } else {
                            false
                        }
                    }
                } }
            }
        }
        _ => false
    }
}

// moves from -> to
fn move_piece(board: &Board, from_pos: &Pos, to_pos: &Pos) -> Board {
    let new_board: &mut Board = &mut board.clone();
    let from = piece_at(from_pos, board);
    new_board[(from_pos.file as usize, from_pos.rank as usize)] = None;
    new_board[(to_pos.file as usize, to_pos.rank as usize)] = from;

    new_board.clone()
}

// O(xy^2)
fn gen_moves(state: GameState) -> Vec<Move> {
    coords(&state.board).iter().flat_map(|from_pos| {
        coords(&state.board).iter().filter_map(|to_pos| {
            if can_move(&state.board, &state.current_player, from_pos, to_pos) {
                Some(Move {
                    index: (*from_pos, *to_pos),
                    next: Game {
                        state: GameState {
                            board: move_piece(&state.board, from_pos, to_pos),
                            current_player: state.current_player.other(),
                        },
                        moves: vec![]
                    }
                })
            } else {
                None
            }
        }).collect::<Vec<Move>>()
    }).collect()
}

fn print_board(game: &Game) -> () {
    fn print_piece(square: &Square) -> () {
        let piece_str = match square {
            None => " ",
            Some((White,Pawn)) => "♙",
            Some((Black,Pawn)) => "♟",
        };
        print!("{}",piece_str);
    }

    for rowi in (0..game.state.board.nrows()).rev() {
        let row = &game.state.board.row(rowi);
        for piece in row.iter() {
            print_piece(piece)
        }
        print!("\n");
    }
}

fn main() {
    let game = new_game();
    let ended: &mut bool = &mut false;
    let mut buf = String::new();

    while !*ended {
        print_board(&game);

        println!("What's your move? ('q' to quit)");

        *ended = true;
    }

    // ♟♟
    // ♙♙
    print_board(&game);



    // panics at runtime
    // println!("{:?}", game.board[(2,2)]);
}
