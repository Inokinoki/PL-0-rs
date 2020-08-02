use std::fs::File;
use std::io::{BufRead, BufReader};


pub struct PL0SourceCodeReader {
    current_char: char,
    current_line: String,
    input_file: File,
}

impl PL0SourceCodeReader {
    pub fn create_from_file(file_name: &str) -> PL0SourceCodeReader {
        PL0SourceCodeReader {
            current_char: ' ',
            current_line: String::new(),
            input_file: File::open(file_name).unwrap()
        }
    }
}
