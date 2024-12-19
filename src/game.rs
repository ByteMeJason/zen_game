use crate::word_utils::{find_valid_substrings, is_valid_guess, load_words, pick_random_word, scramble_word};
use std::collections::{HashMap, HashSet};
use std::io::{self, Write};

pub fn display_game_board(
    scrambled: &str,
    guessed_words: &HashSet<String>, 
    valid_substrings: &[String],
) {
    // Clear the terminal
    print!("\x1B[2J\x1B[1;1H");
    std::io::stdout().flush().unwrap();

    // Group valid words by length
    let mut word_groups: HashMap<usize, Vec<String>> = HashMap::new();
    for word in valid_substrings {
       word_groups.entry(word.len()).or_default().push(word.clone());
    }

    let total_words = valid_substrings.len();
    let words_per_column = 9;
    let num_columns = ((total_words + words_per_column - 1)/words_per_column) as usize; // rounded up

    println!("++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++");
    println!("Scrambled word: {}\n", scrambled);
    println!("Form words using the scrambled letters!");
    println!("Total possible valid words: {}\n", total_words);
    println!("Enter a word, or type:");
    println!("0 - Quit");
    println!("1 - Show all valid substrings");
    println!("2 - Reveal the secret word\n");

    // Flatten all valid substrings and sort by length
    let mut all_words: Vec<_> = valid_substrings.iter().cloned().collect();
    all_words.sort_by(|a, b| a.len().cmp(&b.len())); // Sort words by length

    // Prepare columns
    let mut columns = vec![vec![]; num_columns];
    for (i, word) in all_words.iter().enumerate() {
        if guessed_words.contains(word) {
            columns[i % num_columns].push(word.to_string()); // Guessed word
        } else {
            columns[i % num_columns].push("_".repeat(word.len())); // Placeholder
        }
    }

    // Display columns
    let max_rows = columns.iter().map(|col| col.len()).max().unwrap_or(0);
    for row in 0..max_rows {
        for col in &columns {
            if let Some(word) = col.get(row) {
                print!("{:<12}", word); // Adjust column width
            } else {
                print!("{:<12}", ""); // Empty space for shorter columns
            }
        }
        println!();
    }
    println!("++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++");

}

pub fn run_game() -> io::Result<()> {
    let six_letter_words = load_words("six_letter_words.txt")?;
    let word_bank = load_words("word_bank.txt")?;

    if let Some(secret_word) = pick_random_word(&six_letter_words) {
        let scrambled = scramble_word(&secret_word);
        let valid_substrings = find_valid_substrings(&word_bank, &scrambled);
        let mut guessed_words: HashSet<_> = HashSet::new();
       

        loop {
            // Game board
            display_game_board(&scrambled, &guessed_words, &valid_substrings);
            
            println!("Your guess: ");
            let mut guess = String::new();
            io::stdin().read_line(&mut guess).unwrap();
            let guess = guess.trim().to_lowercase();

            if guess == "0" {
                println!("Thanks for playing! Final score: {}", guessed_words.len());
                break;
            }

            if guess == "1" {
                println!("Valid substrings: {:?}", valid_substrings);
                continue;
            }

            if guess == "2" {
                println!("The secret word is: {}", secret_word);
                println!("Game over. Final score: {}", guessed_words.len());
                break;
            }

            if guessed_words.contains(&guess) {
                println!("You already guessed that word!");
                continue;
            }


            if valid_substrings.contains(&guess) && is_valid_guess(&guess, &scrambled) {
                guessed_words.insert(guess.clone());
                println!("Great! You guessed a valid word: {}", guess);
                
                // let points = guess.len();
                // score += points;
                // println!("You Scored {} points. Current Score: {}", points, score);

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
