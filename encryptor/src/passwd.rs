//! PASSWORD GENERATOR
///
///To generate the password, seed provided 
///is first passed to the hash function to produce its 
///mersenne hash.
///Alongside characters in the seed, the mersenne hash is used 
///to generate password.
use anyhow::{bail, Error, Result};
use base64::{ engine::general_purpose, Engine as _};
use hash::mersnhash::mersn_hash;


const CRYPTO: &str = "!pqHr$*+ST1Vst_uv:?wWS%X&Y-/Z01_2.34<ABl9ECo|x#yDE^F{GHEI[]JK>LM#NOBWPQ:RaKU@}cde56R7=8f/9gIhi,jkzmn";

pub fn gen_passwd(seed: &str, length: usize) -> Result<String, Error> {
    if length < 6 {
        bail!("Length must be >= 6");
    }

    // Calculating the Mersenne hash
    let p = match length {
        6..=10 => 1,
        11..=15 => 2,
        16..=20 => 3,
        _ => 3,
    };
    let mut merhash = mersn_hash(seed).pow(p);

    // Calculate password of the mersenne hash.
    let mut password = String::new();
    let crypto_len = CRYPTO.len();
    while merhash > 9 {
        let loc = merhash % crypto_len;
        let nthc = CRYPTO.chars().nth(loc).expect("Error while getting char!");
        password.push(nthc);
        merhash /= crypto_len;
    }

    // Combine seed and password
    let interval = password.clone();
    for c in seed.chars() {
        password.push(c);
        password += &interval;
    }

    // Encode password to base64
    password = general_purpose::STANDARD.encode(&password);
    password = password.replace("+", "*").replace("/", "*");
    let interval = password.clone();
    while password.len() < length {
        password += &interval;
    }

    Ok(format!("{}: {}", seed, &password[..length]))
}
