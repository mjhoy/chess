use crate::{piece::Piece, player::Player};

pub type Square = Option<(Player, Piece)>;
