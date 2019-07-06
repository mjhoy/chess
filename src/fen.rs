use crate::board::Board;
use crate::piece::Piece;
use crate::player::Player;
use crate::state::State;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::space1;
use nom::combinator::value;
use nom::error::ErrorKind;
use nom::multi::separated_list;
use nom::Err;
use nom::IResult;

#[derive(Debug, PartialEq, Eq, Clone)]
enum SquareBuilder {
    Empty(u8),
    Occupied((Player, Piece)),
}

// parsers

fn piece(player: Player) -> impl Fn(&str) -> IResult<&str, SquareBuilder> {
    let capitalized = match player {
        Player::White => true,
        Player::Black => false,
    };
    move |input| {
        alt((
            value(
                SquareBuilder::Occupied((player, Piece::Bishop)),
                tag(if capitalized { "B" } else { "b" }),
            ),
            value(
                SquareBuilder::Occupied((player, Piece::King)),
                tag(if capitalized { "K" } else { "k" }),
            ),
            value(
                SquareBuilder::Occupied((player, Piece::Pawn)),
                tag(if capitalized { "P" } else { "p" }),
            ),
            value(
                SquareBuilder::Occupied((player, Piece::Rook)),
                tag(if capitalized { "R" } else { "r" }),
            ),
        ))(input)
    }
}

fn square_builder(input: &str) -> IResult<&str, SquareBuilder> {
    alt((
        piece(Player::White),
        piece(Player::Black),
        // empty squares
        value(SquareBuilder::Empty(1), tag("1")),
        value(SquareBuilder::Empty(2), tag("2")),
        value(SquareBuilder::Empty(3), tag("3")),
        value(SquareBuilder::Empty(4), tag("4")),
        value(SquareBuilder::Empty(5), tag("5")),
        value(SquareBuilder::Empty(6), tag("6")),
        value(SquareBuilder::Empty(7), tag("7")),
        value(SquareBuilder::Empty(8), tag("8")),
    ))(input)
}

fn row(input: &str) -> IResult<&str, Vec<SquareBuilder>> {
    let mut row: Vec<SquareBuilder> = vec![];
    let mut full = false;
    let mut count = 0;
    let mut input = input;

    while !full {
        let (next_input, sb) = square_builder(input)?;
        input = next_input;
        match sb {
            SquareBuilder::Empty(n) => {
                count += n;
            }
            _ => {
                count += 1;
            }
        }
        if count > 8 {
            return Err(Err::Error((input, ErrorKind::TooLarge)));
        }
        if count == 8 {
            full = true;
        }
        row.push(sb);
    }
    Ok((input, row))
}

// For now, this is just whose turn it is.
fn extra_state(input: &str) -> IResult<&str, Player> {
    alt((
        value(Player::White, tag("w")),
        value(Player::Black, tag("b")),
    ))(input)
}

pub fn piece_to_fen(player_piece: (Player, Piece)) -> String {
    let (player, piece) = player_piece;
    let piece_str = match piece {
        Piece::Pawn => "p",
        Piece::Bishop => "b",
        Piece::King => "k",
        Piece::Rook => "r",
    };
    if player == Player::White {
        piece_str.to_uppercase().to_string()
    } else {
        piece_str.to_string()
    }
}

/// Parses Forsyth-Edwards notation:
/// https://en.wikipedia.org/wiki/Forsyth–Edwards_Notation
pub fn fen(input: &str) -> IResult<&str, State> {
    let (input, rows) = separated_list(tag("/"), row)(input)?;
    if rows.len() != 8 {
        // TODO: custom error
        return Err(Err::Error((input, ErrorKind::TooLarge)));
    }

    let mut squares = Vec::new();
    for row in rows.iter().rev() {
        // fen starts at 8th rank and moves back
        for builder in row {
            match builder {
                SquareBuilder::Occupied((player, piece)) => squares.push(Some((*player, *piece))),
                SquareBuilder::Empty(n) => {
                    for _ in 0..*n {
                        squares.push(None)
                    }
                }
            }
        }
    }

    assert!(
        squares.len() == 64,
        "parsed board matrix expected to be length 64"
    );

    let (input, _) = space1(input)?;
    let (input, player) = extra_state(input)?;

    let board = Board::from_squares(squares.as_slice());
    Ok((input, State { board, player }))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::pos::*;

    #[test]
    fn test_parse_white_row() {
        let input = "K7";
        assert_eq!(
            row(input),
            Ok((
                "",
                vec![
                    SquareBuilder::Occupied((Player::White, Piece::King)),
                    SquareBuilder::Empty(7)
                ]
            ))
        );
    }

    #[test]
    fn test_parse_black_row() {
        let input = "k7";
        assert_eq!(
            row(input),
            Ok((
                "",
                vec![
                    SquareBuilder::Occupied((Player::Black, Piece::King)),
                    SquareBuilder::Empty(7)
                ]
            ))
        );
    }

    #[test]
    fn test_parse_too_large_row() {
        let input = "6k7";
        assert_eq!(row(input), Err(Err::Error(("", ErrorKind::TooLarge))));
    }

    #[test]
    fn test_parse_valid_fen() {
        let input = "r1b1kb1r/pppppppp/8/8/4P3/8/PPPP1PPP/R1B1KB1R b";
        let state_res = fen(input);
        assert!(state_res.is_ok(), "able to parse fen");
        let (_rest, state) = state_res.unwrap();
        assert_eq!(state.board.piece_at(e4), Some((Player::White, Piece::Pawn)));
        assert_eq!(state.board.piece_at(e8), Some((Player::Black, Piece::King)));
        assert_eq!(state.board.piece_at(e1), Some((Player::White, Piece::King)));
        assert_eq!(state.board.piece_at(h2), Some((Player::White, Piece::Pawn)));
        assert_eq!(state.board.piece_at(h7), Some((Player::Black, Piece::Pawn)));
    }

    #[test]
    fn test_parse_invalid_fen() {
        let input = "r1b1kb1r/pppppppp/8/8/4P3/8/PPPP1PPP/R1B1KB1R/8 b";
        let state_res = fen(input);
        assert!(state_res.is_err(), "recognizes invalid fen");
    }
}
