use std::io;

extern crate nalgebra as na;
#[macro_use] extern crate itertools;

use na::{U2, U3, MatrixArray, Matrix, RowVector2};

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

// start with a 2x3 board
type Board = Matrix<Square, U3, U2, MatrixArray<Square, U3, U2>>;

#[derive(Debug, Clone, Copy)]
struct GameState {
    board: Board,
    current_player: Player,
}

struct Game {
    state: GameState,
}

struct Move {
    index: (Pos,Pos),
    next: Game
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

fn new_game() -> Game {
    let board = initial_board();
    let current_player = White;
    let state = GameState { board, current_player, };
    Game {
        state: state,
    }
}

// movement logic
fn can_move(board: &Board, player: &Player, from_pos: &Pos, to_pos: &Pos) -> bool {
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

fn move_piece(board: &Board, from_pos: &Pos, to_pos: &Pos) -> Board {
    let new_board: &mut Board = &mut board.clone();
    let from = piece_at(from_pos, board);
    new_board[(from_pos.rank as usize, from_pos.file as usize)] = None;
    new_board[(to_pos.rank as usize, to_pos.file as usize)] = from;

    new_board.clone()
}

// On^2 for n squares
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
                    }
                })
            } else {
                None
            }
        }).collect::<Vec<Move>>()
    }).collect()
}

fn board_str(game: &Game) -> String {
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

fn move_str(m0ve: &Move) -> String {
    let (from, to) = m0ve.index;
    let from_file = (from.file + 'A' as u8) as char;
    let from_rank = from.rank + 1;
    let to_file = (to.file + 'A' as u8) as char;
    let to_rank = to.rank + 1;
    format!("{}{} -> {}{}", from_file, from_rank, to_file, to_rank)
}

fn player_str(player: Player) -> &'static str {
    match player {
        White => "White",
        Black => "Black",
    }
}

fn main() {
    let mut game = new_game();
    let ended: &mut bool = &mut false;
    let mut buf = String::new();

    while !*ended {
        println!("\n{}", board_str(&game));

        let moves = gen_moves(game.state);

        if moves.len() == 0 {
            println!("Game over! RET quits.");
            io::stdin().read_line(&mut buf).unwrap();
            *ended = true;
            break;
        }

        println!("{}'s move.", player_str(game.state.current_player));
        for i in 0..moves.len() {
            let m0ve = &moves[i];
            println!("{}: {}", i+1, move_str(&m0ve));
        }

        println!("Enter {}..{} ('q' quits)", 1, moves.len());

        io::stdin().read_line(&mut buf).unwrap();

        if buf.trim() == "q" {
            *ended = true;
            break;
        }

        match buf.trim().parse::<usize>() {
            Ok(i) => {
                if i <= moves.len() && i > 0 {
                    game = moves.into_iter().nth(i - 1).expect("checked index").next;
                } else {
                    println!("Please enter a number between {} and {}.", 1, moves.len());
                }
            }
            Err(_) => {
                println!("Please enter a number between {} and {}.", 1, moves.len());
            }
        }

        buf = String::new();
    }
}
