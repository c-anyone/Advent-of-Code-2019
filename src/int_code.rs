use itertools;
use std::convert::TryFrom;

pub fn parse_program(input: &str) -> Result<Vec<i32>, std::num::ParseIntError> {
    input.split(',').map(|s| s.parse()).collect()
}

#[derive(Debug, Clone)]
pub enum Opcode {
    Add,
    Mult,
    Stop,
    Input,
    Output,
    JumpTrue,
    JumpFalse,
    LessThan,
    Equals,
    Err,
}

type Memory = Vec<i32>;

#[derive(Debug, Clone, Copy)]
pub enum Param {
    Pos,
    Imm,
}

impl Opcode {
    pub fn len(&self) -> usize {
        match self {
            Opcode::Add => 4,
            Opcode::Mult => 4,
            Opcode::Input => 2,
            Opcode::Output => 2,
            Opcode::JumpTrue => 3,
            Opcode::JumpFalse => 3,
            Opcode::LessThan => 4,
            Opcode::Equals => 4,
            Opcode::Stop => 0,
            Opcode::Err => 0,
        }
    }

    pub fn params(&self) -> usize {
        match self {
            Opcode::Err => 0,
            x => x.len().saturating_sub(1),
        }
    }
}

// impl From<&[i32]> for Opcode<'_> {
//     fn from(inst: &[i32]) -> Self {
//         // get the opcode and take out
//         let x = *inst.first().unwrap();

//     }
// }

impl TryFrom<&str> for IntComputer {
    type Error = &'static str;
    fn try_from(text: &str) -> Result<Self, Self::Error> {
        match parse_program(text) {
            Ok(program) => Ok(IntComputer::new(program)),
            Err(_) => Err("Failed to parse IntCode Program"),
        }
    }
}

impl TryFrom<i32> for Param {
    type Error = &'static str;
    fn try_from(val: i32) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Param::Pos),
            1 => Ok(Param::Imm),
            _ => Err("invalid param value"),
        }
    }
}

pub struct IntComputer {
    mem: Memory,
    pc: usize,
    input: Vec<i32>,
}

struct Instruction {
    op: Opcode,
    params: Vec<Param>,
}

impl IntComputer {
    pub fn new(prog: Vec<i32>) -> Self {
        IntComputer {
            mem: prog.to_owned(),
            pc: 0,
            input: Vec::new(),
        }
    }

    pub fn push_input(&mut self, value: i32) {
        self.input.push(value);
    }

    pub fn run(&mut self) -> Result<(), String> {
        loop {
            let x = self.step();
            return match x {
                Ok(true) => continue,
                Ok(false) => Ok(()),
                Err(x) => Err(format!("Execution halted with an Error {}", x)),
            };
            // break; 
            // result
        }
        // result
    }

    pub fn step(&mut self) -> Result<bool, String> {
        let inst = self.get_instruction()?;

        let result = match inst.op {
            Opcode::Add => {
                let &i1 = self.try_get_param_ref(inst.params[0], self.pc + 1)?;
                let &i2 = self.try_get_param_ref(inst.params[1], self.pc + 2)?;
                let &out = self.try_get_param_ref(Param::Imm, self.pc + 3)?;

                self.try_store_at(i1 + i2, out as usize)?;
                self.pc += inst.op.len();

                Ok(true)
            }
            Opcode::Mult => {
                let &i1 = self.try_get_param_ref(inst.params[0], self.pc + 1)?;
                let &i2 = self.try_get_param_ref(inst.params[1], self.pc + 2)?;
                let &out = self.try_get_param_ref(Param::Imm, self.pc + 3)?;

                self.try_store_at(i1 * i2, out as usize)?;
                self.pc += inst.op.len();
                Ok(true)
            }
            Opcode::Input => {
                if self.input.is_empty() {
                    Err(format!("No Input supplied"))
                } else {
                    let val = self.input.pop().unwrap();
                    let p = inst.params[0];
                    let &loc = self.try_get_param_ref(Param::Imm, self.pc + 1).unwrap();
                    self.try_store_at(val, loc as usize)?;
                    self.pc += inst.op.len();
                    Ok(true)
                }
            }
            Opcode::Output => {
                let &out = self.try_get_param_ref(inst.params[0], self.pc + 1)?;
                println!("Output: {}", out);
                self.pc += inst.op.len();
                Ok(true)
            }
            Opcode::Stop => Ok(false),
            Opcode::JumpTrue => {
                let &input = self.try_get_param_ref(inst.params[0], self.pc + 1)?;
                if input != 0 {
                    let &new_pc = self.try_get_param_ref(inst.params[1], self.pc + 2)?;
                    if new_pc as usize > self.mem.len() {
                        return Err(format!(
                            "New PC {} @ {} not valid, len {}",
                            new_pc,
                            self.pc,
                            self.mem.len()
                        ));
                    }
                    self.pc = new_pc as usize;
                } else {
                    self.pc += inst.op.len();
                }
                Ok(true)
            }
            Opcode::JumpFalse => {
                let &input = self.try_get_param_ref(inst.params[0], self.pc + 1)?;
                if input == 0 {
                    let &new_pc = self.try_get_param_ref(inst.params[1], self.pc + 2)?;
                    if new_pc as usize > self.mem.len() {
                        return Err(format!(
                            "New PC {} @ {} not valid, len {}",
                            new_pc,
                            self.pc,
                            self.mem.len()
                        ));
                    }
                    self.pc = new_pc as usize;
                } else {
                    self.pc += inst.op.len();
                }
                Ok(true)
            }
            Opcode::LessThan => {
                let &i1 = self.try_get_param_ref(inst.params[0], self.pc + 1)?;
                let &i2 = self.try_get_param_ref(inst.params[1], self.pc + 2)?;
                let &out = self.try_get_param_ref(Param::Imm, self.pc + 3)?;

                let res = if i1 < i2 { 1 } else { 0 };
                self.try_store_at(res, out as usize)?;
                self.pc += inst.op.len();
                Ok(true)
            }
            Opcode::Equals => {
                let &i1 = self.try_get_param_ref(inst.params[0], self.pc + 1)?;
                let &i2 = self.try_get_param_ref(inst.params[1], self.pc + 2)?;
                let &out = self.try_get_param_ref(Param::Imm, self.pc + 3)?;

                let res = if i1 == i2 { 1 } else { 0 };
                self.try_store_at(res, out as usize)?;
                self.pc += inst.op.len();
                Ok(true)
            }
            Opcode::Err => Err(format!(
                "Invalid Opcode {} @ {}",
                self.mem[self.pc], self.pc
            )),
            _ => {
                return Err(format!(
                    "Not Implemented Instruction {} @ {}",
                    self.mem[self.pc], self.pc
                ))
            }
        };
        result
    }

    fn try_get_param_ref(&self, p: Param, index: usize) -> Result<&i32, String> {
        if index > self.mem.len() {
            return Err(format!(
                "Halted @ {:04} :Index {} out of bounds",
                self.pc, index,
            ));
        }
        match p {
            Param::Imm => Ok(self.mem.get(index).unwrap()),
            Param::Pos => {
                let location = *self.mem.get(index).unwrap() as usize;
                match self.mem.get(location) {
                    Some(x) => Ok(x),
                    None => Err(format!(
                        "Halted @ {:04} :Index {} @ {} out of bounds",
                        self.pc, location, index
                    )),
                }
            }
        }
    }

    fn try_store_at(&mut self, value: i32, index: usize) -> Result<(), String> {
        if index > self.mem.len() {
            Err(format!(
                "Halted @ {:04} :Index {} out of bounds",
                self.pc, index
            ))
        } else {
            self.mem[index] = value;
            Ok(())
        }
    }

    fn get_instruction(&self) -> Result<Instruction, String> {
        if self.pc > self.mem.len() {
            return Err(format!(
                "PC out of bounds: {:04} length {}",
                self.pc,
                self.mem.len()
            ));
        }
        let &x = self.mem.get(self.pc).unwrap();
        let op = if x > 99 { x % 100 } else { x };
        let mut params = x / 100;

        let op = match op {
            1 => Opcode::Add,
            2 => Opcode::Mult,
            3 => Opcode::Input,
            4 => Opcode::Output,
            5 => Opcode::JumpTrue,
            6 => Opcode::JumpFalse,
            7 => Opcode::LessThan,
            8 => Opcode::Equals,
            99 => Opcode::Stop,
            _ => Opcode::Err,
        };
        let mut result: Vec<Param> = Vec::new();
        for _ in 0..op.params() {
            let p = params % 10;
            let p: Param = match Param::try_from(p) {
                Ok(x) => x,
                Err(_) => return Err(format!("Invalid parameter @ pc {}", self.pc)),
            };
            result.push(p);
            params /= 10;
        }

        Ok(Instruction {
            op: op,
            params: result,
        })
    }
}
