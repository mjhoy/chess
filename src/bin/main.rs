use std::io;

extern crate chess;
use chess::{new_game, board_str, gen_moves, player_str, move_str};

fn main() {
    let mut game = new_game();
    let ended: &mut bool = &mut false;
    let mut buf = String::new();

    while !*ended {
        println!("\n{}", board_str(&game));

        let moves = gen_moves(game.state);

        if moves.len() == 0 {
            println!("Game over! RET quits.");
            io::stdin().read_line(&mut buf).unwrap();
            *ended = true;
            break;
        }

        println!("{}'s move.", player_str(game.state.player));
        for i in 0..moves.len() {
            let m0ve = &moves[i];
            println!("{}: {}", i+1, move_str(&m0ve));
        }

        println!("Enter {}..{} ('q' quits)", 1, moves.len());

        io::stdin().read_line(&mut buf).unwrap();

        if buf.trim() == "q" {
            *ended = true;
            break;
        }

        match buf.trim().parse::<usize>() {
            Ok(i) => {
                if i <= moves.len() && i > 0 {
                    game = moves.into_iter().nth(i - 1).expect("checked index").next;
                } else {
                    println!("Please enter a number between {} and {}.", 1, moves.len());
                }
            }
            Err(_) => {
                println!("Please enter a number between {} and {}.", 1, moves.len());
            }
        }

        buf = String::new();
    }
}
