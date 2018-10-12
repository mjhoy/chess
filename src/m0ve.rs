use game::Game;
use pos::Pos;

pub struct Move {
    pub index: (Pos, Pos),
    pub next: Game,
}
