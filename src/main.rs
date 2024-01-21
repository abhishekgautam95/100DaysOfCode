use rand::seq::SliceRandom;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "PasswordGenerator", about = "A strong password generator")]
struct Arguments {
    #[structopt(long = "length", short = "l", default_value = "8")]
    length: usize,
    #[structopt(long = "numbers", short = "n", help = "Include numbers in the password")]
    numbers: bool,
    #[structopt(long = "symbols", short = "s", help = "Include symbols in the password")]
    symbols: bool,
    #[structopt(long = "alphabets", short = "a", help = "Include alphabets in the password")]
    alphabets: bool,
}

impl Arguments {
    fn get() -> Self {
        let args: Arguments = Arguments::from_args();
        args
    }
}

#[derive(Debug)]
pub struct Password {
    as_text: String,
    length: usize,
}

impl Password {
    pub fn new(args: &Arguments) -> Self {
        let char_pool: Vec<char> = {
            let mut pool = Vec::new();
            if args.numbers {
                pool.extend("0123456789".chars());
            }
            if args.symbols {
                pool.extend("!@#$%^&*()_+-=[]{};:,.<>/?".chars());
            }
            if args.alphabets {
                pool.extend("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars());
            }
            pool
        };

        let as_text: String = (0..args.length)
            .map(|_| *char_pool.choose(&mut rand::thread_rng()).unwrap())
            .collect();

        Self {
            as_text,
            length: args.length,
        }
    }
}

fn main() {
    let args = Arguments::get();
    let password = Password::new(&args);
    println!("Generated Password: {:?}", password);
}

