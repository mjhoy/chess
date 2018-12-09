use crate::{
    board::Board, game::Game, m0ve::Move, piece::Piece::*, player::Player, player::Player::*,
    pos::Pos,
};

#[derive(Debug, Clone, Copy)]
pub struct State {
    pub board: Board,
    pub player: Player,
}

impl State {
    /// Is the current player in check?
    pub fn in_check(&self) -> bool {
        let to_pos = self.board.get_king_pos(self.player);
        let next_move_state = State {
            board: self.board,
            player: self.player.other(),
        };

        for from_pos in self.board.coords() {
            if next_move_state.can_move_pseudo(from_pos, to_pos) {
                return true;
            }
        }
        false
    }

    /// Can the current player move the piece, not taking into account
    /// whether the king is in check?
    fn can_move_pseudo(&self, from_pos: Pos, to_pos: Pos) -> bool {
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

        let from = self.board.piece_at(from_pos);
        let to = self.board.piece_at(to_pos);

        match from {
            Some((from_player, piece)) if from_player == self.player => match to {
                Some((to_player, _)) if to_player == self.player => false,
                _ => match piece {
                    Pawn => can_move_pawn(self.player, from_pos, to_pos, to.is_some()),
                    King => can_move_king(self.player, from_pos, to_pos, to.is_some()),
                },
            },
            _ => false,
        }
    }

    /// Can the current player move the piece in `from_pos` to `to_pos`?
    pub fn can_move(&self, from_pos: Pos, to_pos: Pos) -> bool {
        self.can_move_pseudo(from_pos, to_pos) && {
            let next_state = &State {
                player: self.player,
                board: self.board.move_piece(from_pos, to_pos),
            };
            !next_state.in_check()
        }
    }

    /// Generate the next legal moves for this game state.
    /// On^2 for n squares
    pub fn gen_moves(&self) -> Vec<Move> {
        self.board
            .coords()
            .iter()
            .flat_map(|from_pos| {
                self.board
                    .coords()
                    .iter()
                    .filter_map(|to_pos| {
                        if self.can_move(*from_pos, *to_pos) {
                            Some(Move {
                                index: (*from_pos, *to_pos),
                                next: Game {
                                    state: State {
                                        board: self.board.move_piece(*from_pos, *to_pos),
                                        player: self.player.other(),
                                    },
                                },
                            })
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<Move>>()
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_board() -> Board {
        Board::initial()
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

        assert!(white_move.can_move_pseudo(a1, a2));
        assert!(!white_move.can_move_pseudo(a1, a3));
        assert!(!white_move.can_move_pseudo(b3, b2));
        assert!(black_move.can_move_pseudo(b3, b2));
        assert!(black_move.can_move_pseudo(b3, a2));
    }

    #[test]
    fn test_in_check() {
        let not_in_check_board = vec![
            Some((White, Pawn)),
            Some((White, King)),
            Some((White, Pawn)),
            None,
            None,
            None,
            Some((Black, Pawn)),
            Some((Black, King)),
            Some((Black, Pawn)),
        ];
        let not_in_check_state = State {
            board: Board::from_squares(not_in_check_board.as_slice()),
            player: White,
        };

        assert!(!not_in_check_state.in_check());

        let in_check_board_1 = vec![
            // rank 1
            Some((White, Pawn)),
            Some((White, King)),
            Some((White, Pawn)),
            // rank 2
            Some((Black, Pawn)),
            None,
            None,
            // rank 3
            None,
            Some((Black, King)),
            Some((Black, Pawn)),
        ];

        let in_check_state_1 = State {
            board: Board::from_squares(in_check_board_1.as_slice()),
            player: White,
        };
        assert!(in_check_state_1.in_check());

        let in_check_board_2 = vec![
            // rank 1
            None,
            Some((White, King)),
            Some((White, Pawn)),
            // rank 2
            Some((White, Pawn)),
            None,
            Some((Black, Pawn)),
            // rank 3
            Some((Black, Pawn)),
            Some((Black, King)),
            None,
        ];

        let in_check_state_2 = State {
            board: Board::from_squares(in_check_board_2.as_slice()),
            player: Black,
        };
        assert!(in_check_state_2.in_check());
    }
}
