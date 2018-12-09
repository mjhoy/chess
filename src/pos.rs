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
