use std::io;
use std::fs;
use std::env;

mod symbol;
mod vm;
mod compile;


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        // Help info
        panic!("Please pass pl/0 file name as the first command-line argument.");
    }
    let input_file_name = &args[1];

    // input_file_name = "sample/sample1.pl0".to_string();
    println!("Reading {:?}", input_file_name);

    let contents = fs::read_to_string(input_file_name)
        .expect("Something went wrong reading the file");

    let mut lex: symbol::io::PL0Lexer = symbol::io::PL0Lexer::create_from_content(&contents);

    let mut generator = compile::codegen::CodeGenerator::new();

    generator.build_block(0, &mut lex);

    let mut pl0_vm_1: vm::PL0VirtualMachine = 
        vm::PL0VirtualMachine::load(generator.get_vm_code().to_vec());
    pl0_vm_1.execute();

    println!("Execution terminated");
}
