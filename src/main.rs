use std::io;
// use std::fs;
use std::fs::File;

mod symbol;
mod vm;
mod name_table;


fn main() {
    let nxtlev: Vec<bool> = Vec::with_capacity(symbol::SYMBOL_NUMBER.into());

    let mut input_file_name = String::new();
    println!("Input pl/0 file?");
    io::stdin().read_line(&mut input_file_name).expect("Failed to read line");

    input_file_name = input_file_name.trim().to_string();
    println!("Reading {:?}", input_file_name);
    let reader = symbol::io::PL0SourceCodeReader::create_from_file(&input_file_name);
}
