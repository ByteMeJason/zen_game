mod filter;
mod game;
mod word_utils;

use filter::filter_words;
use game::run_game;

fn main() -> std::io::Result<()> {
    let lock = false; // Set to true to filter the words

    if lock {
        match filter_words() {
            Ok(_) => println!("Filtering completed"),
            Err(e) => eprintln!("An error occurred: {}", e),
        }
    } else {
        println!("Skipping word filtering. Set lock to true to filter words.");
    }

    run_game()
}



