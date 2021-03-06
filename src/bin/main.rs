use std::io;

use chess::game::state::State;
use chess::game::Game;
use chess::parsing;
use clap::{App, Arg, SubCommand};

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
    let new_state = play_moves(game.state, matches.value_of("moves"));
    game = Game { state: new_state };

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
            let result = parsing::parse_fen(fen_str);
            if let Ok(state) = result {
                Game::with_state(state)
            } else {
                panic!("Couldn't parse fen: {:?}", result);
            }
        }
    }
}

fn play_moves(mut state: State, moves: Option<&str>) -> State {
    match moves {
        None => (),
        Some(moves_str) => {
            let res = parsing::parse_algebraic_notation_multiple(moves_str);
            match res {
                Ok(move_descriptions) => {
                    for move_description in move_descriptions {
                        let game_moves = state.gen_moves();
                        match move_description.match_moves(game_moves) {
                            Some(m0ve) => {
                                state = m0ve.next;
                            }
                            None => panic!("Error making move {:?}", move_description),
                        }
                    }
                }
                _ => {
                    panic!("couldn't parse move string {}", moves_str);
                }
            }
        }
    }
    state
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

        match parsing::parse_algebraic_notation(buf.trim()) {
            Ok(move_description) => match move_description.match_moves(moves) {
                Some(m0ve) => {
                    game = Game { state: m0ve.next };
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
