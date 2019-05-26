use crate::move_description::MoveDescription;
use crate::piece::Piece;
use crate::pos::Pos;
use nom::{alt, do_parse, named, tag, value};

named!(
    piece<&str, Piece>,
    alt!(
        value!(Piece::King, tag!("K")) |
        value!(Piece::Rook, tag!("R")) |
        value!(Piece::Pawn, tag!(""))
    )
);

named!(
    rank<&str, u8>,
    alt!(
        value!(0, tag!("1")) |
        value!(1, tag!("2")) |
        value!(2, tag!("3")) |
        value!(3, tag!("4")) |
        value!(4, tag!("5")) |
        value!(5, tag!("6")) |
        value!(6, tag!("7")) |
        value!(7, tag!("8"))
    )
);

named!(
    file<&str, u8>,
    alt!(
        value!(0, tag!("a")) |
        value!(1, tag!("b")) |
        value!(2, tag!("c")) |
        value!(3, tag!("d")) |
        value!(4, tag!("e")) |
        value!(5, tag!("f")) |
        value!(6, tag!("g")) |
        value!(7, tag!("h"))
    )
);

named!(
    pos<&str, Pos>,
    do_parse!(
        file: file >>
        rank: rank >>
        (Pos { file, rank })
    )
);

named!(
    #[doc="
        Parses a movement description.
    "],
    pub move_description<&str, MoveDescription>,
    do_parse!(
        src_piece: piece >>
        dst_pos: pos >>
        (MoveDescription { src_piece, dst_pos })
    )
);

#[cfg(test)]
mod test {
    use super::*;
    use crate::pos::*;
    use nom::Context::Code;
    use nom::Err::Error;
    use nom::ErrorKind;

    #[test]
    fn parse_piece() {
        assert_eq!(piece("Ke4"), Ok(("e4", Piece::King)));
        assert_eq!(piece("e4"), Ok(("e4", Piece::Pawn)));
    }

    #[test]
    fn parse_rank() {
        assert_eq!(rank("4e2"), Ok(("e2", 3)));
        assert_eq!(rank("41e2"), Ok(("1e2", 3)));
        assert_eq!(rank("0e2"), Err(Error(Code("0e2", ErrorKind::Alt))));
        assert_eq!(rank("9e2"), Err(Error(Code("9e2", ErrorKind::Alt))));
    }

    #[test]
    fn parse_file() {
        assert_eq!(file("e2"), Ok(("2", 4)));
        assert_eq!(file("i2"), Err(Error(Code("i2", ErrorKind::Alt))));
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
