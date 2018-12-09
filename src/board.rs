use itertools::iproduct;

use crate::{Piece::*, Player, Player::*, Pos, Square};
use na::{Matrix, MatrixArray, RowVector3, U3};

pub type BoardMatrix = Matrix<Square, U3, U3, MatrixArray<Square, U3, U3>>;

/// 3x3 board
#[derive(Debug, Clone, Copy)]
pub struct Board {
    pub board: BoardMatrix,
}

impl Board {
    pub fn initial() -> Board {
        let board = BoardMatrix::from_rows(&[
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
        Board { board }
    }

    pub fn from_squares(squares: Vec<Square>) -> Board {
        Board {
            board: BoardMatrix::from_row_slice_generic(U3, U3, squares.as_slice()),
        }
    }

    pub fn coords(&self) -> Vec<Pos> {
        iproduct!(0..self.board.nrows(), 0..self.board.ncols())
            .map(|(rank, file)| Pos {
                rank: rank as u8,
                file: file as u8,
            })
            .collect()
    }

    pub fn piece_at(&self, pos: Pos) -> Square {
        self.board.row(pos.rank as usize)[pos.file as usize]
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
    pub fn move_piece(&self, from_pos: Pos, to_pos: Pos) -> Board {
        let mut board = self.board;
        let new_board: &mut BoardMatrix = &mut board;
        let from = self.piece_at(from_pos);
        new_board[(from_pos.rank as usize, from_pos.file as usize)] = None;
        new_board[(to_pos.rank as usize, to_pos.file as usize)] = from;

        Board { board: *new_board }
    }

    pub fn str(&self) -> String {
        fn piece_str(square: Square) -> String {
            let piece_str = match square {
                None => " ",
                Some((White, Pawn)) => "♙",
                Some((White, King)) => "♔",
                Some((Black, Pawn)) => "♟",
                Some((Black, King)) => "♚",
            };
            piece_str.to_string()
        }

        let mut buf = String::new();

        for rowi in (0..self.board.nrows()).rev() {
            let row = self.board.row(rowi);
            for piece in row.iter() {
                buf.push_str(&piece_str(*piece));
            }
            buf.push_str("\n");
        }

        buf
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_piece_at_finds_piece() {
        let a1 = Pos { rank: 0, file: 0 };
        let a2 = Pos { rank: 1, file: 0 };
        let b3 = Pos { rank: 2, file: 1 };

        let board = Board::initial();

        assert_eq!(board.piece_at(a1), Some((White, Pawn)));
        assert_eq!(board.piece_at(a2), None);
        assert_eq!(board.piece_at(b3), Some((Black, King)));
    }

    #[test]
    fn test_coords() {
        let board = Board::initial();
        assert_eq!(
            board.coords(),
            vec![
                Pos { rank: 0, file: 0 },
                Pos { rank: 0, file: 1 },
                Pos { rank: 0, file: 2 },
                Pos { rank: 1, file: 0 },
                Pos { rank: 1, file: 1 },
                Pos { rank: 1, file: 2 },
                Pos { rank: 2, file: 0 },
                Pos { rank: 2, file: 1 },
                Pos { rank: 2, file: 2 },
            ]
        );
    }

    #[test]
    fn test_get_king_pos() {
        let board = Board::initial();

        assert_eq!(board.get_king_pos(White), Pos { rank: 0, file: 1 });
        assert_eq!(board.get_king_pos(Black), Pos { rank: 2, file: 1 });
    }

    #[test]
    fn test_from_squares_in_row_major_order() {
        let board_squares = vec![
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

        let board = Board::from_squares(board_squares);

        assert_eq!(
            board.piece_at(Pos { rank: 0, file: 0 }),
            Some((White, Pawn))
        );
        assert_eq!(
            board.piece_at(Pos { rank: 1, file: 0 }),
            Some((Black, Pawn))
        );
        assert_eq!(board.piece_at(Pos { rank: 2, file: 0 }), None,);
        assert_eq!(
            board.piece_at(Pos { rank: 2, file: 1 }),
            Some((Black, King))
        );
    }
}
