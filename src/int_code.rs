use std::collections::VecDeque;
use itertools::zip;
use std::convert::TryFrom;

pub fn parse_program(input: &str) -> Result<Vec<ValueType>, std::num::ParseIntError> {
    input.trim().split(',').map(|s| s.parse()).collect()
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
    SetRel,
    Err,
}

pub type ValueType = i128;
type Memory = Vec<ValueType>;

#[derive(Debug, Clone, Copy)]
pub enum Param {
    Pos,
    Imm,
    Rel,
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
            Opcode::SetRel => 2,
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

impl TryFrom<&str> for IntComputer {
    type Error = &'static str;
    fn try_from(text: &str) -> Result<Self, Self::Error> {
        match parse_program(text) {
            Ok(program) => Ok(IntComputer::new(program)),
            Err(_) => Err("Failed to parse IntCode Program"),
        }
    }
}

impl TryFrom<ValueType> for Param {
    type Error = &'static str;
    fn try_from(val: ValueType) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Param::Pos),
            1 => Ok(Param::Imm),
            2 => Ok(Param::Rel),
            _ => Err("invalid param value"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct IntComputer {
    mem: Memory,
    pc: usize,
    rel_base: usize,
    state: IntComputerState,
    input: VecDeque<ValueType>,
    output: VecDeque<ValueType>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IntComputerState {
    // Created,
    Initialized,
    Running,
    Halted,
    Stopped,
}

struct Instruction {
    op: Opcode,
    params: Vec<Param>,
}

const MEMSIZE: usize = 1024*1024;

impl IntComputer {
    pub fn new(prog: Vec<ValueType>) -> Self {
        // reserve MUCH more memory than needed
        let mut mem: Memory = Vec::with_capacity(MEMSIZE);
        for _ in 0..MEMSIZE {
            mem.push(0);
        }

        zip(prog.iter(), mem.iter_mut()).for_each(|(p, m)| *m = *p as ValueType);

        IntComputer {
            mem: mem,
            pc: 0,
            rel_base: 0,
            state: IntComputerState::Initialized,
            input: VecDeque::new(),
            output: VecDeque::new(),
        }
    }

    pub fn get_output(&mut self) -> Option<ValueType> {
       self.output.pop_front()
    }

    pub fn get_state(&self) -> IntComputerState {
        self.state
    }

    pub fn push_input(&mut self, value: ValueType) {
        self.input.push_back(value);
    }

    pub fn run(&mut self) -> Result<IntComputerState, String> {
        loop {
            // let x = self.step();
            self.step()?;
            use IntComputerState::*;
            match self.state {
                Initialized => (),
                Running => (),
                Halted | Stopped => break,
            };
        }
        Ok(self.state)
    }

    pub fn step(&mut self) -> Result<Opcode, String> {
        let inst = self.get_instruction()?;
        let mut iter = inst.params.iter();
        self.state = IntComputerState::Running;
        let _ = match inst.op {
            Opcode::Add => {
                let &i1 = self.try_get_mem_ref(*iter.next().unwrap(), self.pc + 1)?;
                let &i2 = self.try_get_mem_ref(*iter.next().unwrap(), self.pc + 2)?;
                let out = self.try_get_mem_ref_mut(*iter.next().unwrap(), self.pc + 3)?;

                let (result, overflow) = i1.overflowing_add(i2);
                *out = i1.saturating_add(i2);

                if overflow {
                    println!("Overflow @ {} -> {} + {}", self.pc, i1, i2);
                }
                // self.try_store_at(i1 + i2, out.try_into().unwrap())?;
                self.pc += inst.op.len();

                Ok(true)
            }
            Opcode::Mult => {
                let &i1 = self.try_get_mem_ref(*iter.next().unwrap(), self.pc + 1)?;
                let &i2 = self.try_get_mem_ref(*iter.next().unwrap(), self.pc + 2)?;
                let out = self.try_get_mem_ref_mut(*iter.next().unwrap(), self.pc + 3)?;
                *out = i1.saturating_mul(i2);
                // self.try_store_at(i1 * i2, out.try_into().unwrap())?;
                self.pc += inst.op.len();
                Ok(true)
            }
            Opcode::Input => {
                if self.input.is_empty() {
                    // Err(format!("No Input supplied"))
                    self.state = IntComputerState::Halted;
                    Ok(false)
                } else {
                    let val = self.input.pop_front().unwrap();
                    let loc = self.try_get_mem_ref_mut(*iter.next().unwrap(), self.pc + 1).unwrap();
                    *loc = val;

                    self.pc += inst.op.len();
                    Ok(true)
                }
            }
            Opcode::Output => {
                let &out = self.try_get_mem_ref(*iter.next().unwrap(), self.pc + 1)?;
                self.output.push_back(out);
                // println!("Output: {}", out);
                self.pc += inst.op.len();
                Ok(true)
            }
            Opcode::Stop => {
                self.state = IntComputerState::Stopped;
                Ok(false)
            }
            Opcode::JumpTrue => {
                let &input = self.try_get_mem_ref(*iter.next().unwrap(), self.pc + 1)?;
                if input != 0 {
                    let &new_pc = self.try_get_mem_ref(*iter.next().unwrap(), self.pc + 2)?;
                    let new_pc = new_pc as usize;
                    if new_pc > self.mem.len() {
                        return Err(format!(
                            "New PC {} @ {} not valid, len {}",
                            new_pc,
                            self.pc,
                            self.mem.len()
                        ));
                    }
                    self.pc = new_pc;
                } else {
                    self.pc += inst.op.len();
                }
                Ok(true)
            }
            Opcode::JumpFalse => {
                let &input = self.try_get_mem_ref(*iter.next().unwrap(), self.pc + 1)?;
                if input == 0 {
                    let &new_pc = self.try_get_mem_ref(*iter.next().unwrap(), self.pc + 2)?;
                    let new_pc = new_pc as usize;
                    if new_pc > self.mem.len() {
                        return Err(format!(
                            "New PC {} @ {} not valid, len {}",
                            new_pc,
                            self.pc,
                            self.mem.len()
                        ));
                    }
                    self.pc = new_pc;
                } else {
                    self.pc += inst.op.len();
                }
                Ok(true)
            }
            Opcode::LessThan => {
                let &i1 = self.try_get_mem_ref(*iter.next().unwrap(), self.pc + 1)?;
                let &i2 = self.try_get_mem_ref(*iter.next().unwrap(), self.pc + 2)?;
                let out = self.try_get_mem_ref_mut(*iter.next().unwrap(), self.pc + 3)?;

                let res = if i1 < i2 { 1 } else { 0 };
                *out = res;
                self.pc += inst.op.len();
                Ok(true)
            }
            Opcode::Equals => {
                let &i1 = self.try_get_mem_ref(*iter.next().unwrap(), self.pc + 1)?;
                let &i2 = self.try_get_mem_ref(*iter.next().unwrap(), self.pc + 2)?;
                let out = self.try_get_mem_ref_mut(*iter.next().unwrap(), self.pc + 3)?;

                let res = if i1 == i2 { 1 } else { 0 };
                *out = res;
                self.pc += inst.op.len();
                Ok(true)
            }
            Opcode::Err => Err(format!(
                "Invalid Opcode {} @ {}",
                self.mem[self.pc], self.pc
            )),

            Opcode::SetRel => {
                let base = *self.try_get_mem_ref(*iter.next().unwrap(), self.pc + 1)?;
                let rl = self.rel_base as ValueType;
                self.rel_base = (base + rl) as usize;
                self.pc += inst.op.len();

                Ok(true)
            }
        };
        Ok(inst.op)
    }

    fn try_get_mem_ref(&self, p: Param, index: usize) -> Result<&ValueType, String> {
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
            Param::Rel => {
                // Relative means the value at pc+n is to be added to rel_base and this value is accessed
                let offset = *self.mem.get(index).unwrap() as isize;
                match self.mem.get((self.rel_base as isize + offset) as usize) {
                    Some(x) => Ok(x),
                    None => Err(format!(
                        "Halted @ {:04} Offset {} base {}",
                        self.pc, offset, self.rel_base
                    )),
                }
            }
        }
    }

    fn try_get_mem_ref_mut(&mut self, p: Param, index: usize) -> Result<&mut ValueType, String> {
        if index > self.mem.len() {
            return Err(format!(
                "Halted @ {:04} :Index {} out of bounds",
                self.pc, index,
            ));
        }
        match p {
            Param::Imm => Ok(self.mem.get_mut(index).unwrap()),
            Param::Pos => {
                let location = *self.mem.get(index).unwrap() as usize;
                match self.mem.get_mut(location) {
                    Some(x) => Ok(x),
                    None => Err(format!(
                        "Halted @ {:04} :Index {} @ {} out of bounds",
                        self.pc, location, index
                    )),
                }
            }
            Param::Rel => {
                // Relative means the value at pc+n is to be added to rel_base and this value is accessed
                let offset = *self.mem.get(index).unwrap() as isize;
                match self.mem.get_mut((self.rel_base as isize + offset) as usize) {
                    Some(x) => Ok(x),
                    None => Err(format!(
                        "Halted @ {:04} Offset {} base {}",
                        self.pc, offset, self.rel_base
                    )),
                }
            }
        }
    }

    // fn try_store_at(&mut self, value: ValueType, index: usize) -> Result<(), String> {
    //     if index > self.mem.len() {
    //         Err(format!(
    //             "Halted @ {:04} :Index {} out of bounds",
    //             self.pc, index
    //         ))
    //     } else {
    //         self.mem[index] = value;
    //         Ok(())
    //     }
    // }

    fn get_instruction(&self) -> Result<Instruction, String> {
        if self.pc > self.mem.len() {
            return Err(format!(
                "PC out of bounds: {:04} length {}",
                self.pc,
                self.mem.len()
            ));
        }
        let x = *self.mem.get(self.pc).unwrap() as ValueType;
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
            9 => Opcode::SetRel,
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
