use colored::Colorize;
use rand::Rng;
use std::process;

fn main() {
    loop {
        let hardness: u8 = get_hardness();
        let words = word_list(hardness);

        let word = &words[rand::thread_rng().gen_range(0..words.len())];

        let mut guessed = vec!['-'; word.len()];
        let mut wrong_guesses = 0;
        let mut correct_guesses = 0;
        let mut letters_guessed = Vec::new();
        loop {
            print!("{}[2J", 27 as char);
            println!("The word so far is: {}", guessed.iter().collect::<String>());
            println!(
                "You have guessed the following letters: {}",
                letters_guessed
                    .iter()
                    .map(|&c: &char| c.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
            );
            println!("You have {} wrong guesses left", 6 - wrong_guesses);
            let guess = get_guess(letters_guessed.clone());
            letters_guessed.push(guess);
            let mut found = false;
            for (i, c) in word.chars().enumerate() {
                if c == guess {
                    guessed[i] = c;
                    found = true;
                    correct_guesses += 1;
                }
            }
            if !found {
                wrong_guesses += 1;
            }
            if wrong_guesses == 100 {
                println!("You lost! The word was: {}", word);
                break;
            }
            if correct_guesses == word.len() {
                println!("You won! The word was: {}", word);
                break;
            }
        }

        let mut input = String::new();
        println!("Would you like to play again? (y/n)");
        std::io::stdin().read_line(&mut input).unwrap();
        input = input.trim().to_string();
        if input == "n" {
            println!("Goodbye!");
            process::exit(0);
        }
    }
}

fn get_guess(letters_guessed: Vec<char>) -> char {
    let mut input;
    loop {
        input = String::new();
        println!("{}", "Please guess a letter: ".blue().bold());
        std::io::stdin().read_line(&mut input).unwrap();
        input = input.trim().to_string();
        if input.len() != 1 {
            println!("{}", "Please enter a single letter".bold().red());
            continue;
        }
        let guess = input.chars().next().unwrap();
        if letters_guessed.contains(&guess) {
            println!("{}", "You have already guessed that letter".bold().red());
            continue;
        }
        return guess;
    }
}

fn get_hardness() -> u8 {
    let mut input = String::new();
    loop {
        println!("{}", "Welcome to Hangman!".green().bold());
        println!("1. {}", "Normal mode".bright_blue());
        println!("2. {}", "Hard mode".bright_purple());
        println!("3. {}", "Quit".bright_red().bold());
        println!("Please select a mode: ");
        std::io::stdin().read_line(&mut input).unwrap();
        input = input.trim().to_string();
        match input.as_str() {
            "" => {
                return 0;
            }
            "1" => {
                return 0;
            }
            "2" => {
                return 1;
            }

            "3" => {
                println!("Goodbye!");
                process::exit(0);
            }
            _ => {
                println!("Invalid input, please try again");
            }
        }
    }
}

fn word_list(hardness: u8) -> Vec<String> {
    let hard_bytes = include_bytes!("../data/hard_words.txt");
    let normal_bytes = include_bytes!("../data/normal_words.txt");
    let words;
    if hardness == 0 {
        words = String::from_utf8_lossy(normal_bytes).to_string();
    } else {
        words = String::from_utf8_lossy(hard_bytes).to_string();
    }

    words
        .split("\n")
        .map(|e| e.to_string())
        .collect::<Vec<String>>()
}
