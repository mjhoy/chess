extern crate nalgebra as na;
#[macro_use] extern crate itertools;

use na::{U2, U3, MatrixArray, Matrix, RowVector2};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Player {
    White,
    Black,
}

use Player::*;

impl Player {
    pub fn other(&self) -> Player {
        match &self {
            White => Black,
            Black => White,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Piece {
    Pawn,
}

use Piece::*;

pub type Square = Option<(Player, Piece)>;

#[derive(Debug, Clone, Copy)]
pub struct Pos {
    rank: u8,
    file: u8
}

#[derive(Debug, Clone, Copy)]
pub struct GameState {
    pub board: Board,
    pub current_player: Player,
}

// start with a 2x3 board
pub type Board = Matrix<Square, U3, U2, MatrixArray<Square, U3, U2>>;

pub struct Game {
    pub state: GameState,
}

pub struct Move {
    index: (Pos,Pos),
    pub next: Game
}

fn initial_board() -> Board {
    Board::from_rows(&[
        RowVector2::new(Some((White, Pawn)), Some((White,Pawn))),
        RowVector2::new(None, None),
        RowVector2::new(Some((Black, Pawn)), Some((Black,Pawn))),
    ])
}

fn coords(board: &Board) -> Vec<Pos> {
    iproduct!(0..board.nrows(), 0..board.ncols()).map(|(rank, file)| {
        Pos { rank: rank as u8, file: file as u8 }
    }).collect()
}

fn piece_at(pos: &Pos, board: &Board) -> Option<(Player,Piece)> {
    board.row(pos.rank as usize)[pos.file as usize]
}

pub fn new_game() -> Game {
    let board = initial_board();
    let current_player = White;
    let state = GameState { board, current_player, };
    Game {
        state: state,
    }
}

// movement logic
pub fn can_move(board: &Board, player: &Player, from_pos: &Pos, to_pos: &Pos) -> bool {
    fn can_move_pawn(player: &Player, from_pos: &Pos, to_pos: &Pos, capture: bool) -> bool {
        let next_rank = from_pos.rank as i32 + if *player == White { 1 } else { -1 };
        if to_pos.rank != next_rank as u8 { return false; }

        if capture {
            (to_pos.file > 0 /* u8 guard */ && from_pos.file == to_pos.file - 1) ||
                from_pos.file == to_pos.file + 1
        } else {
            from_pos.file == to_pos.file
        }
    }

    let from = piece_at(from_pos, board);
    let to   = piece_at(to_pos, board);

    match from {
        Some((from_player, piece)) if from_player == *player => {
            match to {
                Some((to_player, _)) if to_player == *player => false,
                _ => {
                    match piece {
                        Pawn => can_move_pawn(&player, &from_pos, &to_pos, to.is_some())
                    }
                }
            }
        }
        _ => false
    }
}

pub fn move_piece(board: &Board, from_pos: &Pos, to_pos: &Pos) -> Board {
    let new_board: &mut Board = &mut board.clone();
    let from = piece_at(from_pos, board);
    new_board[(from_pos.rank as usize, from_pos.file as usize)] = None;
    new_board[(to_pos.rank as usize, to_pos.file as usize)] = from;

    new_board.clone()
}

// On^2 for n squares
pub fn gen_moves(state: GameState) -> Vec<Move> {
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
                    }
                })
            } else {
                None
            }
        }).collect::<Vec<Move>>()
    }).collect()
}

pub fn board_str(game: &Game) -> String {
    fn piece_str(square: &Square) -> String {
        let piece_str = match square {
            None => " ",
            Some((White,Pawn)) => "♙",
            Some((Black,Pawn)) => "♟",
        };
        format!("{}",piece_str)
    }

    let mut buf = String::new();

    for rowi in (0..game.state.board.nrows()).rev() {
        let row = &game.state.board.row(rowi);
        for piece in row.iter() {
            buf.push_str(&piece_str(piece));
        }
        buf.push_str("\n");
    }

    buf
}

pub fn move_str(m0ve: &Move) -> String {
    let (from, to) = m0ve.index;
    let from_file = (from.file + 'A' as u8) as char;
    let from_rank = from.rank + 1;
    let to_file = (to.file + 'A' as u8) as char;
    let to_rank = to.rank + 1;
    format!("{}{} -> {}{}", from_file, from_rank, to_file, to_rank)
}

pub fn player_str(player: Player) -> &'static str {
    match player {
        White => "White",
        Black => "Black",
    }
}
