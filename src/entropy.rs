use std::collections::HashMap;

pub trait Entropy {
    fn optimal_entropy(&self) -> f64;
    fn independent_entropy(&self, probabilities: &HashMap<char, f64>) -> f64;
    fn dependent_entropy(&self) -> f64;
}