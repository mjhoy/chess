extern crate nalgebra as na;
#[macro_use]
extern crate itertools;

pub mod player;
use player::Player;
use player::Player::*;

pub mod piece;
use piece::Piece;
use Piece::*;

pub mod square;
use square::Square;

pub mod pos;
use pos::Pos;

pub mod board;
use board::Board;

pub mod state;
use state::State;

pub mod game;
use game::Game;

pub mod m0ve;
use m0ve::Move;

/// Initial game.
pub fn new_game() -> Game {
    let board = Board::initial();
    let player = White;
    let state = State { board, player };
    Game { state }
}

/// Is the current player in check?
pub fn in_check(state: &State) -> bool {
    let to_pos = state.board.get_king_pos(state.player);
    let next_move_state = State {
        board: state.board,
        player: state.player.other(),
    };

    for from_pos in state.board.coords() {
        if can_move_pseudo(&next_move_state, from_pos, to_pos) {
            return true;
        }
    }
    false
}

/// Can the current player move the piece, not taking into account
/// whether the king is in check?
fn can_move_pseudo(state: &State, from_pos: Pos, to_pos: Pos) -> bool {
    fn can_move_pawn(player: Player, from_pos: Pos, to_pos: Pos, capture: bool) -> bool {
        let next_rank = i32::from(from_pos.rank) + if player == White { 1 } else { -1 };
        if to_pos.rank != next_rank as u8 {
            return false;
        }

        if capture {
            (to_pos.file > 0 /* u8 guard */ && from_pos.file == to_pos.file - 1)
                || from_pos.file == to_pos.file + 1
        } else {
            from_pos.file == to_pos.file
        }
    }

    fn can_move_king(_player: Player, from_pos: Pos, to_pos: Pos, _capture: bool) -> bool {
        (i32::from(from_pos.rank) - i32::from(to_pos.rank)).abs() <= 1
            && (i32::from(from_pos.file) - i32::from(to_pos.file)).abs() <= 1
    }

    let from = state.board.piece_at(from_pos);
    let to = state.board.piece_at(to_pos);

    match from {
        Some((from_player, piece)) if from_player == state.player => match to {
            Some((to_player, _)) if to_player == state.player => false,
            _ => match piece {
                Pawn => can_move_pawn(state.player, from_pos, to_pos, to.is_some()),
                King => can_move_king(state.player, from_pos, to_pos, to.is_some()),
            },
        },
        _ => false,
    }
}

/// Can the current player move the piece in `from_pos` to `to_pos`?
pub fn can_move(state: &State, from_pos: Pos, to_pos: Pos) -> bool {
    can_move_pseudo(state, from_pos, to_pos) && !in_check(&State {
        player: state.player,
        board: state.board.move_piece(from_pos, to_pos),
    })
}

/// Generate the next legal moves for this game state.
/// On^2 for n squares
pub fn gen_moves(state: &State) -> Vec<Move> {
    state
        .board
        .coords()
        .iter()
        .flat_map(|from_pos| {
            state
                .board
                .coords()
                .iter()
                .filter_map(|to_pos| {
                    if can_move(&state, *from_pos, *to_pos) {
                        Some(Move {
                            index: (*from_pos, *to_pos),
                            next: Game {
                                state: State {
                                    board: state.board.move_piece(*from_pos, *to_pos),
                                    player: state.player.other(),
                                },
                            },
                        })
                    } else {
                        None
                    }
                }).collect::<Vec<Move>>()
        }).collect()
}

/// Pretty print a move.
pub fn move_str(m0ve: &Move) -> String {
    let (from, to) = m0ve.index;
    let from_file = (from.file + b'A') as char;
    let from_rank = from.rank + 1;
    let to_file = (to.file + b'A') as char;
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

    use na::RowVector3;
    use *;

    fn test_board() -> Board {
        Board::initial()
    }

    #[test]
    fn test_new_game_starts_white() {
        let game = new_game();
        assert_eq!(game.state.player, White);
    }

    #[test]
    fn test_can_move_pseudo() {
        let board = &test_board();
        let a1 = Pos { rank: 0, file: 0 };
        let a2 = Pos { rank: 1, file: 0 };
        let a3 = Pos { rank: 2, file: 0 };
        let b2 = Pos { rank: 1, file: 1 };
        let b3 = Pos { rank: 2, file: 1 };

        let white_move = State {
            board: *board,
            player: White,
        };
        let black_move = State {
            board: *board,
            player: Black,
        };

        assert!(can_move_pseudo(&white_move, a1, a2));
        assert!(!can_move_pseudo(&white_move, a1, a3));
        assert!(!can_move_pseudo(&white_move, b3, b2));
        assert!(can_move_pseudo(&black_move, b3, b2));
        assert!(can_move_pseudo(&black_move, b3, a2));
    }

    #[test]
    fn test_in_check() {
        let not_in_check_board = board::BoardMatrix::from_rows(&[
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
            board: Board {
                board: not_in_check_board
            },
            player: White
        }));

        let in_check_board_1 = board::BoardMatrix::from_rows(&[
            RowVector3::new(
                Some((White, Pawn)),
                Some((White, King)),
                Some((White, Pawn)),
            ),
            RowVector3::new(Some((Black, Pawn)), None, None),
            RowVector3::new(None, Some((Black, King)), Some((Black, Pawn))),
        ]);

        assert!(in_check(&State {
            board: Board {
                board: in_check_board_1
            },
            player: White
        }));

        let in_check_board_2 = board::BoardMatrix::from_rows(&[
            RowVector3::new(None, Some((White, King)), Some((White, Pawn))),
            RowVector3::new(Some((White, Pawn)), None, Some((Black, Pawn))),
            RowVector3::new(Some((Black, Pawn)), Some((Black, King)), None),
        ]);

        assert!(in_check(&State {
            board: Board {
                board: in_check_board_2
            },
            player: Black
        }));
    }
}
