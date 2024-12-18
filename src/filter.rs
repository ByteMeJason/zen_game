use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

pub fn filter_words() -> io::Result<()> {
    let input_file = File::open("words_alpha.txt")?;
    let reader = BufReader::new(input_file);

    // Create output files
    let mut six_letter_file = File::create("six_letter_words.txt")?;
    let mut word_bank_file = File::create("word_bank.txt")?;

    for line in reader.lines() {
        let word = line?.trim().to_lowercase();

        match word.len() {
            6 => writeln!(six_letter_file, "{}", word)?, // Write 6-letter words
            3..=5 => writeln!(word_bank_file, "{}", word)?, // Write 3..5-letter words
            _ => {} // Skip all other words
        }
    }
    println!("Words successfully filtered & saved!");
    Ok(())
}
