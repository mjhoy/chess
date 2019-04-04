use std::io;

use chess::move_description::MoveDescription;
use chess::new_game;

fn main() {
    let mut game = new_game();
    let ended: &mut bool = &mut false;
    let mut buf = String::new();

    while !*ended {
        println!("\n{}", game.state.board.str());

        let moves = game.state.gen_moves();

        if moves.is_empty() {
            println!("Game over! RET quits.");
            io::stdin().read_line(&mut buf).unwrap();
            *ended = true;
            break;
        }

        println!("{}'s move.", game.state.player);

        println!("Please enter a move, or 'q' quits.");

        io::stdin().read_line(&mut buf).unwrap();

        if buf.trim() == "q" {
            *ended = true;
            break;
        }

        match MoveDescription::parse(buf.trim()) {
            Ok(move_description) => match move_description.match_moves(moves) {
                Some(m0ve) => {
                    game = m0ve.next;
                }
                None => println!("Can't make that move!"),
            },
            Err(_) => {
                println!("Sorry, that doesn't describe a move!");
            }
        }

        buf = String::new();
    }
}
