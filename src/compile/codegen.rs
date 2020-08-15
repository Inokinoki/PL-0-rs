use crate::vm;
use crate::symbol;
use crate::compile::nametab;


pub struct CodeGenerator {
    code: Vec<vm::Instruction>,
    name_table: Vec<nametab::NameTableItem>,

    code_pointer: usize,
    table_pointer: usize,
}

impl CodeGenerator {
    pub fn new() -> CodeGenerator {
        CodeGenerator {
            code: Vec::new(),
            name_table: Vec::new(),
            code_pointer: 0,
            table_pointer: 0,
        }
    }

    pub fn gen(&mut self, opcode: vm::Fct, level: usize, extra: usize) -> vm::Instruction {
        vm::Instruction {
            f: opcode,
            l: level,
            a: extra,
        }
    }

    pub fn build_block(&mut self, level: usize, lexer: &mut symbol::io::PL0Lexer) {
        let mut data_pointer: usize = 0;    // Count data size in this block (single level, no deeper)
        loop {
            match lexer.current() {
                symbol::Symbol::Constsym => {
                    // Const declaration
                    loop {
                        // const gen, at least the first one
                        let mut should_continue = true;
                        let mut identity = String::new();
                        let mut number: i64 = 0;
                        {
                            // Detect identity
                            let symbol = lexer.next();
                            if *symbol != symbol::Symbol::Ident {
                                should_continue = false;
                            }
                            identity = lexer.current_content().to_string();
                        }
                        if should_continue {
                            // Detect =
                            let symbol = lexer.next();
                            if *symbol != symbol::Symbol::Eql && *symbol != symbol::Symbol::Becomes {
                                should_continue = false;
                            }
                            if *symbol == symbol::Symbol::Becomes {
                                println!("Error: {:?} cannot be :=", lexer.current_index());
                                should_continue = false;
                            }
                        }
                        if should_continue {
                            // Detect a number
                            let symbol = lexer.next();
                            if *symbol != symbol::Symbol::Number {
                                should_continue = false;
                            } else {
                                number = lexer.current_content().parse::<i64>()
                                    .expect("Cannot parse i64 constant from current token");
                                self.add_into_name_table(&identity, number, nametab::NameTableObject::Constant, level, data_pointer);
                            }
                        }
                        let symbol = lexer.next();
                        if *symbol != symbol::Symbol::Comma || *symbol == symbol::Symbol::Semicolon {
                            // break
                            break;
                        }
                    }
                },
                symbol::Symbol::Varsym => {
                    // Variable declaration
                    loop {
                        // var gen, at least the first one
                        let mut should_continue = true;
                        let mut identity = String::new();
                        {
                            // Detect identity
                            let symbol = lexer.next();
                            if *symbol != symbol::Symbol::Ident {
                                should_continue = false;
                            }
                            identity = lexer.current_content().to_string();
                            self.add_into_name_table(&identity, 0, nametab::NameTableObject::Variable, level, data_pointer);
                        }
                        let symbol = lexer.next();
                        if *symbol != symbol::Symbol::Comma || *symbol == symbol::Symbol::Semicolon {
                            // break
                            break;
                        }
                    }
                },
                symbol::Symbol::Procsym => {
                    // Proc declaration
                    let mut should_continue = true;
                    let mut identity = String::new();

                    {
                        // Detect identity
                        let symbol = lexer.next();
                        if *symbol != symbol::Symbol::Ident {
                            should_continue = false;
                        }
                        identity = lexer.current_content().to_string();
                        self.add_into_name_table(&identity, 0, nametab::NameTableObject::Procedur, level, data_pointer);
                    }
                    if should_continue {
                        // semicolon
                        let symbol = lexer.next();
                    }
                    if should_continue {
                        // Enter the next level
                        self.build_block(level + 1, lexer);
                    }
                    // TODO: add some rescue solution
                },
                _ => {
                    // Others should not be here...
                },
            }

            if *lexer.current() != symbol::Symbol::Constsym
                && *lexer.current() != symbol::Symbol::Varsym
                && *lexer.current() != symbol::Symbol::Procsym {
                // End of declaration
                break;
            }
        }

        // Generate current block
        self.code[self.name_table[self.table_pointer].adr].a = self.code_pointer;
        self.name_table[self.table_pointer].adr = self.code_pointer;
        self.name_table[self.table_pointer].size = data_pointer;

        // Begin statement
        self.code[self.code_pointer] = vm::Instruction {
            f: vm::Fct::Inte, 
            l: 0,
            a: data_pointer,
        };
        // Statement
        self.parse_statement(level, lexer);
        // Should end with end/semicolon
        self.code[self.code_pointer] = vm::Instruction {
            f: vm::Fct::Opr,
            l: 0,
            a: 0,
        };
        // End statement
    }

    fn add_into_name_table(&mut self, identity: &str, num: i64, k: nametab::NameTableObject, level: usize, pdx: usize) {
        self.table_pointer += 1;
        self.name_table[self.table_pointer] = match k {
            nametab::NameTableObject::Constant => {
                nametab::NameTableItem {
                    name: String::from(identity),
                    kind: k,
                    val: num,
                    level: 0,
                    adr: 0,
                    size: 0,
                }
            },
            nametab::NameTableObject::Variable => {
                nametab::NameTableItem {
                    name: String::from(identity),
                    kind: k,
                    val: 0,
                    level: level,
                    adr: pdx,
                    size: 0,
                }
            },
            nametab::NameTableObject::Procedur => {
                nametab::NameTableItem {
                    name: String::from(identity),
                    kind: k,
                    val: 0,
                    level: level,
                    adr: 0,
                    size: 0,
                }
            },
        };
        
    }

    fn parse_statement(&mut self, level: usize, lexer: &mut symbol::io::PL0Lexer) {
        
    }

    fn find_variable(&self, name: &str, tail: usize) -> usize {
        1
    }

    fn parse_expression(&mut self, level: usize, lexer: &mut symbol::io::PL0Lexer) {
        
    }

    fn parse_term(&mut self, level: usize, lexer: &mut symbol::io::PL0Lexer) {
        
    }

    fn parse_factor(&mut self, level: usize, lexer: &mut symbol::io::PL0Lexer) {
        
    }

    fn parse_condition(&mut self, level: usize, lexer: &mut symbol::io::PL0Lexer) {
        
    }
}
