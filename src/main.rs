#![allow(unused)]
#![allow(dead_code)]

use colored::{self, *};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("Incorrect Number Of Arguments Supplied");
    }
    let config: grepoxide::Config = grepoxide::Config::new(&args[1][..], &args[2][..]);
}
