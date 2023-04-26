use rayon::prelude::*;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> ! {
    // Read the word list into a vector, zip it with the word_power
    // and character count, then sort it by word power, descending
    let mut words: Vec<(String, f64, Vec<isize>)> = read_lines("./words.txt")
        .expect("Words file should be in same directory.")
        .into_iter()
        .map(|line| {
            let word = String::from(line.expect("Failed to read line in words file.").trim());
            (word.clone(), word_power(&word), char_count(&word))
        })
        .collect();

    words.par_sort_unstable_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    loop {
        let mut input = String::new();
        println!("Enter the letters you have:");
        std::io::stdin().read_line(&mut input).unwrap();
        input = String::from(input.trim());
        let input_chars = char_count(&input);

        let possible_words: Vec<(String, f64)> = words
            .par_iter()
            .filter(|word| can_spell(&word, &input_chars))
            .map(|w| (w.0.clone(), w.1.clone()))
            .collect::<Vec<(String, f64)>>()
            .into_iter()
            .rev()
            .take(10)
            .rev()
            .collect();
        println!("{:#?}", possible_words);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// Calculate the relative power of a given word
fn word_power(word: &String) -> f64 {
    let mut length = 0.0;
    let mut letters = word.clone();
    while letters.len() > 0 {
        if letters.len() > 1 && String::from("qu").eq(&letters[0..=1]) {
            length += 2.75;
            letters.pop();
            letters.pop();
        } else {
            let letter = letters.pop();
            length += match letter {
                Some('b') | Some('c') | Some('f') | Some('h') | Some('m') | Some('p') => 1.25,
                Some('v') | Some('w') | Some('y') => 1.5,
                Some('j') | Some('k') => 1.75,
                Some('x') | Some('z') => 2.0,
                _ => 1.0,
            };
        }
    }
    length
}

// Count the number of each letter a word needs to be spelled
fn char_count(word: &String) -> Vec<isize> {
    let mut chars = vec![0; 27];
    let mut letters = word.clone();
    let root = 'a' as usize;
    while letters.len() > 0 {
        let index = letters.pop().expect("String ran out of letters early.") as usize;
        // This handles ? characters
        if index < root || index - root > 25 {
            chars[26] += 1;
        } else {
            // If there's a 'q', take the 'u' as well
            if index == 16 {
                letters.pop();
            }
            chars[index - root] += 1;
        }
    }
    chars
}

// Check if a word can be spelled given an input
fn can_spell(word: &(String, f64, Vec<isize>), input: &Vec<isize>) -> bool {
    let mut remaining_wildcards = input[26].clone();
    for index in 0..26 {
        let char_diff: isize = &input[index] - &word.2[index];
        if char_diff < 0 {
            if char_diff + remaining_wildcards < 0 {
                return false;
            } else {
                remaining_wildcards += char_diff;
            }
        }
    }
    true
}
