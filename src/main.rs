use indicatif::ParallelProgressIterator;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::collections::HashSet;

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

fn word_sets_root<'w>(sorted_options: &[&'w str]) -> Vec<Vec<&'w str>> {
    sorted_options
        .par_iter()
        .progress()
        .flat_map(|word| {
            unique_word_sets(&filter_valid_options(sorted_options, word), 4)
                .into_iter()
                .map(|mut set| {
                    set.insert(0, *word);
                    set
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

fn filter_valid_options<'w>(options: &[&'w str], word: &str) -> Vec<&'w str> {
    options
        .iter()
        .filter(|&w| *w > word)
        .filter(|w| w.chars().all(|c| !word.contains(c)))
        .map(|&w| w)
        .collect()
}

fn unique_word_sets<'w>(valid_options: &[&'w str], needed_words: u8) -> Vec<Vec<&'w str>> {
    if needed_words == 1 {
        return valid_options.iter().map(|&w| vec![w]).collect();
    }

    valid_options
        .iter()
        .flat_map(|option| {
            let next_options = filter_valid_options(valid_options, option);
            unique_word_sets(&next_options, needed_words - 1)
                .into_iter()
                .map(|mut set| {
                    set.insert(0, option);
                    set
                })
        })
        .collect()
}
