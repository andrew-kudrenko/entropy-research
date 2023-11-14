use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::ops::Deref;
use crate::alphabet::{Alphabet};
use crate::entropy::Entropy;
use crate::text_processor::{purge};

mod alphabet;
mod entropy;
mod text_processor;

fn main() {
    let mut spanish_alphabet = Alphabet {
        letters: vec![],
        power: 0,
    };

    if let Ok(source) = File::open("data/spanish_alphabet.txt") {
        spanish_alphabet.load_letters(&source);

        let purging_result = purge(
            &File::open("data/full_test_message.txt").unwrap(),
            &OpenOptions::new().write(true).open("data/result.txt").unwrap(),
            &spanish_alphabet
        );

        if let Ok(_) = purging_result {
            let probabilities = spanish_alphabet.count_probabilities(
                &File::open("data/result.txt").unwrap()
            );

            print_delimiter();
            println!("Regular Spanish Alphabet");
            print_about_alphabet(&spanish_alphabet, &probabilities);

            let truncated = spanish_alphabet.create_truncated(&probabilities);

            println!("Truncated");
            print_about_alphabet(&truncated, &probabilities);
        }
    }
}

fn print_about_alphabet(alphabet: &Alphabet, probabilities: &HashMap<char, f64>) {
    print_delimiter();
    println!("Alphabet size: {}", alphabet.letters.len());
    println!("Optimal entropy: {}", alphabet.optimal_entropy());
    println!("Independent entropy: {}", alphabet.independent_entropy(&probabilities));
    println!("Redundancy: {}", alphabet.get_redundancy(&probabilities));
    print_delimiter();

    for v in alphabet.letters.iter() {
        println!("{}: {}", v.lower, probabilities.get(&v.lower).unwrap());
    }

    print_delimiter();
}

fn print_delimiter() {
    println!("{}", "_".repeat(40));
}
