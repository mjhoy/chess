extern crate nalgebra as na;
#[macro_use]
extern crate itertools;

pub mod player;
use player::Player;
use player::Player::*;

pub mod piece;
use piece::Piece;
use Piece::*;

pub mod square;
use square::Square;

pub mod pos;
use pos::Pos;

pub mod board;
use board::Board;

pub mod state;
use state::State;

pub mod game;
use game::Game;

pub mod m0ve;
use m0ve::Move;

/// Initial game.
pub fn new_game() -> Game {
    let board = Board::initial();
    let player = White;
    let state = State { board, player };
    Game { state }
}

/// Pretty print a move.
pub fn move_str(m0ve: &Move) -> String {
    let (from, to) = m0ve.index;
    let from_file = (from.file + b'A') as char;
    let from_rank = from.rank + 1;
    let to_file = (to.file + b'A') as char;
    let to_rank = to.rank + 1;
    format!("{}{} -> {}{}", from_file, from_rank, to_file, to_rank)
}

/// Pretty print a player.
pub fn player_str(player: Player) -> &'static str {
    match player {
        White => "White",
        Black => "Black",
    }
}

#[cfg(test)]
mod test {
    use *;

    #[test]
    fn test_new_game_starts_white() {
        let game = new_game();
        assert_eq!(game.state.player, White);
    }
}
