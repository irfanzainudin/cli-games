use clap::Parser;
use rand::Rng;
use spinoff::{spinners, Color, Spinner};
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead};
// use std::thread::sleep;
use std::time::Duration;
use tokio::time::sleep;

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

fn populate_dictionary<'a>(
    dictionary: &'a mut Vec<&'a str>,
    file: Result<File, std::io::Error>,
    contents: &'a mut String,
) {
    match file {
        Ok(mut opened_file) => {
            let _ = opened_file.read_to_string(contents);
            for word in contents.split_ascii_whitespace() {
                dictionary.push(word);
            }
        }
        Err(failed) => println!("[FAILURE] failed to open file: {}", failed),
    }
}

// Heap's Algorithm: https://en.wikipedia.org/wiki/Heap%27s_algorithm
pub async fn heaps_algorithm(n: usize, mut a: Vec<char>) {
    // c is an encoding of the stack state. c[k] encodes the for-loop counter for when generate(k - 1, A) is called
    let mut c: Vec<usize> = vec![0; n];

    // output(A)
    for ch in &a {
        print!("{}", ch);
    }
    print!("\n");

    // i acts similarly to a stack pointer
    let mut i = 1;
    while i < n {
        if c[i] < i {
            if i % 2 == 0 {
                a.swap(0, i);
            } else {
                a.swap(c[i], i);
            }

            // output(A)
            for ch in &a {
                print!("{}", ch);
            }
            print!("\n");

            // Swap has occurred ending the for-loop. Simulate the increment of the for-loop counter
            c[i] += 1;
            // Simulate recursive call reaching the base case by bringing the pointer to the base case analog in the array
            i = 1;
        } else {
            // Calling generate(i+1, A) has ended as the for-loop terminated. Reset the state and simulate popping the stack by incrementing the pointer.
            c[i] = 0;
            i += 1
        }
    }

    for ch in &a {
        print!("Last: {}", ch);
    }
    print!("\n");
}

pub async fn countdown_words() {
    println!("Welcome to Countdown Words!\n");

    println!("Populating dictionary...\n");
    let mut dictionary: Vec<&str> = Vec::new();
    let txt_file = File::open("src/dictionary.txt");
    let mut contents = String::new();
    populate_dictionary(&mut dictionary, txt_file, &mut contents);

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

    println!("\nYour time starts now!");
    let mut thinking = Spinner::new(spinners::Dots, "Thinking...", Color::Blue);
    let _ = sleep(Duration::from_secs(30));
    thinking.success("Pens down! 30s have elapsed\n");

    println!("What's your answer?");
    let user_input: Option<Result<String, std::io::Error>> = io::stdin().lock().lines().next();
    match user_input {
        None => println!("[FAILURE] failed to read user input"),
        Some(succeed_option) => match succeed_option {
            Ok(succeed) => {
                println!("\nYour answer is: {}\n", succeed);
                println!("Congrats! You get {} points\n", succeed.len());
            }
            Err(failed) => {
                println!("[FAILURE] failed_option: {}", failed);
            }
        },
    }

    println!("Now...\n");

    println!("Can you do better?");
    println!("Let's ask Susie Rust\n");

    let mut susie = Spinner::new(spinners::Dots, "Checking...", Color::Blue);
    let _ = sleep(Duration::from_secs(5));
    susie.success("You got the highest possible length\n");
}

pub async fn countdown_numbers() {
    println!("Welcome to Countdown Numbers!\n");

    println!("Your time starts now!");
    let mut spinner = Spinner::new(spinners::Dots, "Thinking...", Color::Blue);
    let _ = sleep(Duration::from_secs(30));
    spinner.success("Pens down! 30s have elapsed\n");

    println!("What's your answer?");
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name);
        println!("Please choose what games do you want to play: ");
        show_game_choices();

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
                                1 => countdown_words().await,
                                2 => countdown_numbers().await,
                                3 => {
                                    for _ in 0..9 {
                                        heaps_algorithm(
                                            9,
                                            vec!['P', 'A', 'N', 'U', 'S', 'E', 'F', 'U', 'J'],
                                        )
                                        .await
                                    }
                                }
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
                    println!("[FAILURE] failed_option: {}", failed);
                }
            },
        }
    }
}
