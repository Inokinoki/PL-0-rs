use std::fs::File;
use std::io::{BufRead, BufReader};
use std::vec::IntoIter;



pub struct PL0SourceCodeReader {
    /* keep the fields private */
    current_char: char,
    current_line: String,
    buf_reader: BufReader<File>,
    char_iter: usize,
    // char_iter: Option<IntoIter<char>>,
    max_char_in_current_line: usize,
}

impl PL0SourceCodeReader {
    /**
     * Constructor
     */
    pub fn create_from_file(file_name: &str) -> PL0SourceCodeReader {
        let input_file = File::open(file_name).unwrap();
        let mut reader = PL0SourceCodeReader {
            current_char: ' ',
            current_line: String::new(),
            buf_reader: BufReader::new(input_file),
            char_iter: 0,
            // char_iter: None,
            max_char_in_current_line: 0,
        };
        reader.next_line();
        // reader.current_char = reader.char_iter.unwrap().next().unwrap();
        reader
    }

    /**
     * Read next line into PL0SourceCodeReader structure, only call intern
     */
     fn next_line(&mut self) {
        self.max_char_in_current_line =
            self.buf_reader.read_line(&mut self.current_line).unwrap().into();
        self.char_iter = 0;
    }
    // fn next_line(&mut self) {
    //     self.max_char_in_current_line =
    //         self.buf_reader.read_line(&mut self.current_line).unwrap().into();
    //     let last_char_iter = self.current_line;
    //     self.char_iter = Some(self.current_line.chars().collect::<Vec<_>>().into_iter());
    // }

    /**
     * Read a char, auto jump to next line
     */
    pub fn get_ch(&mut self) -> char {
        let mut chars = self.current_line.chars();
        for n in 0..self.char_iter {
            match chars.next() {
                Some(char) => (),
                None => { self.next_line(); chars = self.current_line.chars(); break; },
            }
        }
        if let c = chars.next() {
            self.current_char = c.unwrap();
        } else {
            self.max_char_in_current_line =
                self.buf_reader.read_line(&mut self.current_line).unwrap().into();
            self.char_iter = 0;
            let mut chars = self.current_line.chars();
            self.current_char = chars.next().unwrap();
        }
        self.char_iter = self.char_iter + 1;

        // self.current_char = self.char_iter.as_ref().unwrap().next().unwrap();
        self.current_char
    }
}