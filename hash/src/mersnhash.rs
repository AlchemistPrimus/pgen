//! HASH FUNCTION.
//!
/// To generate hash value, ASCII value of each character in the seed 
/// is multiplied by its corresponding index value and the result is taken 
/// modulo Mersenne prime 127. The remainder is then raised to the third 
/// power to obtain hash value of the seed.
///
/// # Example usage:
/// ```use hash::mersnhash::mersn_hash;
///    
///    let seed = "seedexample";
///    let hash = mersn_hash(&seed);
/// ```
pub fn mersn_hash(seed: &str) -> usize {
    let mut hash: usize = 0;

    for (i, c) in seed.chars().enumerate() {
        hash += (i + 1) * (c as usize);
    }

    (hash % 127).pow(3) - 1
}
