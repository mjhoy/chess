use crate::m0ve::{Action, Move};
use crate::piece::Piece;
use crate::pos::Pos;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MoveDescription {
    Simple { src_piece: Piece, dst_pos: Pos },
    Castle { kingside: bool },
}

impl MoveDescription {
    pub fn match_moves(&self, moves: Vec<Move>) -> Option<Move> {
        for m0ve in moves {
            if self.match_move(&m0ve) {
                return Some(m0ve);
            }
        }
        None
    }

    fn match_move(&self, m0ve: &Move) -> bool {
        match (&m0ve.action, self) {
            (Action::Simple { from: _, to }, MoveDescription::Simple { src_piece, dst_pos }) => {
                let dst_piece = m0ve.next.board.piece_at(*to).map(|(_, piece)| piece);
                dst_pos == to && Some(*src_piece) == dst_piece
            }
            (
                Action::Castle {
                    kingside: action_kingside,
                },
                MoveDescription::Castle {
                    kingside: description_kingside,
                },
            ) => action_kingside == description_kingside,
            (_, _) => false,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::an::parse_an;
    use crate::game::Game;
    use crate::player::Player;
    use crate::pos::*;

    fn test_game() -> Game {
        Game::default()
    }

    #[test]
    fn test_match_moves() {
        let mut game = test_game();
        for desc in &["e3", "e6", "Ke2", "e5", "Kd3", "e4"] {
            let next_moves = game.state.gen_moves();
            let move_desc = parse_an(desc).unwrap();
            game = Game {
                state: move_desc.match_moves(next_moves).unwrap().next,
            };
        }

        assert_eq!(
            game.state.board.piece_at(d3),
            Some((Player::White, Piece::King))
        );

        assert_eq!(
            game.state.board.piece_at(e4),
            Some((Player::Black, Piece::Pawn))
        );
    }
}
