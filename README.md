# chess

do some chess stuff on the command line

## usage

    chess [OPTIONS] [SUBCOMMAND]

## options

    -i, --initial <initial>    Initial position in Forsyth-Edwards notation
    -m, --moves <moves>        Moves to play in algebraic chess notation

## subcommands

    help    Prints a help message
    play    play moves interactively

### examples

print the initial board:

    $ chess
      8♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜
      7♟ ♟ ♟ ♟ ♟ ♟ ♟ ♟
      6
      5
      4
      3
      2♙ ♙ ♙ ♙ ♙ ♙ ♙ ♙
      1♖ ♘ ♗ ♕ ♔ ♗ ♘ ♖
       A B C D E F G H

play knight to f3:

     $ chess -m Nf3
      8♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜
      7♟ ♟ ♟ ♟ ♟ ♟ ♟ ♟
      6
      5
      4
      3          ♘
      2♙ ♙ ♙ ♙ ♙ ♙ ♙ ♙
      1♖ ♘ ♗ ♕ ♔ ♗   ♖
       A B C D E F G H

start with a different position:

     $ chess -i "4r1k1/p4ppp/8/1p6/2bPN3/4B2P/PP3PP1/R3q1K1 w"
      8        ♜   ♚
      7♟         ♟ ♟ ♟
      6
      5  ♟
      4    ♝ ♙ ♘
      3        ♗     ♙
      2♙ ♙       ♙ ♙
      1♖       ♛   ♔
       A B C D E F G H

## development

Install Rust: https://www.rust-lang.org

     $ cargo build # compiles the code
     $ cargo run -- -m Nf3 # executes the binary

I also use `rustfmt` and `clippy`:

     $ rustup component add clippy
     $ rustup component add rustfmt

Then make sure run these commands before committing.
     
     $ cargo fmt
     $ cargo clippy
