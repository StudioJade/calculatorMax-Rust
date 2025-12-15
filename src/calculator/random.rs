//! Random number generation functions

use rand::Rng;

/// Generates a random integer between a and b (inclusive)
pub fn randint(a: i64, b: i64) -> i64 {
    let mut rng = rand::rng();
    rng.random_range(a..=b)
}

/// Generates a random float between 0 and 1
pub fn random() -> f64 {
    let mut rng = rand::rng();
    rng.random_range(0.0..1.0)
}

/// Generates a random float between a and b
pub fn uniform(a: f64, b: f64) -> f64 {
    let mut rng = rand::rng();
    rng.random_range(a..b)
}
