use crate::castling::Castleside;
use crate::move_description::MoveDescription;
use crate::piece::Piece;
use crate::pos::Pos;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::value;
use nom::IResult;

fn piece(input: &str) -> IResult<&str, Piece> {
    alt((
        value(Piece::Bishop, tag("B")),
        value(Piece::King, tag("K")),
        value(Piece::Rook, tag("R")),
        value(Piece::Queen, tag("Q")),
        value(Piece::Knight, tag("N")),
        value(Piece::Pawn, tag("")),
    ))(input)
}

fn rank(input: &str) -> IResult<&str, u8> {
    alt((
        value(0, tag("1")),
        value(1, tag("2")),
        value(2, tag("3")),
        value(3, tag("4")),
        value(4, tag("5")),
        value(5, tag("6")),
        value(6, tag("7")),
        value(7, tag("8")),
    ))(input)
}

fn file(input: &str) -> IResult<&str, u8> {
    alt((
        value(0, tag("a")),
        value(1, tag("b")),
        value(2, tag("c")),
        value(3, tag("d")),
        value(4, tag("e")),
        value(5, tag("f")),
        value(6, tag("g")),
        value(7, tag("h")),
    ))(input)
}

pub fn pos(input: &str) -> IResult<&str, Pos> {
    let (input, file) = file(input)?;
    let (input, rank) = rank(input)?;
    Ok((input, Pos { file, rank }))
}

pub fn simple(input: &str) -> IResult<&str, MoveDescription> {
    let (input, src_piece) = piece(input)?;
    let (input, dst_pos) = pos(input)?;
    Ok((input, MoveDescription::Simple { src_piece, dst_pos }))
}

fn castle(input: &str) -> IResult<&str, MoveDescription> {
    alt((
        value(
            MoveDescription::Castle {
                castleside: Castleside::Queenside,
            },
            tag("O-O-O"),
        ),
        value(
            MoveDescription::Castle {
                castleside: Castleside::Kingside,
            },
            tag("O-O"),
        ),
    ))(input)
}

pub fn algebraic_notation(input: &str) -> IResult<&str, MoveDescription> {
    alt((simple, castle))(input)
}

/// Parses a movement description from algebraic notation.
pub fn parse_algebraic_notation(input: &str) -> Result<MoveDescription, String> {
    match algebraic_notation(input) {
        Ok((ref rem, ref _md)) if !rem.is_empty() => {
            Err("parsing error: extra characters".to_string())
        }
        Ok((_remaining, md)) => Ok(md),
        Err(e) => Err(format!("parsing error: {:?}", e)),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::pos::*;
    use nom::{error::ErrorKind, error_position, Err};

    #[test]
    fn test_piece() {
        assert_eq!(piece("Ke4"), Ok(("e4", Piece::King)));
        assert_eq!(piece("e4"), Ok(("e4", Piece::Pawn)));
    }

    #[test]
    fn test_rank() {
        assert_eq!(rank("4e2"), Ok(("e2", 3)));
        assert_eq!(rank("41e2"), Ok(("1e2", 3)));
        assert_eq!(
            rank("0e2"),
            Err(Err::Error(error_position!("0e2", ErrorKind::Tag)))
        );
        assert_eq!(
            rank("9e2"),
            Err(Err::Error(error_position!("9e2", ErrorKind::Tag)))
        );
    }

    #[test]
    fn test_file() {
        assert_eq!(file("e2"), Ok(("2", 4)));
        assert_eq!(
            file("i2"),
            Err(Err::Error(error_position!("i2", ErrorKind::Tag)))
        );
    }

    #[test]
    fn test_pos() {
        assert_eq!(pos("e2"), Ok(("", e2)));
        assert_eq!(pos("a1"), Ok(("", a1)));
        assert_eq!(pos("h7"), Ok(("", h7)));
    }

    #[test]
    fn test_algebraic_notation() {
        assert_eq!(
            algebraic_notation("Ke2"),
            Ok((
                "",
                MoveDescription::Simple {
                    src_piece: Piece::King,
                    dst_pos: e2
                }
            ))
        );
        assert_eq!(
            algebraic_notation("a1"),
            Ok((
                "",
                MoveDescription::Simple {
                    src_piece: Piece::Pawn,
                    dst_pos: a1
                }
            ))
        );
    }

    #[test]
    fn test_parse_algebraic_notation() {
        assert_eq!(
            parse_algebraic_notation("Ke2"),
            Ok(MoveDescription::Simple {
                src_piece: Piece::King,
                dst_pos: e2,
            })
        );
        assert_eq!(
            parse_algebraic_notation("O-O"),
            Ok(MoveDescription::Castle {
                castleside: Castleside::Kingside
            })
        );
        assert_eq!(
            parse_algebraic_notation("O-O-O"),
            Ok(MoveDescription::Castle {
                castleside: Castleside::Queenside
            })
        );
        assert_eq!(
            parse_algebraic_notation("Ze2"),
            Err(r#"parsing error: Error(("Ze2", Tag))"#.to_string())
        );
        assert_eq!(
            parse_algebraic_notation("Ke2junk"),
            Err(r#"parsing error: extra characters"#.to_string())
        );
    }
}
