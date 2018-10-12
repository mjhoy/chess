use board::Board;
use player::Player;

#[derive(Debug, Clone, Copy)]
pub struct State {
    pub board: Board,
    pub player: Player,
}
