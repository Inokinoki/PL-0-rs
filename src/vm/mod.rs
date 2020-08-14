#[derive(Copy, Clone)]
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
    f: Fct,         // instruction
    l: usize,         // level difference between declaration and reference
    a: usize,         // a variant depending on l
}

pub struct PL0VirtualMachine {
    pc: usize,  // program counter
    bp: usize,  // base address pointer

    stack: Vec<u32>,

    current_instruction: Instruction,
    instructions: Vec<Instruction>,
}

impl PL0VirtualMachine {
    fn load(ins: Vec<Instruction>) -> PL0VirtualMachine {
        let vm = PL0VirtualMachine {
            pc: 0,
            bp: 0,
            stack: Vec::new(),

            current_instruction: Instruction {
                f: Fct::Hlt,
                l: 0,
                a: 0,
            },
            instructions: ins,
        };
        vm
    }

    fn execute(&mut self) {
        self.pc = 0;
        self.bp = 0;

        loop {
            self.single_step_execute();     // Single step

            if self.pc == 0 {
                break;
            }
        }
    }

    fn single_step_execute(&mut self) {
        self.current_instruction = self.instructions[self.pc];

        // TODO: implement
    }
}
