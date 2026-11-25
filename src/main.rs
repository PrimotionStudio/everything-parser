use everything_parser::{read_file::read_file, setup::setup};
use std::process;

fn main() {
    println!("Everything Parser!");

    let args: Vec<String> = std::env::args().collect();

    if args.len() < 3 {
        println!("Usage: everything-parser analyze access.log");
        process::exit(0);
    }

    let mut config = setup(args);

    println!("Config: {:#?}", config);

    let summary = read_file(&mut config.file);

    println!("Summary: {:#?}", summary);
}
