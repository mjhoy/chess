use std::io;

extern crate chess;
use chess::{board_str, gen_moves, move_str, new_game, player_str};

fn main() {
    let mut game = new_game();
    let ended: &mut bool = &mut false;
    let mut buf = String::new();

    while !*ended {
        println!("\n{}", board_str(&game));

        let moves = gen_moves(&game.state);

        if moves.is_empty() {
            println!("Game over! RET quits.");
            io::stdin().read_line(&mut buf).unwrap();
            *ended = true;
            break;
        }

        println!("{}'s move.", player_str(game.state.player));
        for (i, m0ve) in moves.iter().enumerate() {
            println!("{}: {}", i + 1, move_str(&m0ve));
        }

        println!("Enter 1..{} ('q' quits)", moves.len());

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
                    println!("Please enter a number between 1 and {}.", moves.len());
                }
            }
            Err(_) => {
                println!("Please enter a number between 1 and {}.", moves.len());
            }
        }

        buf = String::new();
    }
}
