use std::{fs::File, io::ErrorKind, process};

#[derive(Debug)]
pub struct Config {
    pub program: String,
    pub command: CommandEnum,
    pub file: File,
    pub log_filename: String,
}

#[derive(Debug)]
pub enum CommandEnum {
    Analyze,
    Parse,
}

pub fn setup(args: Vec<String>) -> Config {
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
        Some(command_str) => {
            let command: CommandEnum = match command_str.as_str() {
                "analyze" => CommandEnum::Analyze,
                "parse" => CommandEnum::Parse,
                _ => {
                    println!("Invalid command: {}", command_str);
                    process::exit(0);
                }
            };
            command
        }
    };

    let log_filename = match args.next() {
        None => {
            println!("Usage: log_filename not found");
            process::exit(0);
        }
        Some(log_filename) => log_filename,
    };
    let file_result = File::open(&log_filename);
    let file = match file_result {
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

    Config {
        program,
        command,
        file,
        log_filename,
    }
}
