extern crate itertools;
use itertools::multizip;
use std::convert::TryFrom;

pub fn parse_program(input: &str) -> Result<Vec<i32>, std::num::ParseIntError> {
    input.split(',').map(|s| s.parse()).collect()
}

impl IntComputer {
    // type Error = &'static str;
    pub fn new(data: &Vec<i32>) -> Self {
        IntComputer {
            initial_state: data.clone(),
            state: data.clone(),
            pc: 0,
            register: [0, 0, 0],
            op: Opcode::Err,
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
            Param::Positional => match state.get(value as usize) {
                Some(&p) => Ok(p),
                None => Err(format!("Index {} out of Bounds", value)),
            },
            Param::Immediate => Ok(value as i32),
            Param::Invalid => Err(format!("Illegal param, value {}", value)),
        }
    }

    fn store_result(&mut self, value: i32, param: Param, location: usize) -> Result<(), String> {
        if location > self.state.len() {
            return Err(format!("Failed to store {}  @{}", value, location));
        }
        self.state[location] = value;
        Ok(())
    }

    fn execute_instruction(&mut self) -> Result<Opcode, String> {
        self.op = match self.load_instruction_at_pc() {
            Ok(op) => op,
            Err(e) => return Err(e),
        };
        let result = match self.op {
            Opcode::Add(params) => {
                let input_iter = self.state.iter().skip(self.pc + 1).take(3);
                let reg_iter = self.register.iter_mut();
                let iter = multizip((input_iter, reg_iter, params.iter()));

                for (&inp, reg, &param) in iter {
                    let val = IntComputer::load_param(&self.state, inp as isize, param)?;
                    *reg = val;
                }
                self.store_result(
                    self.register[0] + self.register[1],
                    params[2],
                    self.register[2] as usize,
                )?;
                Ok(())
            }
            Opcode::Mult(params) => {
                let input_iter = self.state.iter().skip(self.pc + 1).take(3);
                let reg_iter = self.register.iter_mut();
                let iter = multizip((input_iter, reg_iter, params.iter()));

                for (&inp, reg, &param) in iter {
                    let val = IntComputer::load_param(&self.state, inp as isize, param)?;
                    *reg = val;
                }
                self.store_result(
                    self.register[0] * self.register[1],
                    params[2],
                    self.register[2] as usize,
                )?;
                Ok(())
            }
            Opcode::Input => Ok(()), /* Prompt to input by calling an input function */
            Opcode::Output => {
                let index = match self.state.get(self.pc + 1) {
                    Some(&x) => x,
                    None => return Err(format!("Output index {} out of bounds", self.pc + 1)),
                };
                println!(
                    "Value at {}: {}",
                    self.pc + 1,
                    IntComputer::load_param(&self.state, index as isize, Param::Immediate)?
                );
                Ok(())
            } /* Output to the console */
            Opcode::Err => {
                let msg = format!("Invalid instruction @ {}", self.pc);
                println!("{}", msg);
                Err(msg)
            } /* failed to parse instruction, crash execution */
            Opcode::Stop => Ok(()),  /* done executing */
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
    Positional,
    Immediate,
    Invalid,
}

impl From<i32> for Param {
    fn from(val: i32) -> Self {
        match val {
            0 => Param::Positional,
            1 => Param::Immediate,
            _ => Param::Invalid,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Opcode {
    Add([Param; 3]),
    Mult([Param; 3]),
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

        let mut array = [Param::Invalid; 3];
        for i in 0..3 {
            array[2 - i] = (param % 10).into();
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
    initial_state: Vec<i32>,
    state: Vec<i32>,
    pc: usize,
    register: [i32; 3],
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
