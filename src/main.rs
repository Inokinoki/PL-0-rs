use std::io;
use std::fs;

use logos::Logos;

mod symbol;
mod vm;
mod name_table;


fn main() {

    let mut input_file_name = String::new();
    println!("Input pl/0 file?");
    io::stdin().read_line(&mut input_file_name).expect("Failed to read line");

    input_file_name = input_file_name.trim().to_string();
    println!("Reading {:?}", input_file_name);

    let contents = fs::read_to_string(input_file_name)
        .expect("Something went wrong reading the file");

    let mut lex = symbol::symbol::lexer(&contents);

    loop {
        match lex.next() {
            Some(token) => {
                println!("Type: {:?}, \tContent: {}", token, lex.slice());
            },
            None => {
                break;
            }
        }
    }
}
