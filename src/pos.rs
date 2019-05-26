#![allow(non_upper_case_globals)]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pos {
    pub rank: u8,
    pub file: u8,
}

impl Pos {
    pub fn to_offset(self, nsize: u8) -> usize {
        let rank_offset = (self.rank * nsize) as usize;
        rank_offset + (self.file as usize)
    }
}

pub const a1: Pos = Pos { rank: 0, file: 0 };
pub const a2: Pos = Pos { rank: 1, file: 0 };
pub const a3: Pos = Pos { rank: 2, file: 0 };
pub const a4: Pos = Pos { rank: 3, file: 0 };
pub const a5: Pos = Pos { rank: 4, file: 0 };
pub const a6: Pos = Pos { rank: 5, file: 0 };
pub const a7: Pos = Pos { rank: 6, file: 0 };
pub const a8: Pos = Pos { rank: 7, file: 0 };

pub const b1: Pos = Pos { rank: 0, file: 1 };
pub const b2: Pos = Pos { rank: 1, file: 1 };
pub const b3: Pos = Pos { rank: 2, file: 1 };
pub const b4: Pos = Pos { rank: 3, file: 1 };
pub const b5: Pos = Pos { rank: 4, file: 1 };
pub const b6: Pos = Pos { rank: 5, file: 1 };
pub const b7: Pos = Pos { rank: 6, file: 1 };
pub const b8: Pos = Pos { rank: 7, file: 1 };

pub const c1: Pos = Pos { rank: 0, file: 2 };
pub const c2: Pos = Pos { rank: 1, file: 2 };
pub const c3: Pos = Pos { rank: 2, file: 2 };
pub const c4: Pos = Pos { rank: 3, file: 2 };
pub const c5: Pos = Pos { rank: 4, file: 2 };
pub const c6: Pos = Pos { rank: 5, file: 2 };
pub const c7: Pos = Pos { rank: 6, file: 2 };
pub const c8: Pos = Pos { rank: 7, file: 2 };

pub const d1: Pos = Pos { rank: 0, file: 3 };
pub const d2: Pos = Pos { rank: 1, file: 3 };
pub const d3: Pos = Pos { rank: 2, file: 3 };
pub const d4: Pos = Pos { rank: 3, file: 3 };
pub const d5: Pos = Pos { rank: 4, file: 3 };
pub const d6: Pos = Pos { rank: 5, file: 3 };
pub const d7: Pos = Pos { rank: 6, file: 3 };
pub const d8: Pos = Pos { rank: 7, file: 3 };

pub const e1: Pos = Pos { rank: 0, file: 4 };
pub const e2: Pos = Pos { rank: 1, file: 4 };
pub const e3: Pos = Pos { rank: 2, file: 4 };
pub const e4: Pos = Pos { rank: 3, file: 4 };
pub const e5: Pos = Pos { rank: 4, file: 4 };
pub const e6: Pos = Pos { rank: 5, file: 4 };
pub const e7: Pos = Pos { rank: 6, file: 4 };
pub const e8: Pos = Pos { rank: 7, file: 4 };

pub const f1: Pos = Pos { rank: 0, file: 5 };
pub const f2: Pos = Pos { rank: 1, file: 5 };
pub const f3: Pos = Pos { rank: 2, file: 5 };
pub const f4: Pos = Pos { rank: 3, file: 5 };
pub const f5: Pos = Pos { rank: 4, file: 5 };
pub const f6: Pos = Pos { rank: 5, file: 5 };
pub const f7: Pos = Pos { rank: 6, file: 5 };
pub const f8: Pos = Pos { rank: 7, file: 5 };

pub const g1: Pos = Pos { rank: 0, file: 6 };
pub const g2: Pos = Pos { rank: 1, file: 6 };
pub const g3: Pos = Pos { rank: 2, file: 6 };
pub const g4: Pos = Pos { rank: 3, file: 6 };
pub const g5: Pos = Pos { rank: 4, file: 6 };
pub const g6: Pos = Pos { rank: 5, file: 6 };
pub const g7: Pos = Pos { rank: 6, file: 6 };
pub const g8: Pos = Pos { rank: 7, file: 6 };

pub const h1: Pos = Pos { rank: 0, file: 7 };
pub const h2: Pos = Pos { rank: 1, file: 7 };
pub const h3: Pos = Pos { rank: 2, file: 7 };
pub const h4: Pos = Pos { rank: 3, file: 7 };
pub const h5: Pos = Pos { rank: 4, file: 7 };
pub const h6: Pos = Pos { rank: 5, file: 7 };
pub const h7: Pos = Pos { rank: 6, file: 7 };
pub const h8: Pos = Pos { rank: 7, file: 7 };
