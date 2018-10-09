extern crate nalgebra as na;
#[macro_use] extern crate itertools;

use na::{U3, MatrixArray, Matrix, RowVector3};

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
    King,
}

use Piece::*;

pub type Square = Option<(Player, Piece)>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pos {
    rank: u8,
    file: u8
}

#[derive(Debug, Clone, Copy)]
pub struct State {
    pub board: Board,
    pub player: Player,
}

/// 3x3 board
pub type Board = Matrix<Square, U3, U3, MatrixArray<Square, U3, U3>>;

pub struct Game {
    pub state: State,
}

pub struct Move {
    index: (Pos,Pos),
    pub next: Game
}

fn initial_board() -> Board {
    Board::from_rows(&[
        RowVector3::new(Some((White, Pawn)), Some((White,King)), Some((White,Pawn))),
        RowVector3::new(None, None, None),
        RowVector3::new(Some((Black, Pawn)), Some((Black,King)), Some((Black,Pawn))),
    ])
}

fn coords(board: &Board) -> Vec<Pos> {
    iproduct!(0..board.nrows(), 0..board.ncols()).map(|(rank, file)| {
        Pos { rank: rank as u8, file: file as u8 }
    }).collect()
}

fn piece_at(pos: &Pos, board: &Board) -> Square {
    board.row(pos.rank as usize)[pos.file as usize]
}

/// Initial game.
pub fn new_game() -> Game {
    let board = initial_board();
    let player = White;
    let state = State { board, player, };
    Game {
        state: state,
    }
}

/// Find the position of the king for `player`. Panics if no king is
/// found.
pub fn get_king_pos(board: &Board, player: &Player) -> Pos {
    for coord in coords(board) {
        if let Some((plyr, King)) = piece_at(&coord, board)  {
            if &plyr == player { return coord; }
        }
    }

    panic!("No king on the board")
}

/// Is the current player in check?
pub fn in_check(state: &State) -> bool {
    let to_pos = get_king_pos(&state.board, &state.player);
    let next_move_state = State {
        board: state.board,
        player: state.player.other(),
    };

    for from_pos in coords(&state.board) {
        if can_move_pseudo(&next_move_state, &from_pos, &to_pos) {
            return true;
        }
    }
    false
}

/// Can the current player move the piece, not taking into account
/// whether the king is in check?
fn can_move_pseudo(state: &State, from_pos: &Pos, to_pos: &Pos) -> bool {
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

    fn can_move_king(_player: &Player, from_pos: &Pos, to_pos: &Pos, _capture: bool) -> bool {
        ((from_pos.rank as i32) - (to_pos.rank as i32)).abs() <= 1 &&
            ((from_pos.file as i32) - (to_pos.file as i32)).abs() <= 1
    }

    let from = piece_at(from_pos, &state.board);
    let to   = piece_at(to_pos, &state.board);

    match from {
        Some((from_player, piece)) if from_player == state.player => {
            match to {
                Some((to_player, _)) if to_player == state.player => false,
                _ => {
                    match piece {
                        Pawn => can_move_pawn(&state.player, &from_pos, &to_pos, to.is_some()),
                        King => can_move_king(&state.player, &from_pos, &to_pos, to.is_some()),
                    }
                }
            }
        }
        _ => false
    }
}

/// Can the current player move the piece in `from_pos` to `to_pos`?
pub fn can_move(state: &State, from_pos: &Pos, to_pos: &Pos) -> bool {
    if can_move_pseudo(state, from_pos, to_pos) {
        !in_check(&State { player: state.player,
                           board: move_piece(&state.board, from_pos, to_pos) })
    } else {
        false
    }
}

/// Move the piece at `from_pos` to `to_pos` and return the new board.
pub fn move_piece(board: &Board, from_pos: &Pos, to_pos: &Pos) -> Board {
    let new_board: &mut Board = &mut board.clone();
    let from = piece_at(from_pos, board);
    new_board[(from_pos.rank as usize, from_pos.file as usize)] = None;
    new_board[(to_pos.rank as usize, to_pos.file as usize)] = from;

    new_board.clone()
}

/// Generate the next legal moves for this game state.
/// On^2 for n squares
pub fn gen_moves(state: &State) -> Vec<Move> {
    coords(&state.board).iter().flat_map(|from_pos| {
        coords(&state.board).iter().filter_map(|to_pos| {
            if can_move(&state, from_pos, to_pos) {
                Some(Move {
                    index: (*from_pos, *to_pos),
                    next: Game {
                        state: State {
                            board: move_piece(&state.board, from_pos, to_pos),
                            player: state.player.other(),
                        },
                    }
                })
            } else {
                None
            }
        }).collect::<Vec<Move>>()
    }).collect()
}

/// Pretty print the board.
pub fn board_str(game: &Game) -> String {
    fn piece_str(square: &Square) -> String {
        let piece_str = match square {
            None => " ",
            Some((White,Pawn)) => "♙",
            Some((White,King)) => "♔",
            Some((Black,Pawn)) => "♟",
            Some((Black,King)) => "♚",
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

/// Pretty print a move.
pub fn move_str(m0ve: &Move) -> String {
    let (from, to) = m0ve.index;
    let from_file = (from.file + 'A' as u8) as char;
    let from_rank = from.rank + 1;
    let to_file = (to.file + 'A' as u8) as char;
    let to_rank = to.rank + 1;
    format!("{}{} -> {}{}", from_file, from_rank, to_file, to_rank)
}

/// Pretty print a player.
pub fn player_str(player: Player) -> &'static str {
    match player {
        White => "White",
        Black => "Black",
    }
}

#[cfg(test)]
mod test {

    use ::*;

    fn test_board() -> Board {
        Board::from_rows(&[
            RowVector3::new(Some((White, Pawn)), Some((White, King)), Some((White,Pawn))),
            RowVector3::new(None, None, None),
            RowVector3::new(Some((Black, Pawn)), Some((Black, King)), Some((Black,Pawn))),
        ])
    }

    #[test]
    fn test_new_game_starts_white() {
        let game = new_game();
        assert_eq!(game.state.player, White);
    }

    #[test]
    fn test_piece_at_finds_piece() {
        let a1 = &Pos {rank:0, file: 0};
        let a2 = &Pos {rank:1, file: 0};
        let b3 = &Pos {rank:2, file: 1};

        let board = &test_board();

        assert_eq!(piece_at(a1, board), Some((White, Pawn)));
        assert_eq!(piece_at(a2, board), None);
        assert_eq!(piece_at(b3, board), Some((Black, King)));
    }

    #[test]
    fn test_coords() {
        let board = &test_board();
        assert_eq!(::coords(board), vec![
            Pos{rank:0, file:0},
            Pos{rank:0, file:1},
            Pos{rank:0, file:2},
            Pos{rank:1, file:0},
            Pos{rank:1, file:1},
            Pos{rank:1, file:2},
            Pos{rank:2, file:0},
            Pos{rank:2, file:1},
            Pos{rank:2, file:2},
        ]);
    }

    #[test]
    fn test_can_move_pseudo() {
        let board = &test_board();
        let a1 = &Pos {rank:0, file: 0};
        let a2 = &Pos {rank:1, file: 0};
        let a3 = &Pos {rank:2, file: 0};
        let b2 = &Pos {rank:1, file: 1};
        let b3 = &Pos {rank:2, file: 1};

        let white_move = State { board: *board, player: White };
        let black_move = State { board: *board, player: Black };

        assert!(can_move_pseudo(&white_move, a1, a2));
        assert!(!can_move_pseudo(&white_move, a1, a3));
        assert!(!can_move_pseudo(&white_move, b3, b2));
        assert!(can_move_pseudo(&black_move, b3, b2));
        assert!(can_move_pseudo(&black_move, b3, a2));
    }

    #[test]
    fn test_get_king_pos() {
        let board = &test_board();

        assert_eq!(get_king_pos(board, &White), Some(Pos {rank: 0, file: 1}));
        assert_eq!(get_king_pos(board, &Black), Some(Pos {rank: 2, file: 1}));
    }

    #[test]
    fn test_in_check() {
        let not_in_check_board = Board::from_rows(&[
            RowVector3::new(
                Some((White, Pawn)),
                Some((White, King)),
                Some((White, Pawn)),
            ),
            RowVector3::new(None, None, None),
            RowVector3::new(
                Some((Black, Pawn)),
                Some((Black, King)),
                Some((Black, Pawn)),
            ),
        ]);

        assert!(!in_check(&State {
            board: not_in_check_board,
            player: White
        }));

        let in_check_board_1 = Board::from_rows(&[
            RowVector3::new(
                Some((White, Pawn)),
                Some((White, King)),
                Some((White, Pawn)),
            ),
            RowVector3::new(Some((Black, Pawn)), None, None),
            RowVector3::new(None, Some((Black, King)), Some((Black, Pawn))),
        ]);

        assert!(in_check(&State {
            board: in_check_board_1,
            player: White
        }));

        let in_check_board_2 = Board::from_rows(&[
            RowVector3::new(None, Some((White, King)), Some((White, Pawn))),
            RowVector3::new(Some((White, Pawn)), None, Some((Black, Pawn))),
            RowVector3::new(Some((Black, Pawn)), Some((Black, King)), None),
        ]);

        assert!(in_check(&State {
            board: in_check_board_2,
            player: Black
        }));
    }
}
