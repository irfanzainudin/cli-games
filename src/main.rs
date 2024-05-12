use clap::Parser;
use rand::Rng;
use std::io::{self, BufRead};

const CAPITAL_A_ASCII_CODE: u8 = 65;
const CAPITAL_Z_ASCII_CODE: u8 = 90;
const VOWELS: [char; 5] = ['A', 'E', 'I', 'O', 'U'];
const CONSONANTS: [char; 21] = [
    'B', 'C', 'D', 'F', 'G', 'H', 'J', 'K', 'L', 'M', 'N', 'P', 'Q', 'R', 'S', 'T', 'V', 'W', 'X',
    'Y', 'Z',
];

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
    println!("");
    for (i, g) in games.iter().enumerate() {
        println!("{}. {}", i + 1, g);
    }
    println!("");
}

trait Vowel {
    fn is_vowel(&self) -> bool;
}

impl Vowel for char {
    fn is_vowel(&self) -> bool {
        if !self.is_alphabetic() {
            false
        } else {
            match self {
                'a' | 'e' | 'i' | 'o' | 'u' => true,
                _otherwise => false,
            }
        }
    }
}

trait Consonant {
    fn is_consonant(&self) -> bool;
}

impl Consonant for char {
    fn is_consonant(&self) -> bool {
        if self.is_vowel() {
            false
        } else {
            true
        }
    }
}

fn countdown_words() {
    println!("Welcome to Countdown Words!\n");

    let mut chosen_letters = String::new();
    for _ in 0..9 {
        println!("Please choose consonant or vowel (c/v): ");
        let user_input: Option<Result<String, std::io::Error>> = io::stdin().lock().lines().next();
        match user_input {
            None => println!("[FAILURE] failed to read consonant or vowel"),
            Some(uinput) => {
                match uinput {
                    Ok(corv) => {
                        let c_or_v = corv.as_str();
                        match c_or_v {
                            "c" => {
                                // Generate a new consonant
                                let num = rand::thread_rng().gen_range(0..CONSONANTS.len());
                                let st = CONSONANTS[num];
                                chosen_letters.push(st);
                                println!("Chosen Letters: {}", chosen_letters);
                            }
                            "v" => {
                                // Generate a new vowel
                                let num = rand::thread_rng().gen_range(0..VOWELS.len());
                                let st = VOWELS[num];
                                chosen_letters.push(st);
                                println!("Chosen Letters: {}", chosen_letters);
                            }
                            _otherwise => {
                                println!("[FAILURE] invalid input: \"{}\"... punishing/rewarding you with a random letter", c_or_v);

                                // Generate a new consonant
                                let num = rand::thread_rng()
                                    .gen_range(CAPITAL_A_ASCII_CODE..=CAPITAL_Z_ASCII_CODE);
                                let st = num as char;
                                chosen_letters.push(st);
                                println!("Chosen Letters: {}", chosen_letters);
                            }
                        }
                    }
                    Err(failed) => println!("[FAILURE] {}", failed),
                }
            }
        }
    }
}

fn countdown_numbers() {
    println!("Welcome to Countdown Numbers!\n");
}

fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name);
        println!("Please choose what games do you want to play: ");
        show_game_choices();
        // print!("Game: ");
        let user_input: Option<Result<String, std::io::Error>> = io::stdin().lock().lines().next();
        match user_input {
            None => println!("[FAILURE] failed to read user input"),
            Some(succeed_option) => match succeed_option {
                Ok(succeed) => {
                    // TODO: maybe needs a better implementation
                    const RADIX: u32 = 10;
                    if succeed.chars().all(|nums| nums.is_digit(RADIX)) {
                        // IMPROVEMENT: have an enum to show which game user chose
                        println!("[SUCCESS] read {} as chosen game", succeed);
                        println!("");
                        // IMPROVEMENT: change next() to an implementation which reads
                        // ------------ the whole number otherwise the list of games will
                        // ------------ only accommodate 0-9
                        let chosen_game = succeed.chars().next().unwrap_or('1');
                        match chosen_game.to_digit(RADIX) {
                            None => println!(
                                "[FAILURE] failed to convert user_input:char to user_input:u32"
                            ),
                            Some(chosen_game) => match chosen_game {
                                1 => countdown_words(),
                                2 => countdown_numbers(),
                                otherwise => {
                                    println!(
                                        "[FAILURE] no games associated with that number: {}",
                                        otherwise
                                    )
                                }
                            },
                        }
                    } else {
                        println!("[FAILURE] invalid input: {}", succeed);
                    }
                }
                Err(failed) => {
                    println!("[FAILURE] {}", failed);
                }
            },
        }
    }
}
