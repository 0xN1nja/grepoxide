#![allow(unused)]
#![allow(dead_code)]

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("Incorrect Number Of Arguments Supplied");
        std::process::exit(1);
    }
    let config: grepoxide::Config = grepoxide::Config::new(&args[1][..], &args[2][..]);
    let result_vec: Vec<String> = grepoxide::search(config.query, config.file_path);
    for i in result_vec {
        println!("{}", i);
    }
}
