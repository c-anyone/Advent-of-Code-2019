extern crate itertools;
use itertools::multizip;
use std::convert::TryFrom;
use std::io;

pub fn parse_program(input: &str) -> Result<Vec<i32>, std::num::ParseIntError> {
    input.split(',').map(|s| s.parse()).collect()
}

impl IntComputer {
    // type Error = &'static str;
    pub fn new(data: &Vec<i32>) -> Self {
        IntComputer {
            // initial_state: data.clone(),
            state: data.clone(),
            pc: 0,
            in_reg: [0, 0, 0],
            op: Opcode::Err,
            out: 0,
        }
    }

    fn load_instruction_at_pc(&mut self) -> Result<Opcode, String> {
        match self.state.get(self.pc) {
            Some(inst) => {
                let inst = *inst;
                Ok(inst.into())
            }
            None => {
                return Err(format!(
                    "PC out of bounds. PC: {} buffer {}",
                    self.pc,
                    self.state.len()
                ))
            }
        }
    }

    fn load_param(state: &Vec<i32>, value: isize, param: Param) -> Result<i32, String> {
        match param {
            Param::Pos => match state.get(value as usize) {
                Some(&p) => Ok(p),
                None => Err(format!("Index {} out of Bounds", value)),
            },
            Param::Imm => Ok(value as i32),
            Param::Invalid => Err(format!("Illegal param, value {}", value)),
        }
    }

    fn store_result(&mut self, value: i32) -> Result<(), String> {
        if self.out > self.state.len() {
            return Err(format!("Failed to store {}  @{}", value, self.out));
        }
        // println!("    Stored {} @ {}", value, self.out);
        self.state[self.out] = value;
        Ok(())
    }

    fn execute_instruction(&mut self) -> Result<Opcode, String> {
        self.op = match self.load_instruction_at_pc() {
            Ok(op) => op,
            Err(e) => return Err(e),
        };
        print!("{:38}", format!("{:4}           {:?}", self.pc, self.op));
        let result = match self.op {
            Opcode::Add(params) => {
                let input_iter = self.state.iter().skip(self.pc + 1).take(2);
                let reg_iter = self.in_reg.iter_mut().take(2);
                let iter = multizip((input_iter, reg_iter, params.iter().take(2)));

                for (&inp, reg, &param) in iter {
                    let val = IntComputer::load_param(&self.state, inp as isize, param)?;
                    *reg = val;
                }
                self.out = *self.state.get(self.pc + 3).unwrap() as usize;
                println!(
                    "    {:5} + {:5} = {:5} -> {:4}",
                    self.in_reg[0],
                    self.in_reg[1],
                    self.in_reg[0] + self.in_reg[1],
                    self.out
                );
                self.store_result(self.in_reg[0] + self.in_reg[1])?;
                Ok(())
            }
            Opcode::Mult(params) => {
                let input_iter = self.state.iter().skip(self.pc + 1).take(2);
                let reg_iter = self.in_reg.iter_mut().take(2);
                let iter = multizip((input_iter, reg_iter, params.iter().take(2)));

                for (&inp, reg, &param) in iter {
                    let val = IntComputer::load_param(&self.state, inp as isize, param)?;
                    *reg = val;
                }
                self.out = *self.state.get(self.pc + 3).unwrap() as usize;
                println!(
                    "    {:5} + {:5} = {:5} -> {:4}",
                    self.in_reg[0],
                    self.in_reg[1],
                    self.in_reg[0] * self.in_reg[1],
                    self.out
                );
                self.store_result(self.in_reg[0] * self.in_reg[1])?;
                Ok(())
            }
            Opcode::Input => {
                let mut buffer = String::new();
                println!("Please Input an integer");
                io::stdin().read_line(&mut buffer).unwrap();
                let buffer = buffer.trim_end_matches("\r\n");
                self.out = *self.state.get(self.pc + 1).unwrap() as usize;
                match buffer.parse::<i32>() {
                    Ok(x) => self.store_result(x).unwrap(),
                    Err(_) => return Err(format!("Failed to parse Input {} as int", buffer)),
                };
                Ok(())
            } /* Prompt to input by calling an input function */
            Opcode::Output => {
                let index = match self.state.get(self.pc + 1) {
                    Some(&x) => x,
                    None => return Err(format!("Output index {} out of bounds", self.pc + 1)),
                };
                println!(
                    "\nOutput----->Value at {}: {}",
                    index,
                    IntComputer::load_param(&self.state, index as isize, Param::Pos)?
                );
                Ok(())
            } /* Output to the console */
            Opcode::Err => {
                let msg = format!("Invalid instruction @ {}", self.pc);
                println!("{}", msg);
                Err(msg)
            } /* failed to parse instruction, crash execution */
            Opcode::Stop => Ok(()), /* done executing */
        };

        match result {
            Ok(()) => {
                self.pc += self.op.len();
                Ok(self.op.clone())
            }
            Err(x) => Err(x),
        }
    }

    pub fn step(&mut self) -> Result<bool, String> {
        match self.execute_instruction()? {
            Opcode::Stop => Ok(false),
            _ => Ok(true),
        }
    }

    pub fn get(&self, index: usize) -> Option<i32> {
        match self.state.get(index) {
            Some(x) => Some(*x),
            None => None,
        }
    }
}

impl TryFrom<&str> for IntComputer {
    type Error = &'static str;
    fn try_from(text: &str) -> Result<Self, Self::Error> {
        match parse_program(text) {
            Ok(program) => Ok(IntComputer::new(&program)),
            Err(_) => Err("Failed to parse IntCode Program"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Param {
    Pos,
    Imm,
    Invalid,
}

impl From<i32> for Param {
    fn from(val: i32) -> Self {
        match val {
            0 => Param::Pos,
            1 => Param::Imm,
            _ => Param::Invalid,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Opcode {
    Add([Param; 2]),
    Mult([Param; 2]),
    Stop,
    Input,
    Output,
    Err,
}

trait ParseParam {
    fn parse_params(input: u32);
}

impl From<i32> for Opcode {
    fn from(x: i32) -> Self {
        let op = if x > 99 { x % 100 } else { x };
        let mut param = x / 100;

        let mut array = [Param::Invalid; 2];
        for i in 0..2 {
            array[i] = (param % 10).into();
            param /= 10;
        }

        match op {
            1 => Opcode::Add(array),
            2 => Opcode::Mult(array),
            3 => Opcode::Input,
            4 => Opcode::Output,
            99 => Opcode::Stop,
            _ => Opcode::Err,
        }
    }
}
impl Opcode {
    pub fn len(&self) -> usize {
        match self {
            Opcode::Add(_) => 4,
            Opcode::Mult(_) => 4,
            Opcode::Input => 2,
            Opcode::Output => 2,
            Opcode::Stop => 1,
            Opcode::Err => 0,
        }
    }
}

pub struct IntComputer {
    // initial_state: Vec<i32>,
    state: Vec<i32>,
    pc: usize,
    in_reg: [i32; 3],
    out: usize,
    op: Opcode,
}

mod tests {
    use super::*;

    #[test]
    fn test_string_parsing_ok() {
        let correct = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        let input = String::from("1,9,10,3,2,3,11,0,99,30,40,50");
        let result = parse_program(&input);
        assert_eq!(true, result.is_ok());
        assert_eq!(correct, result.unwrap());
    }

    #[test]
    fn test_string_parsing_error() {
        let input = String::from("1,9,10,3,2,3,a11,0,99,30,40,50");
        let result = parse_program(&input);
        assert_eq!(true, result.is_err());
    }

    #[test]
    fn test_opcode_add() {}
}
