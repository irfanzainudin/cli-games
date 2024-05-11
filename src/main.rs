use clap::Parser;
use std::io::{self, BufRead};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn show_game_choices() {
    // let games: Vec<&str> = vec!["Tic Tac Toe", "Chess", "Pacman", "Countdown Words"];
    let games: Vec<&str> = vec!["Countdown Words", "Countdown Numbers"];
    for (i, g) in games.iter().enumerate() {
        println!("{}. {}", i + 1, g);
    }
}

fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name);
        println!("Please choose what games do you want to play: ");
        show_game_choices();
        let user_input: Option<Result<String, std::io::Error>> = io::stdin().lock().lines().next();
        match user_input {
            None => {}
            Some(succeed_option) => match succeed_option {
                Ok(succeed) => {
                    println!("[SUCCESS] {}", succeed);
                }
                Err(failed) => {
                    println!("[FAILURE] {}", failed);
                }
            },
        }
        // TODO: need to convert the input to numbers and match that to the game chosen by user
    }
}
