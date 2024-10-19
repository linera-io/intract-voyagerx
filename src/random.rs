use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

// Function to generate a random number based on a string input
// and within a specified range defined by min and max.
pub fn gen_range(input: &str, min: u16, max: u16) -> u16 {
    // Hash the input string to create a seed
    let mut hasher = DefaultHasher::new();
    input.hash(&mut hasher);
    let seed = hasher.finish();

    // Calculate the range
    let range = max - min;

    // Use the seed to get a number within the range using modulus
    (seed % range as u64) as u16 + min
}
