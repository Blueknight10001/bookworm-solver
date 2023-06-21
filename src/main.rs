use rand::distributions::WeightedIndex;
use rand::prelude::*;
use rayon::prelude::*;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;

fn main() -> () {
    let start = Instant::now();
    // Read the word list into a vector, zip it with the word_power
    // and character count, then sort it by word power, descending
    let mut words: Vec<(String, f64, [isize; 26])> = read_lines("./words.txt")
        .expect("Words file should be in same directory.")
        .collect::<Result<Vec<String>, _>>()
        .unwrap()
        .into_par_iter()
        .map(|line| {
            let word = String::from(line.trim());
            let power = word_power(&word);
            let count = char_count(&word).0;
            (word, power, count)
        })
        .collect();

    // Sort the list of words from most to least powerful
    words.par_sort_unstable_by(|a, b| {
        if a.1 == b.1 {
            a.0.len().partial_cmp(&b.0.len()).unwrap()
        } else {
            b.1.partial_cmp(&a.1).unwrap()
        }
    });

    println!(
        "Time taken to generate word list: {}ms",
        start.elapsed().as_millis()
    );

    // Shows the distribution of characters in the word list
    let mut char_dist: Vec<(isize, char)> = words
        .iter()
        .map(|w| w.2.clone())
        .reduce(|sum, count| {
            <[isize; 26]>::try_from(
                sum.iter()
                    .zip(count.iter())
                    .map(|(s, c)| s + c)
                    .collect::<Vec<isize>>(),
            )
            .unwrap()
        })
        .unwrap()
        .into_iter()
        .zip('a'..='z')
        .collect::<Vec<(isize, char)>>();

    char_dist.sort_by(|a, b| a.0.cmp(&b.0));
    let total_chars = char_dist.iter().map(|t| t.0).sum::<isize>();
    // println!("{:#?}, {}", char_dist, total_chars);
    // eisarntolcdupmghbyfvkwzxqj

    let weights: [f64; 26] = <[f64; 26]>::try_from(
        char_dist
            .iter()
            .map(|t| (t.0 as f64) / (total_chars as f64))
            .collect::<Vec<f64>>(),
    )
    .unwrap();
    let charset = <[char; 26]>::try_from(('a'..='z').collect::<Vec<char>>()).unwrap();
    let dist = WeightedIndex::new(&weights).unwrap();
    let mut count_time = 0;
    let mut find_time = 0;
    let mut rng = thread_rng();
    for _ in 0..1000 {
        let mut test_string: String = String::new();
        (0..16).for_each(|_| {
            let letter = charset[dist.sample(&mut rng)];
            match letter {
                'q' => test_string += "qu",
                _ => test_string.push(letter),
            }
        });

        let start = Instant::now();
        let input_chars = char_count(&test_string);
        count_time += start.elapsed().as_nanos();
        let start = Instant::now();
        let _possible_words: Vec<(String, f64)> = words
            .par_iter()
            .filter(|word| can_spell(&word.2, &input_chars.0, input_chars.1))
            .map(|w| (w.0.clone(), w.1.clone()))
            .collect::<Vec<(String, f64)>>()
            .into_iter()
            .take(10)
            .rev()
            .collect();
        find_time += start.elapsed().as_micros();
    }
    println!("Word set average count time: {}ns", count_time / 1000);
    println!("Word set average find time: {}us", find_time / 1000);

    // Repeatedly ask for user input, and give the 10
    // best words that can be spelled from that input
    loop {
        let mut input = String::new();
        println!("Enter the letters you have (all lowercase, 'Qu' == 'qu'):");
        std::io::stdin().read_line(&mut input).unwrap();
        input = String::from(input.trim());

        let start = Instant::now();
        let input_chars = char_count(&input);
        let possible_words: Vec<(String, f64)> = words
            .par_iter()
            .filter(|word| can_spell(&word.2, &input_chars.0, input_chars.1))
            .map(|w| (w.0.clone(), w.1.clone()))
            .collect::<Vec<(String, f64)>>()
            .into_iter()
            .take(10)
            .rev()
            .collect();
        let time_taken = start.elapsed().as_micros();
        println!("{:#?}, {}us", possible_words, time_taken);
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
    let mut letters = word.chars();
    while let Some(letter) = letters.next() {
        length += match letter {
            'b' | 'c' | 'f' | 'h' | 'm' | 'p' => 1.25,
            'v' | 'w' | 'y' => 1.5,
            'j' | 'k' => 1.75,
            'x' | 'z' => 2.0,
            'q' => {
                letters.next();
                2.75
            }
            _ => 1.0,
        };
    }
    length
}

// Count the number of each letter a word needs to be spelled
// eisarntolcdupmghbyfvkwzxqj
fn char_count(word: &String) -> ([isize; 26], isize) {
    let mut chars = [0; 26];
    let mut wildcards = 0;
    let mut letters = word.chars();
    while let Some(letter) = letters.next() {
        let index = match letter {
            '?' => 26,
            'e' => 00,
            'i' => 01,
            's' => 02,
            'a' => 03,
            'r' => 04,
            'n' => 05,
            't' => 06,
            'o' => 07,
            'l' => 08,
            'c' => 09,
            'd' => 10,
            'u' => 11,
            'p' => 12,
            'm' => 13,
            'g' => 14,
            'h' => 15,
            'b' => 16,
            'y' => 17,
            'f' => 18,
            'v' => 19,
            'k' => 20,
            'w' => 21,
            'z' => 22,
            'x' => 23,
            'q' => 24,
            'j' => 25,
            _ => panic!(),
        };
        if index == 26 {
            wildcards += 1;
        }
        chars[index] += 1;
        // Skips the 'u' after a 'q'
        if letter == 'q' {
            letters.next();
        }
    }
    (chars, wildcards)
}

// Check if a word can be spelled given an input
fn can_spell(word: &[isize; 26], input: &[isize; 26], wildcards: isize) -> bool {
    let mut debt = 0;
    for index in 0..26 {
        let diff: isize = &word[index] - &input[index];
        debt += diff * (diff > 0) as isize;
        if debt > wildcards {
            return false;
        }
    }
    true
}
