use std::io;


pub const STACK_SIZE: usize = 4096;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Fct {
    Lit,
    Opr,
    Lod,
    Sto,
    Cal,
    Inte,
    Jmp,
    Jpc,
    Hlt,    // Halt
}

/* instruction structure */
#[derive(Copy, Clone)]
pub struct Instruction {
    pub f: Fct,         // instruction
    pub l: usize,         // level difference between declaration and reference
    pub a: usize,         // a variant depending on l
}

pub struct PL0VirtualMachine {
    pc: usize,  // program counter
    bp: usize,  // base address pointer
    sp: usize,  // stack pointer

    stack: Vec<i64>,

    current_instruction: Instruction,
    instructions: Vec<Instruction>,
}

impl PL0VirtualMachine {
    pub fn load(ins: Vec<Instruction>) -> PL0VirtualMachine {
        let vm = PL0VirtualMachine {
            pc: 0,
            bp: 0,
            sp: 0,
            stack: Vec::with_capacity(STACK_SIZE),

            current_instruction: Instruction {
                f: Fct::Hlt,
                l: 0,
                a: 0,
            },
            instructions: ins,
        };
        vm
    }

    pub fn execute(&mut self) {
        self.pc = 0;
        self.bp = 0;
        self.sp = 0;

        loop {
            self.single_step_execute();     // Single step

            if self.pc == 0 {
                break;
            }
        }
    }

    pub fn single_step_execute(&mut self) {
        self.current_instruction = self.instructions[self.pc];

        // Debug purpose
        println!("{:?}", self.current_instruction.f);

        self.pc += 1;   // Move PC

        match self.current_instruction.f {
            Fct::Lit => {
                // Push the value of a to the stack
                self.stack[self.sp] = self.current_instruction.a as i64;
                self.sp += 1;
            },
            Fct::Opr => {
                match self.current_instruction.a {
                    0 => {
                        self.sp = self.bp;
                        self.pc = self.stack[self.sp + 2] as usize;
                        self.bp = self.stack[self.sp + 1] as usize;
                    },
                    1 => {
                        // Inverse the number on the top of stack
                        self.stack[self.sp - 1] = - self.stack[self.sp - 1];
                    },
                    2 => {
                        // Sum
                        self.sp -= 1;
                        self.stack[self.sp - 1] = self.stack[self.sp - 1] + self.stack[self.sp];
                    },
                    3 => {
                        // Difference
                        self.sp -= 1;
                        self.stack[self.sp - 1] = self.stack[self.sp - 1] - self.stack[self.sp];
                    },
                    4 => {
                        // Multiplication
                        self.sp -= 1;
                        self.stack[self.sp - 1] = self.stack[self.sp - 1] * self.stack[self.sp];
                    },
                    5 => {
                        // Division
                        self.sp -= 1;
                        self.stack[self.sp - 1] = self.stack[self.sp - 1] / self.stack[self.sp];
                    },
                    6 => {
                        self.stack[self.sp - 1] = self.stack[self.sp - 1] % 2;
                    },
                    8 => {
                        // Equal
                        self.sp -= 1;
                        self.stack[self.sp - 1] = (self.stack[self.sp - 1] == self.stack[self.sp]) as i64;
                    },
                    9 => {
                        // Inequal
                        self.sp -= 1;
                        self.stack[self.sp - 1] = (self.stack[self.sp - 1] != self.stack[self.sp]) as i64;
                    },
                    10 => {
                        // Less
                        self.sp -= 1;
                        self.stack[self.sp - 1] = (self.stack[self.sp - 1] < self.stack[self.sp]) as i64; 
                    },
                    11 => {
                        // Bigger or equal
                        self.sp -= 1;
                        self.stack[self.sp - 1] = (self.stack[self.sp - 1] >= self.stack[self.sp]) as i64; 
                    },
                    12 => {
                        // Bigger
                        self.sp -= 1;
                        self.stack[self.sp - 1] = (self.stack[self.sp - 1] > self.stack[self.sp]) as i64; 
                    },
                    13 => {
                        // Less or equal
                        self.sp -= 1;
                        self.stack[self.sp - 1] = (self.stack[self.sp - 1] <= self.stack[self.sp]) as i64; 
                    },
                    14 => {
                        print!("{}", self.stack[self.sp - 1]);
                        self.sp -= 1;
                    },
                    15 => {
                        print!("\n");
                    },
                    16 => {
                        let mut input_number: String = String::new();
                        print!("?");
                        io::stdin().read_line(&mut input_number)
                            .expect("Failed to read line");

                        match input_number.trim().parse() {
                            Ok(num) => {
                                self.stack[self.sp] = num;
                                self.sp += 1;
                            },
                            Err(err) => {
                                println!("{}", err);
                            }
                        };
                        
                    },
                    _ => {
                        // Unsupported instruction
                    },
                }
            },
            Fct::Lod => {
                // Push the data on address a (base b) to the stack
                self.stack[self.sp] = self.stack[
                    base(self.current_instruction.l, &self.stack, self.bp) + self.current_instruction.a
                ];
                self.sp += 1;
            },
            Fct::Sto => {
                // Stock
                self.sp -= 1;
                let adr: usize = base(self.current_instruction.l, &self.stack, self.bp) + self.current_instruction.a;
                self.stack[adr] = self.stack[self.sp];
            },
            Fct::Cal => {
                // Call a procedure
                self.stack[self.sp] = base(self.current_instruction.l, &self.stack, self.bp) as i64;
                self.stack[self.sp + 1] = self.bp as i64;
                self.stack[self.sp + 2] = self.pc as i64;

                self.bp = self.sp;
                self.pc = self.current_instruction.a;   // Jump
            },
            Fct::Inte => {
                // Expand stack
                self.sp += self.current_instruction.a;
            },
            Fct::Jmp => {
                // Jump
                self.pc = self.current_instruction.a;
            },
            Fct::Jpc => {
                // Conditional Jump
                self.sp -= 1;
                if self.stack[self.sp] == 0 {
                    self.pc = self.current_instruction.a;
                }
            },
            _ => {
                // Other, unsupported instruction
            }
        }
    }
}


fn base(l: usize, s: &Vec<i64>, b: usize) -> usize {
    let mut level = l;
    let mut base_address: usize = b;

    // Search until the first level
    while level > 0 {
        base_address = s[base_address] as usize;
        level -= 1;
    }
    base_address
}
