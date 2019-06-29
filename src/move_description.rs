use crate::m0ve::Move;
use crate::piece::Piece;
use crate::pos::Pos;
use std::result::Result;

mod parser;

#[derive(Debug, PartialEq, Eq)]
pub struct MoveDescription {
    pub src_piece: Piece,
    pub dst_pos: Pos,
}

impl MoveDescription {
    pub fn parse(input: &str) -> Result<MoveDescription, String> {
        match parser::move_description(input) {
            Ok((ref rem, ref _md)) if !rem.is_empty() => {
                Err("parsing error: extra characters".to_string())
            }
            Ok((_remaining, md)) => Ok(md),
            Err(e) => Err(format!("parsing error: {:?}", e)),
        }
    }

    pub fn match_moves(&self, moves: Vec<Move>) -> Option<Move> {
        for m0ve in moves {
            if self.match_move(&m0ve) {
                return Some(m0ve);
            }
        }
        None
    }

    fn match_move(&self, m0ve: &Move) -> bool {
        let (_from, to) = m0ve.index;
        let dst_piece = m0ve
            .next
            .state
            .board
            .piece_at(to)
            .map(|(_player, piece)| piece);
        self.dst_pos == to && Some(self.src_piece) == dst_piece
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::game::Game;
    use crate::player::Player;
    use crate::pos::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            MoveDescription::parse("Ke2"),
            Ok(MoveDescription {
                src_piece: Piece::King,
                dst_pos: e2,
            })
        );
        assert_eq!(
            MoveDescription::parse("Ze2"),
            Err(r#"parsing error: Error(("Ze2", Alt))"#.to_string())
        );
        assert_eq!(
            MoveDescription::parse("Ke2junk"),
            Err(r#"parsing error: extra characters"#.to_string())
        );
    }

    fn test_game() -> Game {
        Game::default()
    }

    #[test]
    fn test_match_moves() {
        let mut game = test_game();
        for desc in &["e3", "e6", "Ke2", "e5", "Kd3", "e4"] {
            let next_moves = game.state.gen_moves();
            let move_desc = MoveDescription::parse(desc).unwrap();
            game = move_desc.match_moves(next_moves).unwrap().next;
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
