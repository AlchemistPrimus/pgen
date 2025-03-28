//! MAIN FILE
//! This is a password generator for any account.
use anyhow::{bail, Result};
use clap::Parser;
use encryptor::passwd::gen_passwd;

const USAGE: &str = "
USAGE:
    pgen --seed <SEED>
";

#[derive(Parser, Debug)]
#[clap(version, about, long_about=None)]
struct Args {
    // Seed phrase to use to generate password.
    #[clap(short, long)]
    seed: String,

    // Length of the password.
    #[clap(short, long, default_value_t=16)]
    length: usize,
}


fn main() -> Result<()> {
    let args = Args::parse();

    if args.seed.len() < 4 {
        bail!("Seed {} length must be >= 4", &args.seed);
    }

    let (seed, length) = (args.seed, args.length);
    let passwd = gen_passwd(&seed[..], length);

    match passwd {
        Ok(val) => println!("{}", val),
        Err(err) => println!("{}", err),
    }

    Ok(())
}
