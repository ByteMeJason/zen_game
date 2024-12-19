use std::fs::File;
use std::io::{self, BufRead, BufReader};
use rand::seq::SliceRandom;
use std::collections::HashSet;
use rand::prelude::IteratorRandom;

pub fn load_words(file_path: &str) -> io::Result<HashSet<String>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let words = reader
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| line.trim().to_lowercase())
        .collect();
    Ok(words)
}

pub fn pick_random_word(words: &HashSet<String>) -> Option<String> {
    let mut rng = rand::thread_rng();
    words.iter().choose(&mut rng).cloned()
}

pub fn scramble_word(word: &str) -> String {
    let mut chars: Vec<char> = word.chars().collect();
    let mut rng = rand::thread_rng();
    chars.shuffle(&mut rng);
    chars.into_iter().collect()
}

pub fn is_valid_guess(guess: &str, scrambled: &str) -> bool {
    let mut available_letters: Vec<char> = scrambled.chars().collect();
    for ch in guess.chars() {
        if let Some(pos) = available_letters.iter().position(|&x| x == ch) {
            available_letters.remove(pos);
        } else {
            return false;
        }
    }
    true
}

pub fn find_valid_substrings(word_bank: &HashSet<String>, scrambled: &str) -> Vec<String> {
    word_bank
        .iter()
        .filter(|word| is_valid_guess(word, scrambled))
        .cloned()
        .collect()
}

