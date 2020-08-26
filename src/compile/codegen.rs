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

    pub fn gen(&self, opcode: vm::Fct, level: usize, extra: usize) -> vm::Instruction {
        vm::Instruction {
            f: opcode,
            l: level,
            a: extra,
        }
    }

    pub fn build_block(&mut self, level: usize, lexer: &mut symbol::io::PL0Lexer) {
        // Create anonymous main procedure
        self.add_into_name_table("_main", 0, nametab::NameTableObject::Procedur, 0, 0);

        {
            self.block(0, lexer);
        }

        let sym = lexer.current();
        if *sym == symbol::Symbol::Period {
            println!("Parsing finished");
        } else {
            println!("Parsing Failed {:?}", sym);
        }
    }

    pub fn block(&mut self, level: usize, lexer: &mut symbol::io::PL0Lexer) {
        let table_pointer_0 = self.table_pointer;

        println!("=============== Level {} ===============", level);
        // Add jump to code
        self.code_pointer += 1;
        self.code.push(self.gen(vm::Fct::Jmp, 0, 0));

        // Set procedur begin pos
        self.name_table[self.table_pointer - 1].adr = self.code_pointer - 1;

        let mut data_pointer: usize = 0;    // Count data size in this block (single level, no deeper)
        {
            lexer.next();
        }
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
                                data_pointer += 1;
                            }
                        }
                        let symbol = lexer.next();
                        if *symbol == symbol::Symbol::Semicolon {
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
                            data_pointer += 1;
                        }
                        let symbol = lexer.next();
                        if *symbol == symbol::Symbol::Semicolon {
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
                        data_pointer += 1;
                    }
                    if should_continue {
                        // semicolon
                        let symbol = lexer.next();

                        if *symbol != symbol::Symbol::Semicolon {
                            panic!("Procedur is not ended with ;");
                        }
                    }
                    // Enter the next level
                    self.block(level + 1, lexer);
   
                    if *lexer.current() != symbol::Symbol::Semicolon {
                        panic!("Block is not ended with ;");
                    }
                    // TODO: add some rescue solution
                },
                _ => {
                    // Others should not be here...
                    break;
                },
            }

            {
                lexer.next();
            }

            if *lexer.current() != symbol::Symbol::Constsym
                && *lexer.current() != symbol::Symbol::Varsym
                && *lexer.current() != symbol::Symbol::Procsym {
                // End of declaration
                break;
            }
        }

        // Generate current block
        self.code[self.name_table[table_pointer_0 - 1].adr].a = self.code_pointer;
        self.name_table[table_pointer_0 - 1].adr = self.code_pointer;
        self.name_table[table_pointer_0 - 1].size = data_pointer;

        // Record current code pointer pos
        // let code_pointer_0 = self.code_pointer;

        // Begin statement
        self.code_pointer += 1;
        self.code.push(self.gen(vm::Fct::Inte, 0, data_pointer));

        // Statement
        self.parse_statement(level, lexer);
        // Should end with end/semicolon
        self.code_pointer += 1;
        self.code.push(self.gen(vm::Fct::Opr, 0, 0));
        // End statement

        println!("=============== Level {} End ===============", level);
        println!("{} codes", self.code_pointer);
    }

    fn add_into_name_table(&mut self, identity: &str, num: i64, k: nametab::NameTableObject, level: usize, pdx: usize) {
        self.table_pointer += 1;
        self.name_table.push(match k {
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
        });
        
    }

    fn parse_statement(&mut self, level: usize, lexer: &mut symbol::io::PL0Lexer) {
        if *lexer.current() == symbol::Symbol::Semicolon {
            // Get the next symbol if upper level doesn't do that
            lexer.next();
        }
        println!("Parsing statement starting with {:?}", lexer.current());
        match *lexer.current() {
            symbol::Symbol::Ident => {
                // Handle as a assignment statement
                println!("=> Got ident: {}", lexer.current_content());

                // Get the index of identifier
                let identifier_index: usize = self.find_variable(lexer.current_content(), self.table_pointer);
                let mut should_continue = true;
                
                if identifier_index == 0 {
                    should_continue = false;
                }

                if should_continue {
                    // Detect Becomes symbol
                    if *lexer.next() != symbol::Symbol::Becomes {
                        should_continue = false;
                    }
                }

                if should_continue {
                    // Expression
                    self.parse_expression(level, lexer);

                    // Store the result in the variable
                    self.code_pointer += 1;
                    self.code.push(self.gen(
                        vm::Fct::Sto,
                        level - self.name_table[identifier_index - 1].level,
                        self.name_table[identifier_index - 1].adr
                    ));
                }
            },
            symbol::Symbol::Readsym => {
                // read()
                let mut should_continue = true;

                {
                    if *lexer.next() != symbol::Symbol::Lparen {
                        should_continue = false;
                    }
                }

                if should_continue {
                    loop {
                        let mut identifier_index: usize = 0;
                        {
                            // Get the next symbol
                            lexer.next();
                        }

                        if *lexer.current() != symbol::Symbol::Ident {
                            should_continue = false;
                        }

                        if should_continue {
                            // Get the index of identifier
                            identifier_index = self.find_variable(lexer.current_content(), self.table_pointer);
                        }

                        if identifier_index == 0{
                            should_continue = false;
                        }

                        if should_continue {
                            self.code_pointer += 2;
                            // Read content to the stack top
                            self.code.push(self.gen(vm::Fct::Opr, 0, 16));
                            // Store the result in the variable
                            self.code.push(self.gen(
                                vm::Fct::Sto,
                                level - self.name_table[identifier_index].level,
                                self.name_table[identifier_index].adr
                            ));
                        }

                        if *lexer.next() != symbol::Symbol::Comma {
                            break;
                        }
                    }
                }

                if *lexer.next() != symbol::Symbol::Rparen {
                    panic!("Read statement should end with Rparent )");
                }
            },
            symbol::Symbol::Writesym => {
                // write()
                let mut should_continue = true;

                {
                    if *lexer.next() != symbol::Symbol::Lparen {
                        should_continue = false;
                    }
                }

                if should_continue {
                    loop {
                        if should_continue {
                            self.parse_expression(level, lexer);
                        }

                        if should_continue {
                            self.code_pointer += 1;
                            // Write content on the stack top
                            self.code.push(self.gen(vm::Fct::Opr, 0, 14));
                        }

                        if *lexer.current() != symbol::Symbol::Comma {
                            break;
                        }
                    }
                }

                if *lexer.current() != symbol::Symbol::Rparen {
                    panic!("Write statement should end with Rparent )")
                }

                if should_continue {
                    self.code_pointer += 1;
                    // New line
                    self.code.push(self.gen(vm::Fct::Opr, 0, 15));
                }
            },
            symbol::Symbol::Callsym => {
                // Call a function
                let mut should_continue = true;
                let mut index = 0;
                {
                    // Get the next symbol
                    lexer.next();
                }

                if *lexer.current() != symbol::Symbol::Ident {
                    should_continue = false;
                }

                if should_continue {
                    // Get the index of identifier
                    index = self.find_variable(lexer.current_content(), self.table_pointer);
                }

                if index == 0 {
                    should_continue = false;
                }

                if self.name_table[index - 1].kind == nametab::NameTableObject::Procedur {
                    self.code_pointer += 1;
                    self.code.push(self.gen(vm::Fct::Cal, 
                        level - self.name_table[index - 1].level, self.name_table[index - 1].adr));
                }
            },
            symbol::Symbol::Ifsym => {
                let mut should_continue = true;
                self.parse_condition(level, lexer);
                {
                    // Get the next symbol
                    lexer.next();
                }

                {
                    if *lexer.current() != symbol::Symbol::Thensyn {
                        should_continue = false;
                    }
                }

                if should_continue {
                    let cx1 = self.code_pointer;

                    // Generate Jump before parse statement
                    self.code_pointer += 1;
                    self.code.push(self.gen(vm::Fct::Jpc, 0, 0));

                    self.parse_statement(level, lexer);

                    // Modify the jump address
                    self.code[cx1].a = self.code_pointer;
                }
            },
            symbol::Symbol::Beginsym => {
                let mut should_continue = false;
                {
                    lexer.next();
                }

                self.parse_statement(level, lexer);

                loop {
                    if *lexer.current() != symbol::Symbol::Semicolon && *lexer.current() != symbol::Symbol::Endsym {
                        println!("- Semicolon not in Begin Stmt ;Stmt End {:?}",  lexer.current());
                        lexer.next();
                        println!("- Semicolon not in Begin Stmt ;Stmt End {:?}",  lexer.current());
                    }
                    if *lexer.current() == symbol::Symbol::Endsym || *lexer.current() == symbol::Symbol::Period {
                        if *lexer.current() == symbol::Symbol::Endsym {
                            lexer.next();
                        }
                        println!("Begin end {}", level);
                        break;
                    }
                    if *lexer.current() != symbol::Symbol::Semicolon {
                        panic!("Statement is not ended with ;, {:?}", lexer.current());
                    }

                    self.parse_statement(level, lexer);
                }
                if lexer.previous() != symbol::Symbol::Endsym &&
                    *lexer.current() != symbol::Symbol::Period {
                    panic!("Begin should end with Endsym");
                }
            },
            symbol::Symbol::Whilesym => {
                let mut should_continue = true;

                let cx1 = self.code_pointer;    // condition pos

                self.parse_condition(level, lexer);

                let cx2 = self.code_pointer;    // loop end pos

                // Generate Jump before parse statement
                self.code_pointer += 1;
                self.code.push(self.gen(vm::Fct::Jpc, 0, 0));
                if *lexer.next() != symbol::Symbol::Dosym {
                    should_continue = false;
                }

                if should_continue {
                    {
                        lexer.next();
                    }
                    self.parse_statement(level, lexer);
                    self.code_pointer += 1;
                    self.code.push(self.gen(vm::Fct::Jpc, 0, cx1));  // Jump to condition
                    self.code[cx1].a = self.code_pointer;
                }
            },
            _ => {
                // I cannot handle the sym
            },
        }

        println!("Statement parsed ok {:?}", lexer.current());
    }

    fn find_variable(&self, name: &str, tail: usize) -> usize {
        let mut pointer = tail;
        if pointer > self.name_table.len() {
            return 0;
        }
        loop {
            if pointer <= 0 || self.name_table[pointer - 1].name == name {
                break;
            }
            pointer -= 1;
        }
        pointer
    }

    fn parse_expression(&mut self, level: usize, lexer: &mut symbol::io::PL0Lexer) {
        let mut is_positive = true;

        {
            // Parse a term
            is_positive = self.parse_term(level, lexer);
        }

        if !is_positive {
            // Negative
            self.code_pointer += 1;
            self.code.push(self.gen(vm::Fct::Opr, 0, 1));
        }

        loop {
            /*if *lexer.current() == symbol::Symbol::Minus {
                is_positive = false;
            } else if *lexer.current() == symbol::Symbol::Plus {
                is_positive = true;
            }*/
            if *lexer.current() != symbol::Symbol::Minus && *lexer.current() != symbol::Symbol::Plus {
                break;
            }

            is_positive = self.parse_term(level, lexer);

            self.code_pointer += 1;
            if is_positive {
                self.code.push(self.gen(vm::Fct::Opr, 0, 2));
            } else {
                self.code.push(self.gen(vm::Fct::Opr, 0, 3));
            }
        }


    }

    fn parse_term(&mut self, level: usize, lexer: &mut symbol::io::PL0Lexer) -> bool {
        let mut is_positive = self.parse_factor(level, lexer);
        println!("Starting term {:?} {}", lexer.current(), lexer.current_content());
        loop {
            let mut is_time = false;
            let mut is_slash = false;

            {
                lexer.next();
            }

            match *lexer.current() {
                symbol::Symbol::Times => {
                    is_time = true;
                },
                symbol::Symbol::Slash => {
                    is_slash = true;
                },
                _ => {
                    // Nothing
                    // panic!("There should be * or / between terms");
                    break;
                },
            }

            self.parse_factor(level, lexer);

            if is_time {
                self.code_pointer += 1;
                self.code.push(self.gen(vm::Fct::Opr, 0, 4));
            } else if is_slash {
                self.code_pointer += 1;
                self.code.push(self.gen(vm::Fct::Opr, 0, 5));
            }
        }

        is_positive
    }

    fn parse_factor(&mut self, level: usize, lexer: &mut symbol::io::PL0Lexer) -> bool {
        // Handle factor
        let mut is_positive = true;
        {
            lexer.next();
        }

        if lexer.previous() == symbol::Symbol::Minus {
            is_positive = false;
        } else if *lexer.current() == symbol::Symbol::Minus {
            is_positive = false;
            lexer.next();
        }
        match &lexer.current() {
            symbol::Symbol::Ident => {
                let mut should_continue = true;
                // Get the name
                let index = self.find_variable(lexer.current_content(), self.table_pointer);

                if index == 0 {
                    should_continue = false;
                }

                if should_continue {
                    match self.name_table[index - 1].kind {
                        nametab::NameTableObject::Constant => {
                            self.code_pointer += 1;
                            self.code.push(self.gen(vm::Fct::Lit, 0,
                                self.name_table[index - 1].val as usize));
                        },
                        nametab::NameTableObject::Variable => {
                            println!("Lod {} {} {} {}", self.name_table[index - 1].name, index, level, self.name_table[index - 1].level);
                            self.code_pointer += 1;
                            self.code.push(self.gen(vm::Fct::Lod,
                                level - self.name_table[index - 1].level,
                                self.name_table[index - 1].val as usize));
                        },
                        _ => {
                            // Error, should not be a procedur
                        },
                    }
                }
            },
            symbol::Symbol::Number => {
                // Number
                self.code_pointer += 1;
                // parse i64 as usize
                self.code.push(self.gen(vm::Fct::Lit, 0,
                    lexer.current_content().parse::<usize>().unwrap()));
            },
            symbol::Symbol::Lparen => {
                // Left parent
                self.parse_expression(level, lexer);

                {
                    lexer.next();
                }

                if *lexer.current() != symbol::Symbol::Rparen {
                    panic!("Expression factor should be Rparen )");
                }
            },
            _ => {
                // Nothing to do
                // jump out
            },
        }
        is_positive
    }

    fn parse_condition(&mut self, level: usize, lexer: &mut symbol::io::PL0Lexer) {
        let mut should_continue = true;

        {
            lexer.next();
        }

        if *lexer.current() == symbol::Symbol::Oddsym {
            self.parse_expression(level, lexer);
            self.code_pointer += 1;
            self.code.push(self.gen(vm::Fct::Opr, 0, 6));
        } else {
            self.parse_expression(level, lexer);
            match *lexer.current() {
                symbol::Symbol::Eql => {
                    self.parse_expression(level, lexer);
                    self.code_pointer += 1;
                    self.code.push(self.gen(vm::Fct::Opr, 0, 8));
                },
                symbol::Symbol::Neq => {
                    self.parse_expression(level, lexer);
                    self.code_pointer += 1;
                    self.code.push(self.gen(vm::Fct::Opr, 0, 9));
                },
                symbol::Symbol::Lss => {
                    self.parse_expression(level, lexer);
                    self.code_pointer += 1;
                    self.code.push(self.gen(vm::Fct::Opr, 0, 10));
                },
                symbol::Symbol::Geq => {
                    self.parse_expression(level, lexer);
                    self.code_pointer += 1;
                    self.code.push(self.gen(vm::Fct::Opr, 0, 11));
                },
                symbol::Symbol::Gtr => {
                    self.parse_expression(level, lexer);
                    self.code_pointer += 1;
                    self.code.push(self.gen(vm::Fct::Opr, 0, 12));
                },
                symbol::Symbol::Leq => {
                    self.parse_expression(level, lexer);
                    self.code_pointer += 1;
                    self.code.push(self.gen(vm::Fct::Opr, 0, 13));
                },
                _ => {
                    // I will not handle it
                },
            }
        }
    }

    pub fn get_vm_code(&self) -> &Vec<vm::Instruction> {
        &self.code
    }
}

#[cfg(test)]
mod tests {
    use crate::compile::nametab;
    use crate::compile::codegen;

    #[test]
    fn test_add_into_name_table() {
        let mut generator = codegen::CodeGenerator::new();
        generator.add_into_name_table("const_1", 10, nametab::NameTableObject::Constant, 0, 0);
        generator.add_into_name_table("var_1", 20, nametab::NameTableObject::Variable, 0, 0);
        generator.add_into_name_table("func_1", 0, nametab::NameTableObject::Procedur, 0, 0);

        assert_eq!(generator.name_table.len(), 3);
        assert_eq!(generator.name_table[0].name, "const_1");
        assert!(generator.name_table[0].kind == nametab::NameTableObject::Constant);

        assert_eq!(generator.name_table[1].name, "var_1");
        assert!(generator.name_table[1].kind == nametab::NameTableObject::Variable);

        assert_eq!(generator.name_table[2].name, "func_1");
        assert!(generator.name_table[2].kind == nametab::NameTableObject::Procedur);
    }

    #[test]
    fn test_find_variable_no_duplicated_name() {
        let mut generator = codegen::CodeGenerator::new();

        generator.add_into_name_table("const_1", 10, nametab::NameTableObject::Constant, 0, 0);
        generator.add_into_name_table("const_2", 10, nametab::NameTableObject::Constant, 0, 0);
        generator.add_into_name_table("const_3", 10, nametab::NameTableObject::Constant, 0, 0);

        assert_eq!(generator.find_variable("const_3", generator.name_table.len() + 1), 0);
        assert_eq!(generator.find_variable("const_1", generator.name_table.len()), 1);
        assert_eq!(generator.find_variable("const_2", generator.name_table.len()), 2);
        assert_eq!(generator.find_variable("const_3", generator.name_table.len()), 3);
    }

    #[test]
    fn test_find_variable_with_duplicated_name() {
        let mut generator = codegen::CodeGenerator::new();

        generator.add_into_name_table("const_1", 10, nametab::NameTableObject::Constant, 0, 0);
        generator.add_into_name_table("const_1", 20, nametab::NameTableObject::Constant, 1, 0);
        generator.add_into_name_table("const_1", 30, nametab::NameTableObject::Constant, 2, 0);

        assert_eq!(generator.find_variable("const_1", 1), 1);
        assert_eq!(generator.find_variable("const_1", 2), 2);
        assert_eq!(generator.find_variable("const_1", 3), 3);
    }

    #[test]
    fn test_find_variable_not_found() {
        let mut generator = codegen::CodeGenerator::new();

        generator.add_into_name_table("const_1", 10, nametab::NameTableObject::Constant, 0, 0);
        generator.add_into_name_table("const_2", 10, nametab::NameTableObject::Constant, 0, 0);
        generator.add_into_name_table("const_3", 10, nametab::NameTableObject::Constant, 0, 0);

        assert_eq!(generator.find_variable("const_3", generator.name_table.len() + 1), 0);
        assert_eq!(generator.find_variable("const_4", generator.name_table.len()), 0);
    }

    /* ================================================ */
    /* --------------- test Code Generator ------------ */
    /* ================================================ */
    use crate::symbol;
    use crate::vm;

    /* test number factor*/
    #[test]
    fn test_simple_number_factor() {
        let mut lex: symbol::io::PL0Lexer =
            symbol::io::PL0Lexer::create_from_content("123");
        let mut generator = codegen::CodeGenerator::new();

        generator.parse_factor(0, &mut lex);

        assert_eq!(generator.code_pointer, 1);
        assert_eq!(generator.code[0].f, vm::Fct::Lit);
        assert_eq!(generator.code[0].l, 0);
        assert_eq!(generator.code[0].a, 123);
    }

    /* test const ident factor */
    #[test]
    fn test_simple_const_ident_factor() {
        let mut lex: symbol::io::PL0Lexer =
            symbol::io::PL0Lexer::create_from_content("abc");
        let mut generator = codegen::CodeGenerator::new();

        generator.add_into_name_table("abc", 10, nametab::NameTableObject::Constant, 0, 0);

        generator.parse_factor(0, &mut lex);

        assert_eq!(generator.code_pointer, 1);
        assert_eq!(generator.code[0].f, vm::Fct::Lit);
        assert_eq!(generator.code[0].l, 0);
        assert_eq!(generator.code[0].a, 10);
    }

    /* test var ident factor */
    #[test]
    fn test_simple_var_ident_factor() {
        let mut lex: symbol::io::PL0Lexer =
            symbol::io::PL0Lexer::create_from_content("abc");
        let mut generator = codegen::CodeGenerator::new();

        // the value 10 will not take effect
        generator.add_into_name_table("abc", 10, nametab::NameTableObject::Variable, 0, 0);

        generator.parse_factor(0, &mut lex);

        assert_eq!(generator.code_pointer, 1);
        assert_eq!(generator.code[0].f, vm::Fct::Lod);
        assert_eq!(generator.code[0].l, 0);
        assert_eq!(generator.code[0].a, 0);
    }

    /* TODO: test procedur ident factor, should panic */
    #[test]
    #[ignore]
    fn test_simple_var_procedur_factor() {
        // not ready for test
    }

    /* TODO: test Lparent factor */
    #[test]
    #[ignore]
    fn test_simple_var_lparent_factor() {
        // not ready for test
    }

    /* test single term */
    #[test]
    fn test_single_term() {
        let mut lex: symbol::io::PL0Lexer =
            symbol::io::PL0Lexer::create_from_content("8");
        let mut generator = codegen::CodeGenerator::new();

        generator.parse_term(0, &mut lex);

        assert_eq!(generator.code_pointer, 1);
        assert_eq!(generator.code[0].f, vm::Fct::Lit);
        assert_eq!(generator.code[0].l, 0);
        assert_eq!(generator.code[0].a, 8);
    }

    /* test constant production term */
    #[test]
    fn test_constant_production_term() {
        let mut lex: symbol::io::PL0Lexer =
            symbol::io::PL0Lexer::create_from_content("8 * 9");
        let mut generator = codegen::CodeGenerator::new();

        generator.parse_term(0, &mut lex);

        assert_eq!(generator.code_pointer, 3);
        assert_eq!(generator.code[0].f, vm::Fct::Lit);
        assert_eq!(generator.code[0].l, 0);
        assert_eq!(generator.code[0].a, 8);
        assert_eq!(generator.code[1].f, vm::Fct::Lit);
        assert_eq!(generator.code[1].l, 0);
        assert_eq!(generator.code[1].a, 9);
        assert_eq!(generator.code[2].f, vm::Fct::Opr);
        assert_eq!(generator.code[2].l, 0);
        assert_eq!(generator.code[2].a, 4);
    }

    /* test consecutive constant production term */
    #[test]
    fn test_consecutive_constant_production_term() {
        let mut lex: symbol::io::PL0Lexer =
            symbol::io::PL0Lexer::create_from_content("8 * 9 * 10");
        let mut generator = codegen::CodeGenerator::new();

        generator.parse_term(0, &mut lex);

        assert_eq!(generator.code_pointer, 5);
        assert_eq!(generator.code[0].f, vm::Fct::Lit);
        assert_eq!(generator.code[0].l, 0);
        assert_eq!(generator.code[0].a, 8);
        assert_eq!(generator.code[1].f, vm::Fct::Lit);
        assert_eq!(generator.code[1].l, 0);
        assert_eq!(generator.code[1].a, 9);
        assert_eq!(generator.code[2].f, vm::Fct::Opr);
        assert_eq!(generator.code[2].l, 0);
        assert_eq!(generator.code[2].a, 4);
        assert_eq!(generator.code[3].f, vm::Fct::Lit);
        assert_eq!(generator.code[3].l, 0);
        assert_eq!(generator.code[3].a, 10);
        assert_eq!(generator.code[4].f, vm::Fct::Opr);
        assert_eq!(generator.code[4].l, 0);
        assert_eq!(generator.code[4].a, 4);
    }

    /* test constant division term */
    #[test]
    fn test_constant_division_term() {
        let mut lex: symbol::io::PL0Lexer =
            symbol::io::PL0Lexer::create_from_content("18 / 9");
        let mut generator = codegen::CodeGenerator::new();

        generator.parse_term(0, &mut lex);

        assert_eq!(generator.code_pointer, 3);
        assert_eq!(generator.code[0].f, vm::Fct::Lit);
        assert_eq!(generator.code[0].l, 0);
        assert_eq!(generator.code[0].a, 18);
        assert_eq!(generator.code[1].f, vm::Fct::Lit);
        assert_eq!(generator.code[1].l, 0);
        assert_eq!(generator.code[1].a, 9);
        assert_eq!(generator.code[2].f, vm::Fct::Opr);
        assert_eq!(generator.code[2].l, 0);
        assert_eq!(generator.code[2].a, 5);
    }

    /* test consecutive constant production term */
    #[test]
    fn test_consecutive_constant_division_term() {
        let mut lex: symbol::io::PL0Lexer =
            symbol::io::PL0Lexer::create_from_content("18 / 9 / 2");
        let mut generator = codegen::CodeGenerator::new();

        generator.parse_term(0, &mut lex);

        assert_eq!(generator.code_pointer, 5);
        assert_eq!(generator.code[0].f, vm::Fct::Lit);
        assert_eq!(generator.code[0].l, 0);
        assert_eq!(generator.code[0].a, 18);
        assert_eq!(generator.code[1].f, vm::Fct::Lit);
        assert_eq!(generator.code[1].l, 0);
        assert_eq!(generator.code[1].a, 9);
        assert_eq!(generator.code[2].f, vm::Fct::Opr);
        assert_eq!(generator.code[2].l, 0);
        assert_eq!(generator.code[2].a, 5);
        assert_eq!(generator.code[3].f, vm::Fct::Lit);
        assert_eq!(generator.code[3].l, 0);
        assert_eq!(generator.code[3].a, 2);
        assert_eq!(generator.code[4].f, vm::Fct::Opr);
        assert_eq!(generator.code[4].l, 0);
        assert_eq!(generator.code[4].a, 5);
    }

    /* test simple plus expression */
    #[test]
    fn test_simple_plus_expression() {
        let mut lex: symbol::io::PL0Lexer =
            symbol::io::PL0Lexer::create_from_content("18 / 9 + 2");
        let mut generator = codegen::CodeGenerator::new();

        generator.parse_expression(0, &mut lex);

        /* 18, 9, /, 2, +, */
        assert_eq!(generator.code_pointer, 5);
        assert_eq!(generator.code[0].f, vm::Fct::Lit);
        assert_eq!(generator.code[0].l, 0);
        assert_eq!(generator.code[0].a, 18);
        assert_eq!(generator.code[1].f, vm::Fct::Lit);
        assert_eq!(generator.code[1].l, 0);
        assert_eq!(generator.code[1].a, 9);
        assert_eq!(generator.code[2].f, vm::Fct::Opr);
        assert_eq!(generator.code[2].l, 0);
        assert_eq!(generator.code[2].a, 5);
        assert_eq!(generator.code[3].f, vm::Fct::Lit);
        assert_eq!(generator.code[3].l, 0);
        assert_eq!(generator.code[3].a, 2);
        assert_eq!(generator.code[4].f, vm::Fct::Opr);
        assert_eq!(generator.code[4].l, 0);
        assert_eq!(generator.code[4].a, 2);
    }

    /* test simple expression */
    #[test]
    fn test_simple_expression() {
        let mut lex: symbol::io::PL0Lexer =
            symbol::io::PL0Lexer::create_from_content("2");
        let mut generator = codegen::CodeGenerator::new();

        generator.parse_expression(0, &mut lex);

        /* 2 */
        assert_eq!(generator.code_pointer, 1);
        assert_eq!(generator.code[0].f, vm::Fct::Lit);
        assert_eq!(generator.code[0].l, 0);
        assert_eq!(generator.code[0].a, 2);
    }

    /* test simple minus expression */
    #[test]
    fn test_simple_minus_expression() {
        let mut lex: symbol::io::PL0Lexer =
            symbol::io::PL0Lexer::create_from_content("18 / 9 - 2");
        let mut generator = codegen::CodeGenerator::new();

        generator.parse_expression(0, &mut lex);

        /* 18, 9, /, 2, -, */
        assert_eq!(generator.code_pointer, 5);
        assert_eq!(generator.code[0].f, vm::Fct::Lit);
        assert_eq!(generator.code[0].l, 0);
        assert_eq!(generator.code[0].a, 18);
        assert_eq!(generator.code[1].f, vm::Fct::Lit);
        assert_eq!(generator.code[1].l, 0);
        assert_eq!(generator.code[1].a, 9);
        assert_eq!(generator.code[2].f, vm::Fct::Opr);
        assert_eq!(generator.code[2].l, 0);
        assert_eq!(generator.code[2].a, 5);
        assert_eq!(generator.code[3].f, vm::Fct::Lit);
        assert_eq!(generator.code[3].l, 0);
        assert_eq!(generator.code[3].a, 2);
        assert_eq!(generator.code[4].f, vm::Fct::Opr);
        assert_eq!(generator.code[4].l, 0);
        assert_eq!(generator.code[4].a, 3);
    }

    /* test negative minus expression */
    #[test]
    fn test_negative_minus_expression() {
        let mut lex: symbol::io::PL0Lexer =
            symbol::io::PL0Lexer::create_from_content("- 18 / 9 - 2");
        let mut generator = codegen::CodeGenerator::new();

        generator.parse_expression(0, &mut lex);

        /* 18, 9, /, -, 2, -, */
        assert_eq!(generator.code_pointer, 6);
        assert_eq!(generator.code[0].f, vm::Fct::Lit);
        assert_eq!(generator.code[0].l, 0);
        assert_eq!(generator.code[0].a, 18);
        assert_eq!(generator.code[1].f, vm::Fct::Lit);
        assert_eq!(generator.code[1].l, 0);
        assert_eq!(generator.code[1].a, 9);
        assert_eq!(generator.code[2].f, vm::Fct::Opr);
        assert_eq!(generator.code[2].l, 0);
        assert_eq!(generator.code[2].a, 5);
        assert_eq!(generator.code[3].f, vm::Fct::Opr);
        assert_eq!(generator.code[3].l, 0);
        assert_eq!(generator.code[3].a, 1);
        assert_eq!(generator.code[4].f, vm::Fct::Lit);
        assert_eq!(generator.code[4].l, 0);
        assert_eq!(generator.code[4].a, 2);
        assert_eq!(generator.code[5].f, vm::Fct::Opr);
        assert_eq!(generator.code[5].l, 0);
        assert_eq!(generator.code[5].a, 3);
    }

    /* test odd condition */
    #[test]
    fn test_condition_odd() {
        let mut lex: symbol::io::PL0Lexer =
            symbol::io::PL0Lexer::create_from_content("odd 1 + 2");
        let mut generator = codegen::CodeGenerator::new();

        generator.parse_condition(0, &mut lex);

        assert_eq!(generator.code_pointer, 4);
        assert_eq!(generator.code[0].f, vm::Fct::Lit);
        assert_eq!(generator.code[0].l, 0);
        assert_eq!(generator.code[0].a, 1);
        assert_eq!(generator.code[1].f, vm::Fct::Lit);
        assert_eq!(generator.code[1].l, 0);
        assert_eq!(generator.code[1].a, 2);
        assert_eq!(generator.code[2].f, vm::Fct::Opr);
        assert_eq!(generator.code[2].l, 0);
        assert_eq!(generator.code[2].a, 2);
        assert_eq!(generator.code[3].f, vm::Fct::Opr);
        assert_eq!(generator.code[3].l, 0);
        assert_eq!(generator.code[3].a, 6);
    }

    /* test equal condition */
    #[test]
    fn test_condition_eql() {
        let mut lex: symbol::io::PL0Lexer =
            symbol::io::PL0Lexer::create_from_content("= 1 + 2");
        let mut generator = codegen::CodeGenerator::new();

        generator.parse_condition(0, &mut lex);

        assert_eq!(generator.code_pointer, 4);
        assert_eq!(generator.code[0].f, vm::Fct::Lit);
        assert_eq!(generator.code[0].l, 0);
        assert_eq!(generator.code[0].a, 1);
        assert_eq!(generator.code[1].f, vm::Fct::Lit);
        assert_eq!(generator.code[1].l, 0);
        assert_eq!(generator.code[1].a, 2);
        assert_eq!(generator.code[2].f, vm::Fct::Opr);
        assert_eq!(generator.code[2].l, 0);
        assert_eq!(generator.code[2].a, 2);
        assert_eq!(generator.code[3].f, vm::Fct::Opr);
        assert_eq!(generator.code[3].l, 0);
        assert_eq!(generator.code[3].a, 8);
    }

    /* test not equal condition */
    #[test]
    fn test_condition_neq() {
        let mut lex: symbol::io::PL0Lexer =
            symbol::io::PL0Lexer::create_from_content("!= 1 + 2");
        let mut generator = codegen::CodeGenerator::new();

        generator.parse_condition(0, &mut lex);

        assert_eq!(generator.code_pointer, 4);
        assert_eq!(generator.code[0].f, vm::Fct::Lit);
        assert_eq!(generator.code[0].l, 0);
        assert_eq!(generator.code[0].a, 1);
        assert_eq!(generator.code[1].f, vm::Fct::Lit);
        assert_eq!(generator.code[1].l, 0);
        assert_eq!(generator.code[1].a, 2);
        assert_eq!(generator.code[2].f, vm::Fct::Opr);
        assert_eq!(generator.code[2].l, 0);
        assert_eq!(generator.code[2].a, 2);
        assert_eq!(generator.code[3].f, vm::Fct::Opr);
        assert_eq!(generator.code[3].l, 0);
        assert_eq!(generator.code[3].a, 9);
    }

    /* test less condition */
    #[test]
    fn test_condition_lss() {
        let mut lex: symbol::io::PL0Lexer =
            symbol::io::PL0Lexer::create_from_content("< 1 + 2");
        let mut generator = codegen::CodeGenerator::new();

        generator.parse_condition(0, &mut lex);

        assert_eq!(generator.code_pointer, 4);
        assert_eq!(generator.code[0].f, vm::Fct::Lit);
        assert_eq!(generator.code[0].l, 0);
        assert_eq!(generator.code[0].a, 1);
        assert_eq!(generator.code[1].f, vm::Fct::Lit);
        assert_eq!(generator.code[1].l, 0);
        assert_eq!(generator.code[1].a, 2);
        assert_eq!(generator.code[2].f, vm::Fct::Opr);
        assert_eq!(generator.code[2].l, 0);
        assert_eq!(generator.code[2].a, 2);
        assert_eq!(generator.code[3].f, vm::Fct::Opr);
        assert_eq!(generator.code[3].l, 0);
        assert_eq!(generator.code[3].a, 10);
    }

    /* test greater or equal condition */
    #[test]
    fn test_condition_geq() {
        let mut lex: symbol::io::PL0Lexer =
            symbol::io::PL0Lexer::create_from_content(">= 1 + 2");
        let mut generator = codegen::CodeGenerator::new();

        generator.parse_condition(0, &mut lex);

        assert_eq!(generator.code_pointer, 4);
        assert_eq!(generator.code[0].f, vm::Fct::Lit);
        assert_eq!(generator.code[0].l, 0);
        assert_eq!(generator.code[0].a, 1);
        assert_eq!(generator.code[1].f, vm::Fct::Lit);
        assert_eq!(generator.code[1].l, 0);
        assert_eq!(generator.code[1].a, 2);
        assert_eq!(generator.code[2].f, vm::Fct::Opr);
        assert_eq!(generator.code[2].l, 0);
        assert_eq!(generator.code[2].a, 2);
        assert_eq!(generator.code[3].f, vm::Fct::Opr);
        assert_eq!(generator.code[3].l, 0);
        assert_eq!(generator.code[3].a, 11);
    }

    /* test greater than condition */
    #[test]
    fn test_condition_gtr() {
        let mut lex: symbol::io::PL0Lexer =
            symbol::io::PL0Lexer::create_from_content("> 1 + 2");
        let mut generator = codegen::CodeGenerator::new();

        generator.parse_condition(0, &mut lex);

        assert_eq!(generator.code_pointer, 4);
        assert_eq!(generator.code[0].f, vm::Fct::Lit);
        assert_eq!(generator.code[0].l, 0);
        assert_eq!(generator.code[0].a, 1);
        assert_eq!(generator.code[1].f, vm::Fct::Lit);
        assert_eq!(generator.code[1].l, 0);
        assert_eq!(generator.code[1].a, 2);
        assert_eq!(generator.code[2].f, vm::Fct::Opr);
        assert_eq!(generator.code[2].l, 0);
        assert_eq!(generator.code[2].a, 2);
        assert_eq!(generator.code[3].f, vm::Fct::Opr);
        assert_eq!(generator.code[3].l, 0);
        assert_eq!(generator.code[3].a, 12);
    }

    /* test less or equal condition */
    #[test]
    fn test_condition_leq() {
        let mut lex: symbol::io::PL0Lexer =
            symbol::io::PL0Lexer::create_from_content("<= 1 + 2");
        let mut generator = codegen::CodeGenerator::new();

        generator.parse_condition(0, &mut lex);

        assert_eq!(generator.code_pointer, 4);
        assert_eq!(generator.code[0].f, vm::Fct::Lit);
        assert_eq!(generator.code[0].l, 0);
        assert_eq!(generator.code[0].a, 1);
        assert_eq!(generator.code[1].f, vm::Fct::Lit);
        assert_eq!(generator.code[1].l, 0);
        assert_eq!(generator.code[1].a, 2);
        assert_eq!(generator.code[2].f, vm::Fct::Opr);
        assert_eq!(generator.code[2].l, 0);
        assert_eq!(generator.code[2].a, 2);
        assert_eq!(generator.code[3].f, vm::Fct::Opr);
        assert_eq!(generator.code[3].l, 0);
        assert_eq!(generator.code[3].a, 13);
    }

    /* test simple write statement */
    #[test]
    fn test_simple_write_statement() {
        let mut lex: symbol::io::PL0Lexer =
            symbol::io::PL0Lexer::create_from_content("write(1)");
        let mut generator = codegen::CodeGenerator::new();

        generator.parse_statement(0, &mut lex);

        assert_eq!(generator.code_pointer, 3);
        assert_eq!(generator.code[0].f, vm::Fct::Lit);
        assert_eq!(generator.code[0].l, 0);
        assert_eq!(generator.code[0].a, 1);
        assert_eq!(generator.code[1].f, vm::Fct::Opr);
        assert_eq!(generator.code[1].l, 0);
        assert_eq!(generator.code[1].a, 14);
        assert_eq!(generator.code[2].f, vm::Fct::Opr);
        assert_eq!(generator.code[2].l, 0);
        assert_eq!(generator.code[2].a, 15);
    }

    /* test double write statement */
    #[test]
    fn test_double_write_statement() {
        let mut lex: symbol::io::PL0Lexer =
            symbol::io::PL0Lexer::create_from_content("write(1, 2)");
        let mut generator = codegen::CodeGenerator::new();

        generator.parse_statement(0, &mut lex);

        assert_eq!(generator.code_pointer, 5);
        assert_eq!(generator.code[0].f, vm::Fct::Lit);
        assert_eq!(generator.code[0].l, 0);
        assert_eq!(generator.code[0].a, 1);
        assert_eq!(generator.code[1].f, vm::Fct::Opr);
        assert_eq!(generator.code[1].l, 0);
        assert_eq!(generator.code[1].a, 14);
        assert_eq!(generator.code[2].f, vm::Fct::Lit);
        assert_eq!(generator.code[2].l, 0);
        assert_eq!(generator.code[2].a, 2);
        assert_eq!(generator.code[3].f, vm::Fct::Opr);
        assert_eq!(generator.code[3].l, 0);
        assert_eq!(generator.code[3].a, 14);
        assert_eq!(generator.code[4].f, vm::Fct::Opr);
        assert_eq!(generator.code[4].l, 0);
        assert_eq!(generator.code[4].a, 15);
    }
}
