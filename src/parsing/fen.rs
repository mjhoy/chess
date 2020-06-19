use crate::board::Board;
use crate::castles::{CastleAbility, Castles};
use crate::parsing::algebraic_notation::pos;
use crate::piece::Piece;
use crate::player::Player;
use crate::pos::Pos;
use crate::state::State;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::space1;
use nom::combinator::{map, value};
use nom::error::ErrorKind;
use nom::multi::{many1, separated_list};
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
            value(
                SquareBuilder::Occupied((player, Piece::Queen)),
                tag(if capitalized { "Q" } else { "q" }),
            ),
            value(
                SquareBuilder::Occupied((player, Piece::Knight)),
                tag(if capitalized { "N" } else { "n" }),
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

fn current_player(input: &str) -> IResult<&str, Player> {
    alt((
        value(Player::White, tag("w")),
        value(Player::Black, tag("b")),
    ))(input)
}

fn en_passant_pos(input: &str) -> IResult<&str, Option<Pos>> {
    alt((value(None, tag("-")), map(pos, Some)))(input)
}

fn castling(input: &str) -> IResult<&str, Castles> {
    let (input, res) = many1(alt((tag("-"), tag("K"), tag("Q"), tag("k"), tag("q"))))(input)?;

    Ok((
        input,
        Castles {
            white: CastleAbility {
                kingside: res.contains(&"K"),
                queenside: res.contains(&"Q"),
            },
            black: CastleAbility {
                kingside: res.contains(&"k"),
                queenside: res.contains(&"q"),
            },
        },
    ))
}

pub fn piece_to_fen(player_piece: (Player, Piece)) -> String {
    let (player, piece) = player_piece;
    let piece_str = match piece {
        Piece::Pawn => "p",
        Piece::Bishop => "b",
        Piece::King => "k",
        Piece::Rook => "r",
        Piece::Queen => "q",
        Piece::Knight => "n",
    };
    if player == Player::White {
        piece_str.to_uppercase()
    } else {
        piece_str.to_string()
    }
}

fn fen(input: &str) -> IResult<&str, State> {
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

    assert_eq!(
        squares.len(),
        64,
        "parsed board matrix expected to be length 64"
    );

    let (input, _) = space1(input)?;
    let (input, player) = current_player(input)?;
    let (input, _) = space1(input)?;
    let (input, castling) = castling(input)?;
    let (input, _) = space1(input)?;
    let (input, en_passant) = en_passant_pos(input)?;

    let board = Board::from_squares(squares.as_slice());
    Ok((
        input,
        State {
            board,
            player,
            en_passant,
            castling,
        },
    ))
}

/// Parses Forsyth-Edwards notation:
/// https://en.wikipedia.org/wiki/Forsyth–Edwards_Notation
pub fn parse_fen(input: &str) -> Result<State, String> {
    match fen(input) {
        Ok((_, state)) => Ok(state),
        _ => Err(format!("parsing error: {}", input)),
    }
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
    fn test_parse_castling() {
        assert_eq!(castling("KQkq"), Ok(("", Castles::initial())));
        assert_eq!(
            castling("Qkq"),
            Ok((
                "",
                Castles {
                    white: CastleAbility {
                        queenside: true,
                        kingside: false,
                    },
                    black: CastleAbility {
                        queenside: true,
                        kingside: true,
                    },
                }
            ))
        );
        assert_eq!(
            castling("-"),
            Ok((
                "",
                Castles {
                    white: CastleAbility {
                        queenside: false,
                        kingside: false,
                    },
                    black: CastleAbility {
                        queenside: false,
                        kingside: false,
                    },
                }
            ))
        );
    }

    #[test]
    fn test_parse_valid_fen() {
        let input = "r1b1kb1r/pppppppp/8/8/4P3/8/PPPP1PPP/R1B1KB1R b Kq e3";
        let state_res = fen(input);
        assert!(state_res.is_ok(), "able to parse fen");
        let (_rest, state) = state_res.unwrap();
        assert_eq!(state.board.piece_at(e4), Some((Player::White, Piece::Pawn)));
        assert_eq!(state.board.piece_at(e8), Some((Player::Black, Piece::King)));
        assert_eq!(state.board.piece_at(e1), Some((Player::White, Piece::King)));
        assert_eq!(state.board.piece_at(h2), Some((Player::White, Piece::Pawn)));
        assert_eq!(state.board.piece_at(h7), Some((Player::Black, Piece::Pawn)));
        assert_eq!(state.en_passant, Some(e3));
        assert_eq!(
            state.castling,
            Castles {
                white: CastleAbility {
                    kingside: true,
                    queenside: false,
                },
                black: CastleAbility {
                    kingside: false,
                    queenside: true,
                }
            }
        )
    }

    #[test]
    fn test_parse_invalid_fen() {
        let input = "r1b1kb1r/pppppppp/8/8/4P3/8/PPPP1PPP/R1B1KB1R/8 b";
        let state_res = fen(input);
        assert!(state_res.is_err(), "recognizes invalid fen");
    }
}
