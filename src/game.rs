use crate::word_utils::{load_words, pick_random_word, scramble_word, is_valid_guess};
use std::collections::HashSet;
use std::io;

pub fn run_game() -> io::Result<()> {
    let six_letter_words = load_words("six_letter_words.txt")?;
    let word_bank = load_words("word_bank.txt")?;

    if let Some(secret_word) = pick_random_word(&six_letter_words) {
        let scrambled = scramble_word(&secret_word);
        println!("Scrambled word: {}", scrambled);
        println!("Form words using the scrambled letters!");

        let mut score = 0;
        let mut guessed_words = HashSet::new();

        loop {
            println!("Enter a word (or type 'quit' to end): ");
            let mut guess = String::new();
            io::stdin().read_lines(&mut guess).unwrap();
            let guess = guess.trim().to_lowercase();

            if guess == "quit" {
                println!("Thanks for playing! Final score: {}", score);
                break;
            }

            if guessed_words.contains(&guess) {
                println!("You already guessed that word!");
                continue;
            }

            if word_bank.contains(&guess) && is_valid_guess(&guess, &scrambled) {
                guessed_words.insert(guess.clone());
                let points = guess.len();
                score += points;
                println!("You Scored {} points. Current Score: {}", points, score);

                if guess == secret_word {
                    println!("You guessed the SECRET WORD: {}", secret_word);
                    break;
                }
            } else {
                println!("Nope! Try Again!");
            }
        }
    } else {
        println!("There are no words in the six_word_file. THIS IS AN ERROR");
    }
    Ok(())
}