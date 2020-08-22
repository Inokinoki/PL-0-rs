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
        self.code_pointer += 1;
        self.code.push(self.gen(vm::Fct::Inte, 0, data_pointer));
        // Statement
        self.parse_statement(level, lexer);
        // Should end with end/semicolon
        self.code_pointer += 1;
        self.code.push(self.gen(vm::Fct::Opr, 0, 0));
        // End statement
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
        {
            // Get the next symbol
            lexer.next();
        }
        match *lexer.current() {
            symbol::Symbol::Ident => {
                // Handle as a assignment statement

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
                        level - self.name_table[identifier_index].level,
                        self.name_table[identifier_index].adr
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

                            lexer.next();
                        }

                        if *lexer.current() != symbol::Symbol::Comma {
                            break;
                        }
                    }
                }

                if should_continue {
                    if *lexer.next() != symbol::Symbol::Rparen {
                        should_continue = false;
                    }
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

                            lexer.next();
                        }

                        if *lexer.current() != symbol::Symbol::Comma {
                            break;
                        }
                    }
                }

                if should_continue {
                    if *lexer.next() != symbol::Symbol::Rparen {
                        should_continue = false;
                    }
                }

                if should_continue {
                    self.code_pointer += 1;
                    // New line
                    self.code.push(self.gen(vm::Fct::Opr, 0, 16));
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

                if self.name_table[index].kind == nametab::NameTableObject::Procedur {
                    self.code_pointer += 1;
                    self.code.push(self.gen(vm::Fct::Cal, 
                        level - self.name_table[index].level, self.name_table[index].adr));
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

                self.parse_statement(level, lexer);

                loop {
                    {
                        // Get the next symbol
                        lexer.next();
                    }
                    if *lexer.current() != symbol::Symbol::Semicolon {
                        break;
                    }
                    self.parse_statement(level, lexer);
                }

                if *lexer.current() != symbol::Symbol::Endsym {
                    // Can raise an error
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

                if *lexer.current() != symbol::Symbol::Dosym {
                    should_continue = false;
                }

                if should_continue {
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
            lexer.next();
        }
        if *lexer.current() == symbol::Symbol::Minus {
            is_positive = false;
        }

        {
            // Parse a term
            self.parse_term(level, lexer);
        }

        if !is_positive {
            // Negative
            self.code_pointer += 1;
            self.code.push(self.gen(vm::Fct::Opr, 0, 1));
        }

        // TODO: Clarify sym != plus and sym != minus, gensym or not

        loop {
            {
                lexer.next();
            }

            if *lexer.current() == symbol::Symbol::Minus {
                is_positive = false;
            } else if *lexer.current() == symbol::Symbol::Plus {
                is_positive = true;
            }
    
            self.parse_term(level, lexer);

            self.code_pointer += 1;
            if is_positive {
                self.code.push(self.gen(vm::Fct::Opr, 0, 2));
            } else {
                self.code.push(self.gen(vm::Fct::Opr, 0, 3));
            }

            if *lexer.current() != symbol::Symbol::Minus && *lexer.current() != symbol::Symbol::Plus {
                break;
            }
        }


    }

    fn parse_term(&mut self, level: usize, lexer: &mut symbol::io::PL0Lexer) {
        self.parse_factor(level, lexer);

        {
            lexer.next();
        }

        loop {
            let mut isTime = false;
            let mut isSlash = false;
            match lexer.current() {
                symbol::Symbol::Times => {
                    isTime = true;
                },
                symbol::Symbol::Slash => {
                    isSlash = true;
                },
                _ => {
                    // Nothing
                },
            }

            self.parse_factor(level, lexer);

            self.code_pointer += 1;
            if isTime {
                self.code.push(self.gen(vm::Fct::Opr, 0, 4));
            } else if isSlash {
                self.code.push(self.gen(vm::Fct::Opr, 0, 5));
            }

            {
                lexer.next();
            }

            if *lexer.current() != symbol::Symbol::Times && *lexer.current() != symbol::Symbol::Slash {
                break;
            }
        }
    }

    fn parse_factor(&mut self, level: usize, lexer: &mut symbol::io::PL0Lexer) {
        // Handle factor
        loop {
            {
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
                    // TODO: parse usize/i64
                    // self.code.push(self.gen(vm::Fct::Lit, 0,
                    //     lexer.current_content() as usize));
                },
                symbol::Symbol::Lparen => {
                    // Left parent
                    self.parse_expression(level, lexer);

                    {
                        lexer.next();
                    }

                    if *lexer.current() != symbol::Symbol::Rparen {
                        // TODO: raise an error
                    }
                },
                _ => {
                    // Nothing to do
                },
            }

            // TODO: jump out condition
        }
    }

    fn parse_condition(&mut self, level: usize, lexer: &mut symbol::io::PL0Lexer) {
        let mut should_continue = true;

        {
            lexer.next();
        }

        match *lexer.current() {
            symbol::Symbol::Oddsym => {
                self.parse_expression(level, lexer);
                self.code_pointer += 1;
                self.code.push(self.gen(vm::Fct::Opr, 0, 6));
            },
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
}
