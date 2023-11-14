use std::collections::{HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::{Neg};
use crate::entropy::Entropy;

pub struct Alphabet {
    pub letters: Vec<AlphabetLetter>,
    pub power: i32,
}

pub struct AlphabetLetter {
    pub lowers: String,
    pub uppers: String,
    pub upper: char,
    pub lower: char,
}

impl Alphabet {
    pub fn get_redundancy(&self, probabilities: &HashMap<char, f64>) -> f64 {
        (1f64 - self.optimal_entropy() / self.independent_entropy(probabilities)).abs()
    }

    pub fn create_truncated(&mut self, probabilities: &HashMap<char, f64>) -> Alphabet {
        let mut sorted_letters: Vec<(char, f64)> = Vec::new();

        for (&ch, &f) in probabilities.iter() {
            sorted_letters.push((ch, f));
        }

        sorted_letters.sort_by(|(a_k, a_v), (b_k, b_v)| a_v.total_cmp(b_v));

        let power = i32::min((self.letters.len() / 3) as i32, 15);
        let freq = sorted_letters.iter().rev().take(power as usize);

        let mut prob = probabilities.clone();
        prob.retain(|ch, f| self.has_letter(ch));

        let mut letters = Vec::new();

        freq.for_each(|(c, f)| {
            let copy = self.letters.iter().find(|v| v.lower == *c).unwrap();
            letters.push(AlphabetLetter {
                lower: copy.lower,
                upper: copy.upper,
                uppers: copy.uppers.to_string(),
                lowers: copy.lowers.to_string(),
            });
        });

        Alphabet { power, letters }
    }

    pub fn to_string(&self) -> String {
        self.letters.iter()
            .fold(String::new(), |mut acc, l| {
                acc.push(l.upper);
                acc.push(l.lower);
                acc
            })
    }

    pub fn count_probabilities(&mut self, source: &File) -> HashMap<char, f64> {
        let reader = BufReader::new(source);
        let mut occurrences: HashMap<char, i32> = HashMap::new();
        let mut total_chars = 0;
        let mut probabilities: HashMap<char, f64> = HashMap::new();

        for line in reader.lines() {
            for ch in line.unwrap().chars() {
                if let Some(letter) = self.find_letter(&ch) {
                    occurrences.entry(letter.lower)
                        .and_modify(|v| *v += 1)
                        .or_insert(1);
                    total_chars += 1;
                }
            }
        }

        for (&ch, count) in occurrences.iter() {
            probabilities.insert(ch, *count as f64 / total_chars as f64);
        }

        probabilities
    }

    fn find_letter(&self, ch: &char) -> Option<&AlphabetLetter> {
        self.letters.iter()
            .find(|l| l.lowers.contains(*ch) || l.uppers.contains(*ch))
    }

    fn has_letter(&self, ch: &char) -> bool {
        match self.find_letter(ch) {
            Some(_) => true,
            _ => false
        }
    }

    fn has_letter_der(&self, ch: char) -> bool {
        match self.find_letter(&ch) {
            Some(_) => true,
            _ => false
        }
    }

    pub fn load_letters(&mut self, source: &File) {
        let reader = BufReader::new(source);

        for line in reader.lines() {
            if let Ok(value) = line {
                let parts: Vec<&str> = value.split(',').collect();

                if parts.len() == 4 {
                    self.letters.push(AlphabetLetter {
                        upper: parts[0].chars().nth(0).unwrap(),
                        lower: parts[1].chars().nth(0).unwrap(),
                        uppers: parts[2].to_string(),
                        lowers: parts[3].to_string(),
                    });
                    self.power += 1;
                };
            }

        }
    }
}

impl Entropy for Alphabet {
    fn optimal_entropy(&self) -> f64 {
        f64::from(self.power).log2()
    }

    fn independent_entropy(&self, probabilities: &HashMap<char, f64>) -> f64 {
        probabilities
            .values()
            .fold(0f64, |acc, prob| acc + prob * prob.log2())
            .neg()
    }

    fn dependent_entropy(&self) -> f64 {
        0.0
    }
}