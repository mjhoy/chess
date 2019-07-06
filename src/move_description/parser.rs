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

fn pos(input: &str) -> IResult<&str, Pos> {
    let (input, file) = file(input)?;
    let (input, rank) = rank(input)?;
    Ok((input, Pos { file, rank }))
}

/// Parses a movement description.
pub fn move_description(input: &str) -> IResult<&str, MoveDescription> {
    let (input, src_piece) = piece(input)?;
    let (input, dst_pos) = pos(input)?;
    Ok((input, MoveDescription { src_piece, dst_pos }))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::pos::*;
    use nom::{error::ErrorKind, error_position, Err};

    #[test]
    fn parse_piece() {
        assert_eq!(piece("Ke4"), Ok(("e4", Piece::King)));
        assert_eq!(piece("e4"), Ok(("e4", Piece::Pawn)));
    }

    #[test]
    fn parse_rank() {
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
    fn parse_file() {
        assert_eq!(file("e2"), Ok(("2", 4)));
        assert_eq!(
            file("i2"),
            Err(Err::Error(error_position!("i2", ErrorKind::Tag)))
        );
    }

    #[test]
    fn parse_pos() {
        assert_eq!(pos("e2"), Ok(("", e2)));
        assert_eq!(pos("a1"), Ok(("", a1)));
        assert_eq!(pos("h7"), Ok(("", h7)));
    }

    #[test]
    fn parse_move_description() {
        assert_eq!(
            move_description("Ke2"),
            Ok((
                "",
                MoveDescription {
                    src_piece: Piece::King,
                    dst_pos: e2
                }
            ))
        );
        assert_eq!(
            move_description("a1"),
            Ok((
                "",
                MoveDescription {
                    src_piece: Piece::Pawn,
                    dst_pos: a1
                }
            ))
        );
    }
}
