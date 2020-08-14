use std::io;
use std::fs;

use logos::Logos;

mod symbol;
mod vm;
mod name_table;
mod parser;


fn main() {

    let mut input_file_name = String::new();
    println!("Input pl/0 file?");
    // io::stdin().read_line(&mut input_file_name).expect("Failed to read line");

    input_file_name = "sample/sample0.pl0".to_string();
    println!("Reading {:?}", input_file_name);

    let contents = fs::read_to_string(input_file_name)
        .expect("Something went wrong reading the file");

    let mut current_symbol: symbol::Symbol = symbol::Symbol::Nul;

    let mut lex: symbol::io::PL0Lexer = symbol::io::PL0Lexer::create_from_content(&contents);

    loop {
        match lex.next() {
            symbol::Symbol::Constsym => {
                println!("Declaring some constants {}, {:?}", lex.current_content(), lex.current_index());
            },
            symbol::Symbol::Varsym => {
                println!("Declaring some variables");
            },
            symbol::Symbol::Procsym => {
                println!("Declaring some procedures");
            },
            symbol::Symbol::EOF => {
                break;
            },
            _ => {
                // println!("Other");
            }
        }
    }
}
