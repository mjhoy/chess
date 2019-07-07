use std::io;

use chess::fen::fen;
use chess::game::Game;
use chess::move_description::MoveDescription;
use chess::new_game;
use clap::{App, Arg, SubCommand};

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let matches = App::new("chess")
        .version(VERSION)
        .version_short("v")
        .about("do some chess stuff on the command line")
        .arg(
            Arg::with_name("fen")
                .short("f")
                .long("fen")
                .value_name("FEN")
                .help("Position in Forsyth-Edwards notation"),
        )
        .subcommand(SubCommand::with_name("play").about("play a game"))
        .get_matches();

    let mut game;
    if let Some(fen_str) = matches.value_of("fen") {
        let result = fen(fen_str);
        if let Ok((_, state)) = result {
            game = Game::with_state(state);
        } else {
            panic!("Couldn't parse fen: {:?}", result);
        }
    } else {
        game = new_game();
    }

    if matches.subcommand_matches("play").is_some() {
        play(game);
    } else {
        println!("{}", game.state.board.str());
    }
}

fn play(mut game: Game) {
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
