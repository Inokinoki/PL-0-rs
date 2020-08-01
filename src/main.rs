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

    if let input_pl0_file = File::open(&input_file_name).unwrap() {
        let mut contents = String::new();
        // input_pl0_file.read_to_string(&mut contents);
        println!("Contents: {:?}", contents);
    }

}
