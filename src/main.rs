use std::{fs::File, io::ErrorKind, process};

#[derive(Debug)]
struct Config {
    program: String,
    command: String,
    log_filename: String,
}

// enum Commands {
//     ANALYZE,
//     PARSE,
// }

fn main() {
    println!("Everything Parser!");

    let args: Vec<String> = std::env::args().collect();

    if args.len() < 3 {
        println!("Usage: everything-parser analye access.log");
        process::exit(0);
    }

    let mut args = args.into_iter();
    let program = match args.next() {
        None => {
            println!("Usage: program not found");
            process::exit(0);
        }
        Some(program) => program,
    };
    let command = match args.next() {
        None => {
            println!("Usage: command not found");
            process::exit(0);
        }
        Some(command) => command,
    };
    let log_filename = match args.next() {
        None => {
            println!("Usage: log_filename not found");
            process::exit(0);
        }
        Some(log_filename) => {
            let file_result = File::open(&log_filename);
            let _ = match file_result {
                Err(error) => match error.kind() {
                    ErrorKind::NotFound => {
                        println!("Error: File: {} not found", log_filename);
                        process::exit(0);
                    }
                    _ => {
                        println!("Error: an error occured");
                        println!("{}", error);
                        process::exit(0);
                    }
                },
                Ok(file) => file,
            };
            log_filename
        }
    };

    let config = Config {
        program,
        command,
        log_filename,
    };

    println!("Config: {:#?}", config);
}
