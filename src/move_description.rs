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
            Ok((_, md)) => Ok(md),
            Err(e) => Err(format!("parsing error: {:?}", e)),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_move_description() {
        assert_eq!(
            MoveDescription::parse("Ke2"),
            Ok(MoveDescription {
                src_piece: Piece::King,
                dst_pos: Pos { file: 4, rank: 1 }
            })
        );
        assert_eq!(
            MoveDescription::parse("Ze2"),
            Err(r#"parsing error: Error(Code("Ze2", Alt))"#.to_string())
        );
    }
}
