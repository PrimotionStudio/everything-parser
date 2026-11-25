use std::{
    fs::File,
    io::{BufRead, BufReader},
    process,
};

#[derive(Debug)]
pub struct Summary {
    pub total_lines: i32,
    pub empty_lines: i32,
    pub get_entries: i32,
    pub post_entries: i32,
    pub summary: String,
}

pub fn read_file(file: &mut File) -> Summary {
    let buffer_reader = BufReader::new(file);
    let mut empty_line_count = 0;
    let mut get_line_count = 0;
    let mut post_line_count = 0;
    let mut line_count = 0;

    for (i, line) in buffer_reader.lines().enumerate() {
        match line {
            Err(error) => {
                println!("Cannot read file, line {}", i + 1);
                println!("Error: {:#?}", error);
                process::exit(0);
            }
            Ok(line) => {
                if line.is_empty() {
                    empty_line_count += 1;
                } else if line.contains("GET") {
                    get_line_count += 1;
                } else if line.contains("POST") {
                    post_line_count += 1;
                }
                line_count += 1;
            }
        }
    }

    Summary {
        empty_lines: empty_line_count,
        get_entries: get_line_count,
        post_entries: post_line_count,
        total_lines: line_count,
        summary: format!(
            "We have {} empty line\nWe have {} GET requests\nWe have {} POST requests\n...and {} lines in total.",
            empty_line_count, get_line_count, post_line_count, line_count
        ),
    }
}
