#![allow(unused_import)]

use indicatif::{ProgressIterator, ParallelProgressIterator};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::{cmp::Ordering, collections::HashSet};

fn main() {
    let file = std::env::args()
        .nth(1)
        .unwrap_or(String::from("words_alpha.txt"));

    let file_contents = std::fs::read_to_string(file).unwrap();
    let words = file_contents
        .lines()
        .filter(|w| w.len() == 5)
        .filter(|w| w.chars().collect::<HashSet<_>>().len() == 5)
        .collect::<Vec<_>>();
    eprintln!("Calculating solutions for {} words", words.len());

    let start = std::time::Instant::now();
    let result = word_sets_root(&words);
    for set in &result {
        println!("{}", set.join(" "));
    }
    eprintln!("Took {:?}", start.elapsed());
    eprintln!("Found {} solutions", result.len());
}

#[derive(Clone, Copy)]
struct Word<'w> {
    string: &'w str,
    bits: u32,
}

impl<'w> Word<'w> {
    fn new(string: &'w str) -> Self {
        let is_ascii = string.chars().all(|c| c.is_ascii_lowercase());
        if !is_ascii {
            panic!("Requires ascii string got {}", string);
        }

        let mut bits = 0;
        for b in string.bytes() {
            let index = b - b'a';
            let bit = 1 << index;
            if bits & bit != 0 {
                panic!("Found duplicate characters {}", string);
            }
            bits |= bit;
        }

        Word { string, bits }
    }

    fn overlaps(&self, other: Self) -> bool {
        self.bits & other.bits != 0
    }
}

// Orders based on bit strings.
impl<'w> PartialOrd for Word<'w> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.bits.partial_cmp(&other.bits)
    }
}

impl<'w> PartialEq for Word<'w> {
    fn eq(&self, other: &Self) -> bool {
        self.bits == other.bits
    }
}

fn word_sets_root<'w>(options: &[&'w str]) -> Vec<Vec<&'w str>> {
    let options = options.iter().map(|&w| Word::new(w)).collect::<Vec<_>>();
    options
        .par_iter()
        .progress()
        .flat_map(|&word| {
            unique_word_sets(&filter_valid_options(&options, word), 4)
                .into_iter()
                .map(|mut set| {
                    set.insert(0, word.string);
                    set
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

fn filter_valid_options<'w>(options: &[Word<'w>], word: Word<'w>) -> Vec<Word<'w>> {
    options
        .iter()
        // Order of these matters a lot. overlaps is much faster than comparison.
        .filter(|w| !w.overlaps(word))
        .filter(|&w| *w > word)
        .map(|&w| w)
        .collect()
}

fn unique_word_sets<'w>(valid_options: &[Word<'w>], needed_words: u8) -> Vec<Vec<&'w str>> {
    if needed_words == 1 {
        return valid_options.iter().map(|&w| vec![w.string]).collect();
    }

    valid_options
        .iter()
        .flat_map(|&option| {
            let next_options = filter_valid_options(valid_options, option);
            unique_word_sets(&next_options, needed_words - 1)
                .into_iter()
                .map(|mut set| {
                    set.insert(0, option.string);
                    set
                })
        })
        .collect()
}