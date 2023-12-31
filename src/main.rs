use clap::Parser;
use rayon::prelude::*;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long, action)]
    use_ba2: bool,
}

struct Word {
    letters: String,
    power: u8,
    counts: Counts,
}

// This is the simplest way of making a HashMap with my own hasher
// This lets me only check the characters that have values without iterating
struct Counts {
    char_keys: Vec<u8>,
    char_values: [u8; 26],
    wildcards: u8,
}

fn main() -> () {
    let args = Args::parse();
    let words_file = File::open(Path::new(if args.use_ba2 {
        "./ba2-words.txt"
    } else {
        "./ba1-words.txt"
    }))
    .expect("Bookworm Adventures words file not in directory!");
    let words_reader = io::BufReader::new(words_file);
    let mut words: Vec<Word> = words_reader
        .lines()
        .collect::<Result<Vec<String>, _>>()
        .expect("Failed to read lines from file!")
        .into_par_iter()
        .map(|line| {
            let letters = String::from(line.trim()).replace("qu", "q");
            let power = word_power(&letters);
            let counts = char_counts(&letters);
            Word {
                letters,
                power,
                counts,
            }
        })
        .collect();

    // Sort the list of words from most to least powerful
    words.par_sort_unstable_by(|a, b| b.power.cmp(&a.power));

    // Repeatedly ask for user input, and give the 5
    // best words that can be spelled from that input
    loop {
        let mut input = String::new();
        println!("Enter letters ('Qu' == 'q', wildcards == non-letters, Ctrl-C to stop):");
        std::io::stdin().read_line(&mut input).unwrap();
        input = String::from(input.trim());

        let input_counts = char_counts(&input);
        let possible_words: Vec<(&String, &u8)> = words
            .par_iter()
            .filter(|word| can_spell(&word.counts, &input_counts))
            .map(|word| (&word.letters, &word.power))
            .collect::<Vec<(&String, &u8)>>()
            .into_iter()
            .take(5)
            .collect();
        println!("{:?}", possible_words);
    }
}

// Calculate the relative power of a given word
#[inline]
fn word_power(word: &String) -> u8 {
    word.as_bytes().into_iter().fold(0, |length, letter| {
        length
            + match letter {
                b'b' | b'c' | b'f' | b'h' | b'm' | b'p' => 5,
                b'v' | b'w' | b'y' => 6,
                b'j' | b'k' => 7,
                b'x' | b'z' => 8,
                b'q' => 11,
                _ => 4,
            }
    })
}

// Count the number of each letter a word needs to be spelled
#[inline]
fn char_counts(word: &String) -> Counts {
    word.as_bytes().into_iter().fold(
        Counts {
            char_keys: Vec::new(),
            char_values: [0; 26],
            wildcards: 0,
        },
        |mut counts, letter| {
            let index = (*letter as usize) - ('a' as usize);
            match index > 25 {
                false => {
                    if counts.char_values[index] == 0 {
                        counts.char_keys.push(*letter);
                    }
                    counts.char_values[index] += 1;
                }
                true => counts.wildcards += 1,
            };
            counts
        },
    )
}

// Check if a word can be spelled given an input
#[inline]
fn can_spell(word: &Counts, input: &Counts) -> bool {
    word.char_keys
        .iter()
        .fold(Some(input.wildcards), |wildcards, key| {
            let index = (*key as usize) - ('a' as usize);
            let debt = word.char_values[index].checked_sub(input.char_values[index]);
            match debt {
                None => wildcards,
                Some(d) => wildcards?.checked_sub(d),
            }
        })
        .is_some()
}
