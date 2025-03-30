# pgen
A simple commandline based password generator for accounts. This 
program leverages Mersenne primes to generate safe and secure passwords for use.  
Mersenne prime of 127 is used here to calculate the hash that will aid in creating a 
secure password.  

### Requirements  
rustc 1.85.1  
cargo 1.85.1  
#### Project dependencies  
anyhow 1.0.97  
clap 4.5.32

## Usage  
Compiling the source code. After cloning the repository, we can compile the 
source code using the command below for optimized release builds.  
`$ cargo build --release`  
After compiling the source code, we can run(with required permissions) our executable program located in 
target/release/main to generate our password as:  
`$ ./target/release/main --seed <ENTER_SEED_PHRASE_HERE>`  
example:  
`$ ./target/release/main --seed myexampleseed`  
`$ myexampleseed: Rj5ndUcmV1cxbUy`  
