use rayon::Scope;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::sync::{Arc, Mutex};
use trie_rs::{Trie, TrieBuilder};

fn main() {
    let mut builder = TrieBuilder::new();
    let mut word_set = HashSet::new();
    // Read the word list file into a trie
    match read_lines("./words.txt") {
        Err(why) => panic!("Couldn't read word list: {}", why),
        Ok(lines) => {
            for line in lines {
                match line {
                    Err(why) => panic!("Couldn't read a word in the list: {}", why),
                    Ok(word) => {
                        builder.push(word.trim());
                        word_set.insert(String::from(word.trim()));
                    }
                }
            }
        }
    };

    let word_trie = builder.build();

    loop {
        let mut input = String::new();
        println!("Enter the letters you have:");
        std::io::stdin().read_line(&mut input).unwrap();

        let visited = Arc::new(Mutex::new(HashSet::new()));
        let possible_words = Arc::new(Mutex::new(HashSet::new()));

        fn process_queue<'a>(
            input: String,
            substring: String,
            word_set: &'a HashSet<String>,
            visited: &'a Arc<Mutex<HashSet<String>>>,
            word_trie: &'a Trie<u8>,
            possible_words: &'a Arc<Mutex<HashSet<String>>>,
            scope: &Scope<'a>,
        ) {
            if word_set.contains(&substring) {
                possible_words.lock().unwrap().insert(substring.clone());
            }
            if substring.len() == 16 {
                return;
            }
            for (index, letter) in input.chars().enumerate() {
                let create_child = |letter| {
                    let mut new_string = substring.clone();
                    new_string.push(letter);
                    let mut new_input = input.clone();
                    new_input.remove(index);
                    let visit = !visited.lock().unwrap().contains(&new_string);
                    let trim = word_trie.predictive_search(&new_string).len() == 0;
                    if visit && !trim {
                        visited.lock().unwrap().insert(new_string.clone());
                        scope.spawn(move |s| {
                            process_queue(
                                new_input,
                                new_string,
                                word_set,
                                &visited,
                                word_trie,
                                &possible_words,
                                s,
                            )
                        });
                    }
                };
                match letter {
                    '?' => (b'a'..=b'z')
                        .map(|l| l as char)
                        .for_each(|l| create_child(l)),
                    _ => create_child(letter),
                }
            }
        }

        rayon::scope(|s| {
            process_queue(
                input.clone(),
                String::from(""),
                &word_set,
                &visited,
                &word_trie,
                &possible_words,
                s,
            )
        });

        let mut usable_words = possible_words
            .lock()
            .unwrap()
            .clone()
            .into_iter()
            .collect::<Vec<String>>();
        usable_words.sort_unstable_by(|a, b| {
            word_power(b)
                .partial_cmp(&word_power(a))
                .unwrap_or(Ordering::Equal)
        });
        println!(
            "{:#?}",
            usable_words
                .iter()
                .take(10)
                .zip(usable_words.iter().take(10).map(|w| word_power(&w)))
                .rev()
                .collect::<Vec<(&String, f64)>>()
        );
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

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
