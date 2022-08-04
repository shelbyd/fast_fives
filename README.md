# fast fives

A faster implementation of finding sets of 5 words of length 5 that all contain different letters from a provided list of words.
See [https://www.youtube.com/watch?v=\_-AfhLQfb6w](https://www.youtube.com/watch?v=_-AfhLQfb6w) for more info.
The algorithms presented in the video appeared optimizable.

## Results

Currently this takes about `~15 seconds` to generate the "correct" 831 words output according to the linked video on my i9-12900K @ 3.2GHz, which is admittedly a little overkill.

## Technique

This primarily achieves the performance it has by:

1. Being written in Rust, a language that compiles to machine code
1. Using multiple threads
1. After choosing a word, only considering words that are lexicographically after that one, trusting another iteration would capture when that appears before

There are likely more improvements to make. But another few orders of magnitude improvement over the existing solutions is sufficient for now. :)

1. Compare word validity using bitstrings and XORs
1. Prune fully-checked words from initial filtering

## Installation

Install Rust [https://rustup.rs/](https://rustup.rs/).

Clone this repository.

Run:

```sh
cargo run --release
```

## Options

If you'd like to provide an alternate word list, you can specify the input file as an argument to the program:

```sh
cargo run --release -- your_word_file.txt
```

The program expects that file to be a newline separated list of words. It will only consider 5 letter words.
