use crate::{piece::Piece::*, player::Player, player::Player::*, pos::Pos, square::Square};
use ansi_term::Colour;
use ansi_term::Style;

use itertools::Itertools;

pub type BoardMatrix = Vec<Square>;

const NSIZE: u8 = 8;

/// 3x3 board
#[derive(Debug, Clone)]
pub struct Board {
    inner: BoardMatrix,
}

impl Board {
    pub fn initial() -> Board {
        let inner = vec![
            // rank 1
            Some((White, Rook)),
            Some((White, Knight)),
            Some((White, Bishop)),
            Some((White, Queen)),
            Some((White, King)),
            Some((White, Bishop)),
            Some((White, Knight)),
            Some((White, Rook)),
            // rank 2
            Some((White, Pawn)),
            Some((White, Pawn)),
            Some((White, Pawn)),
            Some((White, Pawn)),
            Some((White, Pawn)),
            Some((White, Pawn)),
            Some((White, Pawn)),
            Some((White, Pawn)),
            // rank 3
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            // rank 4
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            // rank 5
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            // rank 6
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            // rank 7
            Some((Black, Pawn)),
            Some((Black, Pawn)),
            Some((Black, Pawn)),
            Some((Black, Pawn)),
            Some((Black, Pawn)),
            Some((Black, Pawn)),
            Some((Black, Pawn)),
            Some((Black, Pawn)),
            // rank 8
            Some((Black, Rook)),
            Some((Black, Knight)),
            Some((Black, Bishop)),
            Some((Black, Queen)),
            Some((Black, King)),
            Some((Black, Bishop)),
            Some((Black, Knight)),
            Some((Black, Rook)),
        ];
        Board { inner }
    }

    pub fn from_squares(squares: &[Square]) -> Board {
        Board {
            inner: squares.to_vec(),
        }
    }

    pub fn coords(&self) -> Vec<Pos> {
        (0..NSIZE)
            .cartesian_product(0..NSIZE)
            .map(|(rank, file)| Pos { rank, file })
            .collect()
    }

    pub fn piece_at(&self, pos: Pos) -> Square {
        self.inner[pos.to_offset(NSIZE)]
    }

    pub fn all_empty(&self, positions: &[Pos]) -> bool {
        positions.iter().all(|&pos| self.piece_at(pos).is_none())
    }

    /// Find the position of the king for `player`. Panics if no king is
    /// found.
    pub fn get_king_pos(&self, player: Player) -> Pos {
        for coord in self.coords() {
            if let Some((plyr, King)) = self.piece_at(coord) {
                if plyr == player {
                    return coord;
                }
            }
        }

        panic!("No king on the board")
    }

    /// Move the piece at `from_pos` to `to_pos` and return the new board.
    pub fn move_piece(&self, from: Pos, to: Pos) -> Board {
        let mut new_inner = self.inner.clone();
        let from_piece = self.piece_at(from);
        new_inner[from.to_offset(NSIZE)] = None;
        new_inner[to.to_offset(NSIZE)] = from_piece;

        Board { inner: new_inner }
    }

    pub fn str(&self) -> String {
        fn piece_str(square: Square) -> String {
            let piece_str = match square {
                None => " ",
                Some((White, Pawn)) => "♙",
                Some((White, Bishop)) => "♗",
                Some((White, King)) => "♔",
                Some((White, Rook)) => "♖",
                Some((White, Queen)) => "♕",
                Some((White, Knight)) => "♘",
                Some((Black, Pawn)) => "♟",
                Some((Black, Bishop)) => "♝",
                Some((Black, King)) => "♚",
                Some((Black, Rook)) => "♜",
                Some((Black, Queen)) => "♛",
                Some((Black, Knight)) => "♞",
            };
            piece_str.to_string()
        }

        let mut buf = String::new();

        for rowi in (0..NSIZE).rev() {
            buf.push_str(&format!("{}", rowi + 1));
            for coli in 0..NSIZE {
                let pos = Pos {
                    rank: rowi,
                    file: coli,
                };
                let square = self.inner[pos.to_offset(NSIZE)];
                let color = if (rowi + coli) % 2 == 0 {
                    Colour::RGB(200, 200, 200)
                } else {
                    Colour::RGB(255, 255, 255)
                };
                let mut piece_color = color;
                if let Some((Black, _)) = square {
                    piece_color = Colour::RGB(0, 0, 0);
                } else if let Some((White, _)) = square {
                    piece_color = Colour::RGB(0, 0, 0);
                }

                buf.push_str(&format!(
                    "{}",
                    Style::new()
                        .on(color)
                        .fg(piece_color)
                        .paint(format!("{} ", piece_str(square)))
                ));
            }
            buf.push_str("\n");
        }
        buf.push_str(" ");
        for coli in 0..NSIZE {
            buf.push_str(&format!("{} ", (coli + b'A') as char));
        }

        buf
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::pos::*;

    #[test]
    fn test_piece_at_finds_piece() {
        let board = Board::initial();

        assert_eq!(board.piece_at(e2), Some((White, Pawn)));
        assert_eq!(board.piece_at(e1), Some((White, King)));
        assert_eq!(board.piece_at(b3), None);
    }

    #[test]
    fn test_all_empty() {
        let board = Board::initial();

        assert_eq!(board.all_empty(&[e2]), false);
        assert_eq!(board.all_empty(&[b3]), true);
        assert_eq!(board.all_empty(&[e2, b3]), false);
        assert_eq!(board.all_empty(&[e3, b3]), true);
    }

    #[test]
    fn test_get_king_pos() {
        let board = Board::initial();

        assert_eq!(board.get_king_pos(White), e1);
        assert_eq!(board.get_king_pos(Black), e8);
    }

    #[test]
    fn test_move_piece() {
        let board = Board::initial();
        assert_eq!(board.piece_at(e2), Some((White, Pawn)));
        assert_eq!(board.piece_at(e3), None);

        let next_board = board.move_piece(e2, e3);
        assert_eq!(next_board.piece_at(e2), None);
        assert_eq!(next_board.piece_at(e3), Some((White, Pawn)));
    }
}
