fn main() {
    let mut mycpu = CPU::new();
    let mut program: Vec<Instructions> = vec![];

    program.push(Instructions::Set(Register::A, 5));
    program.push(Instructions::Set(Register::B, 3));

    program.push(Instructions::Add(Register::B, Register::A));
    program.push(Instructions::Sub(Register::A, 2));
    program.push(Instructions::Print(Register::A));
    program.push(Instructions::Print(Register::B));
    program.push(Instructions::Halt);

    match mycpu.execute(program) {
        Ok(_) => println!("Success!"),
        Err(_) => println!("Failure!"),
    }

    let mut program: Vec<Instructions> = vec![];

    program.push(Instructions::Set(Register::A, 5));
    program.push(Instructions::Set(Register::B, 3));

    program.push(Instructions::Add(Register::B, Register::A));
    program.push(Instructions::Sub(Register::A, 2));
    program.push(Instructions::Print(Register::A));
    program.push(Instructions::Print(Register::B));
    program.push(Instructions::Halt);

    mycpu.pc = 0;
    match mycpu.execute_improved(program) {
        Ok(_) => println!("Success!"),
        Err(_) => println!("Failure!"),
    }
}

struct CPU {
    a: i32,
    b: i32,
    pc: usize,
}
enum Instructions {
    Set(Register, i32),
    Add(Register, Register),
    Sub(Register, i32),
    Print(Register),
    Halt,
}

enum Register {
    A,
    B,
}

impl CPU {
    fn new() -> Self {
        CPU { a: 0, b: 0, pc: 0 }
    }
    fn get_reg_mut(&mut self, reg: &Register) -> &mut i32 {
        match reg {
            Register::A => &mut self.a,
            Register::B => &mut self.b,
        }
    }
    fn execute(&mut self, prog: Vec<Instructions>) -> Result<i32, String> {
        loop {
            match prog.get(self.pc) {
                Some(instruction) => match instruction {
                    Instructions::Set(reg, num) => match reg {
                        Register::A => self.a = *num,
                        Register::B => self.b = *num,
                    },
                    Instructions::Add(src, dest) => match (src, dest) {
                        (Register::A, Register::A) => self.a += self.a,
                        (Register::A, Register::B) => self.b += self.a,
                        (Register::B, Register::A) => self.a += self.b,
                        (Register::B, Register::B) => self.b += self.b,
                    },
                    Instructions::Sub(reg, num) => match reg {
                        Register::A => self.a -= *num,
                        Register::B => self.b -= *num,
                    },
                    Instructions::Print(reg) => match reg {
                        Register::A => println!("Register A: {}", self.a),
                        Register::B => println!("Register B: {}", self.b),
                    },
                    Instructions::Halt => {
                        break Ok::<i32, String>(self.a);
                    }
                },
                None => {
                    println!("Booom! Out of bounds");
                    break Err(String::from("Crash: Segment Fault"));
                }
            }
            self.pc += 1;
        }
    }
    fn execute_improved(&mut self, prog: Vec<Instructions>) -> Result<i32, String> {
        loop {
            match prog.get(self.pc) {
                Some(instruction) => match instruction {
                    Instructions::Set(reg, num) => {
                        let target = self.get_reg_mut(reg);
                        *target = *num;
                    }
                    Instructions::Add(src, dest) => {
                        let src_val = *self.get_reg_mut(src);
                        let dest = self.get_reg_mut(dest);
                        *dest += src_val;
                    }
                    Instructions::Sub(reg, num) => {
                        let target = self.get_reg_mut(reg);
                        *target -= *num;
                    }
                    Instructions::Print(reg) => {
                        let target = self.get_reg_mut(reg);
                        println!("{}", *target);
                    }
                    Instructions::Halt => {
                        break Ok::<i32, String>(self.a);
                    }
                },
                None => {
                    println!("Booom! Out of bounds");
                    break Err(String::from("Crash: Segment Fault"));
                }
            }
            self.pc += 1;
        }
    }
}
