extern crate ansi_term;
extern crate rand;

use std::io;

// use rand::{thread_rng,};
use rand::seq::IteratorRandom;

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

// use ansi_term::Colour::Red;
// use ansi_term::Colour::Green;
// use ansi_term::Colour::Yellow;

use ansi_term::Colour;

enum PlayerGuess {
    Right,
    Wrong,
}

struct PlayerStatus {
    lives: u8,
    guessed_chars: Vec<char>,
    wrong_guessed_chars: Vec<char>,
    random_sentence: String,
    guess_chars_len: usize,
}

fn main() {
    // initializing the values for the game
    let guessed_chars = vec![];
    let wrong_guessed_chars = vec![];
    let lives = 5;

    // getting a random string from the file lines
    let random_sentence = pick_rand_sentence();

    let random_sentence_clone = random_sentence.clone();
    let mut random_sentence_clone = random_sentence_clone
        .replace(" ", "")
        .chars()
        .collect::<Vec<char>>();
    random_sentence_clone.sort();
    random_sentence_clone.dedup();
    let guess_chars_len = random_sentence_clone.len();

    let mut player_status = PlayerStatus {
        lives,
        guessed_chars,
        wrong_guessed_chars,
        random_sentence,
        guess_chars_len,
    };

    loop {
        update_screen(&player_status);

        // waiting for the user to insert input
        // and read it .
        let user_input = read_user_input();

        let user_input_result: PlayerGuess = check_user_input(
            user_input,
            &player_status.guessed_chars,
            &player_status.random_sentence,
        );

        let can_continue: bool =
            update_player_status(&mut player_status, user_input_result, user_input);

        // if can't continue == true break the loop
        if !can_continue {
            break;
        }
    }
}

/// This function is used to pick a random line from the input file
/// It returns a Optional<String> because of the possibility of
/// having an unexpected error .
fn pick_rand_sentence() -> String {
    // file name to read from .
    const FILENAME: &str = "input.txt";

    // opening the file .
    // if any error is encountered then None value will
    // be returned otherwise the value it self will be assigned
    // to the file variable .
    let file = File::open(&FILENAME).unwrap();

    // with handling the above error
    // we are not expecting any errors with new function
    let file = BufReader::new(&file);

    // let mut rng = thread_rng();

    let mut rng = rand::thread_rng();

    // let lines_iter = file.lines().map(|l| l.unwrap());

    let sample = file.lines().choose(&mut rng).unwrap();

    let lower_case_sample = match sample {
        Ok(sample) => sample.to_lowercase(),
        Err(e) => panic!("{}", e),
    };

    lower_case_sample

    // let lower_case_sample = sample.

    // let sample = seq::sample_iter(&mut rng, file.lines(), 5).unwrap();
    // let rand_line: String = thread_rng()
    // .sample_iter(file.lines().)
    // .take(1)
    // .map(char::from)
    // .collect();
}

fn update_screen(player_status: &PlayerStatus) {
    // TODO clear the command line to keep it clean
    clear();

    println!("");

    println!("{}", Colour::Red.paint("\t\t\t\t\t>> Hangman Game <<"));

    println!("");

    println!(
        "{}  {} {} ",
        Colour::Green.paint("Your lives :"),
        &player_status.lives.to_string(),
        Colour::Red.paint("❤")
    );

    // let foo: = &player_status.guessed_chars.clone().into_iter().collect::<String>();
    println!(
        "{} {:?} ",
        Colour::Green.paint("Your guessed chars :"),
        &player_status.guessed_chars
    );
    

    println!(
        "{} {:?} ",
        Colour::Green.paint("Your wrong  guesses :"),
        &player_status.wrong_guessed_chars
    );

    // println!("{}", &player_status.random_sentence);

    format_sentence(&player_status.random_sentence, &player_status.guessed_chars);
    let x = String::from("hello");
    print_sticker_man(&player_status.lives)
}

fn format_sentence(sentence: &str, guessed_chars: &Vec<char>) {
    // let re = Regex::new(r"^[A-Za-z]+$").unwrap();
    // let after = re.replace_all(sentence, "_");
    // after
    let mut final_sentence = String::from(sentence);
    for char in sentence.chars() {
        if !(guessed_chars.contains(&char)) && char != '_' && char != ' ' {
            final_sentence = final_sentence.replace(char, "_");
        }
    }

    println!("{} {}",Colour::Green.paint("Secret sentence :"), final_sentence);
}

#[allow(unused_must_use)]
fn read_user_input() -> char {
    let mut input: String = String::new();

    io::stdin().read_line(&mut input);

    let input = input.trim();
    let chr: char = validate_input(&input);

    chr
}

fn validate_input(input: &str) -> char {
    // alphabetic check

    match input.chars().all(|c| -> bool { c.is_alphabetic() }) {
        false => panic!("The input is not alphabetic"),
        true => true,
    };

    // getting the first element
    let chr: char = match input.chars().nth(0) {
        Some(chr) => chr,
        None => panic!("The input seems to be empty"),
    };

    chr

}

fn check_user_input(chr: char, guessed_chars: &Vec<char>, random_sentence: &str) -> PlayerGuess {
    if random_sentence.contains(chr) && !(guessed_chars.contains(&chr)) {
        PlayerGuess::Right
    } else {
        PlayerGuess::Wrong
    }
}

fn update_player_status(
    player_status: &mut PlayerStatus,
    player_guess: PlayerGuess,
    user_input: char,
) -> bool {
    match player_guess {
        PlayerGuess::Right => {
            player_status.guessed_chars.push(user_input);

            if player_status.guess_chars_len == player_status.guessed_chars.len() {
                // The game has a winner
                println!("{}", Colour::Green.paint("You won !"));
                println!("{}", Colour::Yellow.paint(&player_status.random_sentence));
                // break the loop
                false
            } else {
                // continue the game
                true
            }
        }
        PlayerGuess::Wrong => {
            player_status.wrong_guessed_chars.push(user_input);

            player_status.lives = player_status.lives - 1;

            if player_status.lives == 0 {
                // The game has a loser
                println!("{}", Colour::Red.paint("You lost !"));
                println!("{}", Colour::Yellow.paint(&player_status.random_sentence));
                // break the loop
                false
            } else {
                // continue the game
                true
            }
        } // It's either Right , Wrong so there is no need for default case
    }
}

fn print_sticker_man(lives: &u8) {
    match lives {
        0 => {
            println!(" _________   ");
            println!("|         |  ");
            println!("|         XO ");
            println!("|        /|\\ ");
            println!("|        / \\ ");
            println!("|            ");
            println!("|            ");
        }

        1 => {
            println!(" _________   ");
            println!("|         |  ");
            println!("|         O  ");
            println!("|        /|\\ ");
            println!("|        / \\ ");
            println!("|        ||| ");
            println!("|        ||| ");
        }

        2 => {
            println!(" _________   ");
            println!("|            ");
            println!("|         O  ");
            println!("|        /|\\ ");
            println!("|        / \\ ");
            println!("|        ||| ");
            println!("|        ||| ");
        }

        3 => {
            println!(" _________   ");
            println!("|            ");
            println!("|            ");
            println!("|         O  ");
            println!("|        /|\\ ");
            println!("|        / \\ ");
            println!("|        ||| ");
        }

        4 => {
            println!(" _________   ");
            println!("|            ");
            println!("|            ");
            println!("|            ");
            println!("|         O  ");
            println!("|        /|\\ ");
            println!("|        / \\ ");
        }

        5 => {
            println!("             ");
            println!("             ");
            println!("             ");
            println!("             ");
            println!("          O  ");
            println!("         /|\\ ");
            println!("         / \\ ");
        }

        // because the u8 includes the range between 2^8 = 256
        // and not all the cases are not handled so
        // there should be a default case .
        _ => {
            println!("             ");
            println!("             ");
            println!("             ");
            println!("             ");
            println!("          O  ");
            println!("         /|\\ ");
            println!("         / \\ ");
        }
    }
}

fn clear() {
    print!("\x1b[3J");
    // let output = Command::new("clear")
    // .status()
    // .unwrap();
    //   println!("{}", String::from_utf8_lossy(&output.stdout));
}
