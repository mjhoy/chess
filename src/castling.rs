#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CastleAbility {
    pub king: bool,
    pub queen: bool,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Castling {
    pub white: CastleAbility,
    pub black: CastleAbility,
}

impl Castling {
    pub fn initial() -> Self {
        Castling {
            white: CastleAbility {
                king: true,
                queen: true,
            },
            black: CastleAbility {
                king: true,
                queen: true,
            },
        }
    }
}
