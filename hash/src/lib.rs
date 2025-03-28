//! HASH LIBRARY. CALCULATES HASH VALUES BASED ON MERSENNE PRIMES
pub mod mersnhash;

#[cfg(test)]
mod tests {
    use super::mersnhash::mersn_hash;

    #[test]
    fn it_mersn_hash_works() {
        let seed = String::from("seedexample");
        let hash = mersn_hash(&seed);
        assert_eq!(999, hash);
    }
}
