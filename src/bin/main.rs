use std::io;

use chess;
use chess::an;
use chess::fen;
use chess::game::Game;
use clap::{App, Arg, SubCommand};
use nom::bytes::complete::tag;
use nom::multi::separated_list;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let matches = App::new("chess")
        .version(VERSION)
        .version_short("v")
        .about("do some chess stuff on the command line")
        .arg(
            Arg::with_name("initial")
                .short("i")
                .long("initial")
                .takes_value(true)
                .help("Initial position in Forsyth-Edwards notation"),
        )
        .arg(
            Arg::with_name("moves")
                .short("m")
                .long("moves")
                .takes_value(true)
                .help("Moves to play in algebraic chess notation"),
        )
        .subcommand(SubCommand::with_name("play").about("play moves interactively"))
        .get_matches();

    let mut game = setup_game(matches.value_of("initial"));
    game = play_moves(game, matches.value_of("moves"));

    if matches.subcommand_matches("play").is_some() {
        play(game);
    } else {
        println!("{}", game.state.board.str());
    }
}

fn setup_game(initial_fen: Option<&str>) -> Game {
    match initial_fen {
        None => chess::new_game(),
        Some(fen_str) => {
            let result = fen::fen(fen_str);
            if let Ok((_, state)) = result {
                Game::with_state(state)
            } else {
                panic!("Couldn't parse fen: {:?}", result);
            }
        }
    }
}

fn play_moves(mut game: Game, moves: Option<&str>) -> Game {
    match moves {
        None => (),
        Some(moves_str) => {
            let res = separated_list(tag(" "), an::an)(moves_str);
            match res {
                Ok((_, move_descriptions)) => {
                    for move_description in move_descriptions {
                        let game_moves = game.state.gen_moves();
                        match move_description.match_moves(game_moves) {
                            Some(m0ve) => {
                                game = m0ve.next;
                            }
                            None => panic!("Error making move {:?}", move_description),
                        }
                    }
                }
                Err(_) => {
                    panic!("couldn't parse move string {}", moves_str);
                }
            }
        }
    }
    game
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

        match an::parse_an(buf.trim()) {
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
