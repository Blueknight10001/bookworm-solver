// use rand::distributions::WeightedIndex;
// use rand::prelude::*;
use rayon::prelude::*;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;

fn main() -> ! {
    let start = Instant::now();
    // Read the word list into a vector, zip it with the word_power
    // and character count, then sort it by word power, descending
    let mut words: Vec<(String, f64, [isize; 27])> = read_lines("./words.txt")
        .expect("Words file should be in same directory.")
        .into_iter()
        .map(|line| {
            let word = String::from(line.expect("Failed to read line in words file.").trim());
            (word.clone(), word_power(&word), char_count(&word))
        })
        .collect();

    // Sort the list of words from most to least powerful
    words.par_sort_unstable_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    println!(
        "Time taken to generate word list: {}",
        start.elapsed().as_millis()
    );

    // let start = Instant::now();
    // // Read the word list into a vector, zip it with the word_power
    // // and character count, then sort it by word power, descending
    // let mut better_words: Vec<(String, f64, [isize; 27])> = read_lines("./words.txt")
    //     .expect("Words file should be in same directory.")
    //     .into_iter()
    //     .map(|line| {
    //         let word = String::from(line.expect("Failed to read line in words file.").trim());
    //         (word.clone(), word_power(&word), better_char_count(&word))
    //     })
    //     .collect();
    //
    // // Sort the list of words from most to least powerful
    // better_words.par_sort_unstable_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    //
    // println!(
    //     "Time taken to generate better word list: {}",
    //     start.elapsed().as_millis()
    // );
    //
    // // Shows the distribution of characters in the word list
    // let mut total_chars: Vec<(isize, char)> = words
    //     .iter()
    //     .map(|w| w.2.clone())
    //     .reduce(|sum, count| <[isize; 27]>::try_from(sum.iter().zip(count.iter()).map(|(s, c)| s + c).collect::<Vec<isize>>()).unwrap())
    //     .unwrap()
    //     .into_iter()
    //     .zip('a'..='z')
    //     .collect::<Vec<(isize, char)>>();
    //
    // total_chars.sort_by(|a, b| a.0.cmp(&b.0));
    // let total_total_chars = total_chars.iter().map(|t| t.0).sum::<isize>();
    // println!("{:#?}, {}", total_chars, total_total_chars);
    //
    // let weights: [f64; 26] = <[f64; 26]>::try_from(total_chars.iter().map(|t| (t.0 as f64) / (total_total_chars as f64)).collect::<Vec<f64>>()).unwrap();
    // let charset = <[char; 26]>::try_from(('a'..='z').collect::<Vec<char>>()).unwrap();
    // let dist = WeightedIndex::new(&weights).unwrap();
    // let mut orig_count_time = 0;
    // let mut better_count_time = 0;
    // let mut orig_find_time = 0;
    // let mut better_find_time = 0;
    // let mut rng = thread_rng();
    // for _ in 0..1000 {
    //     let mut test_string: String = String::new();
    //     (0..=16).for_each(|_| test_string.push(charset[dist.sample(&mut rng)]));
    //
    //     let start = Instant::now();
    //     let input_chars = char_count(&test_string);
    //     orig_count_time += start.elapsed().as_nanos();
    //     let start = Instant::now();
    //     let orig_possible_words: Vec<(String, f64)> = words
    //         .par_iter()
    //         .filter(|word| can_spell(&word, &input_chars))
    //         .map(|w| (w.0.clone(), w.1.clone()))
    //         .collect::<Vec<(String, f64)>>()
    //         .into_iter()
    //         .rev()
    //         .take(10)
    //         .rev()
    //         .collect();
    //     orig_find_time += start.elapsed().as_micros();
    //
    //     let start = Instant::now();
    //     let input_chars = better_char_count(&test_string);
    //     better_count_time += start.elapsed().as_nanos();
    //     let start = Instant::now();
    //     let better_possible_words: Vec<(String, f64)> = better_words
    //         .par_iter()
    //         .filter(|word| can_spell(&word, &input_chars))
    //         .map(|w| (w.0.clone(), w.1.clone()))
    //         .collect::<Vec<(String, f64)>>()
    //         .into_iter()
    //         .rev()
    //         .take(10)
    //         .rev()
    //         .collect();
    //     better_find_time += start.elapsed().as_micros();
    //     assert!(
    //         orig_possible_words == better_possible_words,
    //         "orig: {:#?}, better: {:#?}",
    //         orig_possible_words,
    //         better_possible_words
    //     );
    // }
    // println!(
    //     "Original word set average count time: {}",
    //     orig_count_time / 1000
    // );
    // println!(
    //     "Better word set average count time: {}",
    //     better_count_time / 1000
    // );
    // println!(
    //     "Original word set average find time: {}",
    //     orig_find_time / 1000
    // );
    // println!(
    //     "Better word set average find time: {}",
    //     better_find_time / 1000
    // );

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
            .filter(|word| can_spell(&word.2, &input_chars))
            .map(|w| (w.0.clone(), w.1.clone()))
            .collect::<Vec<(String, f64)>>()
            .into_iter()
            .rev()
            .take(10)
            .rev()
            .collect();
        let time_taken = start.elapsed().as_micros();
        println!("{:#?}, {}", possible_words, time_taken);
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
    let mut letters: String = word.chars().rev().collect();
    while letters.len() > 0 {
        let letter = letters.pop();
        length += match letter {
            Some('b') | Some('c') | Some('f') | Some('h') | Some('m') | Some('p') => 1.25,
            Some('v') | Some('w') | Some('y') => 1.5,
            Some('j') | Some('k') => 1.75,
            Some('x') | Some('z') => 2.0,
            Some('q') => {
                letters.pop();
                2.75
            }
            _ => 1.0,
        };
    }
    length
}

// Count the number of each letter a word needs to be spelled
fn char_count(word: &String) -> [isize; 27] {
    let mut chars = [0; 27];
    let mut letters: String = word.chars().rev().collect();
    let root = 'a' as usize;
    while letters.len() > 0 {
        let index = letters.pop().expect("String ran out of letters early.") as usize;
        // This handles ? characters
        if index < root || index - root > 25 {
            chars[26] += 1;
        } else {
            // If there's a 'q', take the 'u' as well
            if index - root == 16 {
                letters.pop();
            }
            chars[index - root] += 1;
        }
    }
    chars
}

// fn better_char_count(word: &String) -> [isize; 27] {
//     let mut chars = [0; 27];
//     let mut letters: String = word.chars().rev().collect();
//     while letters.len() > 0 {
//         let next = letters.pop().expect("String ran out of letters early.");
//         // eisarntolcdupmghbyfvkwzxqj
//         let index = "jqxzwkvfybhgmpudclotnrasie"
//             .chars()
//             .position(|c| c == next)
//             .unwrap_or(26);
//         if index == 1 {
//             letters.pop();
//         }
//         chars[index] += 1;
//     }
//     chars
// }

// Check if a word can be spelled given an input
fn can_spell(word: &[isize; 27], input: &[isize; 27]) -> bool {
    let mut remaining_wildcards = input[26].clone();
    for index in 0..26 {
        let char_diff: isize = &input[index] - &word[index];
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
