use std::io;
use std::fs;

use logos::Logos;

mod symbol;
mod vm;
mod compile;


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

    let mut generator = compile::codegen::CodeGenerator::new();

    generator.build_block(0, &mut lex);

    let mut pl0_vm_1: vm::PL0VirtualMachine = 
        vm::PL0VirtualMachine::load(generator.get_vm_code().to_vec());
    pl0_vm_1.execute();

    println!("Execution terminated");
}
