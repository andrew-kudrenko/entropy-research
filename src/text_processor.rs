use std::fmt::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use crate::alphabet::Alphabet;

pub fn purge(input: &File, output: &File, alphabet: &Alphabet) -> Result<(), Error> {
    let reader = BufReader::new(input);
    let mut writer = BufWriter::new(output);

    for line in reader.lines() {
        if let Ok(value) = line {
            let cleared = alphabet_letters_only(alphabet, &value);

            writer.write_all(cleared.as_bytes()).expect("Writing failed");
        }
    }

    Ok(())
}

fn alphabet_letters_only(alphabet: &Alphabet, value: &str) -> String {
    let mut result = String::new();

    for char in value.chars() {
        let found = alphabet.letters.iter()
            .find(|l| l.lowers.contains(char) || l.uppers.contains(char));

        if let Some(letter) = found {
            result.push(letter.lower);
        }
    }

    result.push('\n');
    result
}