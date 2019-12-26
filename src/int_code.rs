use num::PrimInt;
use std::convert::TryFrom;

pub fn parse_program(input: &str) -> Result<Vec<i32>, std::num::ParseIntError> {
    input.split(',').map(|s| s.parse()).collect()
}

#[derive(Debug, PartialEq)]
pub enum Parameter {
    Positional,
    Immediate,
}

#[derive(Debug)]
pub struct IntCode {
    opcode: Opcode,
    in1: usize,
    in2: usize,
    out: usize,
}

impl From<&[i32]> for IntCode {
    fn from(x: &[i32]) -> Self {
        IntCode {
            opcode: Opcode::from(x[0]),
            in1: x[1] as usize,
            in2: x[2] as usize,
            out: x[3] as usize,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Opcode {
    Add,
    Mult,
    Stop,
    Input,
    Output,
    Err,
}

impl From<i32> for Opcode {
    fn from(x: i32) -> Self {
        let op = if x > 99 { x % 100 } else { x };
        let param = x - op;
        match op {
            1 => Opcode::Add,
            2 => Opcode::Mult,
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
            Opcode::Add => 4,
            Opcode::Mult => 4,
            Opcode::Input => 2,
            Opcode::Output => 2,
            Opcode::Stop => 1,
            Opcode::Err => 0,
        }
    }
}

pub struct IntCodeProgram {
    initial_state: Vec<i32>,
    state: Vec<i32>,
    pc: usize,
}

impl IntCodeProgram {
    pub fn current_instruction(&self) -> String {
        let x = String::from(format!("{:?}", self.parse_current_instruction()));
        x
    }

    pub fn exec_step(&mut self) -> Option<()> {
        // let inst = &self.instructions[self.pc];
        let inst = self.parse_current_instruction().unwrap();
        let mut ret = match inst.opcode {
            Opcode::Add => {
                self.state[inst.out] = self.state[inst.in1] + self.state[inst.in2];
                Some(())
            }
            Opcode::Mult => {
                self.state[inst.out] = self.state[inst.in1] * self.state[inst.in2];
                Some(())
            }
            Opcode::Input => None,
            Opcode::Output => None,
            Opcode::Stop => None,
            Opcode::Err => {
                println!("Illegal Opcode {}", self.state[self.pc]);
                None
            }
        };
        self.pc += 4;
        if self.pc >= self.state.len() {
            self.pc = 0;
            ret = None;
        }
        ret
    }

    pub fn run(&mut self) -> i32 {
        while self.exec_step() != None {}
        self.state[0]
    }

    pub fn run_with(&mut self, noun: i32, verb: i32) -> i32 {
        self.state[1] = noun;
        self.state[2] = verb;
        self.pc = 0;
        self.run()
    }

    pub fn reset(&mut self) {
        self.state = self.initial_state.clone();
        self.pc = 0;
    }

    fn parse_current_instruction(&self) -> Option<IntCode> {
        let op: Opcode = self.state[self.pc].into();

        let slice = &self.state[self.pc..self.pc + op.len()];

        // IntCode::from(slice);
        match IntCode::try_from(slice) {
            Ok(x) => Some(x),
            Err(_) => None,
        }
    }

    pub fn new(data: &Vec<i32>) -> Self {
        // let test = data.iter();
        // let state: Vec<i32> = data.iter().map(|x| (*x).from()).collect();
        IntCodeProgram {
            initial_state: data.clone(),
            state: data.clone(),
            // instructions: IntCodeProgram::parse_instructions(data).unwrap(),
            pc: 0,
        }
    }
}

impl TryFrom<&str> for IntCode {
    type Error = &'static str;
    fn try_from(text: &str) -> Result<Self, Self::Error> {
        let data = parse_program(text);

        Err("Fuck")
    }
}

mod int_code_computer {
    #[derive(Debug, Clone, Copy)]
    pub enum Param {
        Positional,
        Immediate,
        Invalid,
    }

    #[derive(Debug)]
    pub enum Opcode {
        Add { in1: Param, in2: Param, out: Param },
        Mult { in1: Param, in2: Param, out: Param },
        Stop,
        Input(Param),
        Output(Param),
        Err,
    }
    trait ParseParam {
        fn parse_params(input: u32);
    }

    impl From<u32> for Opcode {
        fn from(x: u32) -> Self {
            let op = if x > 99 { x % 100 } else { x };
            let param = x / 100;

            let mut result: Vec<Param> = Vec::with_capacity(3);

            for _ in 0..2 {
                result.push(match param % 10 {
                    0 => Param::Positional,
                    1 => Param::Immediate,
                    _ => Param::Invalid,
                });
            }

            result.reverse();
            let mut x = result.iter();

            match op {
                1 => Opcode::Add {
                    in1: *x.next().unwrap(),
                    in2: *x.next().unwrap(),
                    out: *x.next().unwrap(),
                },
                2 => Opcode::Mult {
                    in1: *x.next().unwrap(),
                    in2: *x.next().unwrap(),
                    out: *x.next().unwrap(),
                },
                3 => Opcode::Input(*x.next().unwrap()),
                4 => Opcode::Output(*x.next().unwrap()),
                99 => Opcode::Stop,
                _ => Opcode::Err,
            }
        }
    }
    impl Opcode {
        pub fn len(&self) -> usize {
            match self {
                Opcode::Add {
                    in1: _,
                    in2: _,
                    out: _,
                } => 4,
                Opcode::Mult {
                    in1: _,
                    in2: _,
                    out: _,
                } => 4,
                Opcode::Input(_) => 2,
                Opcode::Output(_) => 2,
                Opcode::Stop => 1,
                Opcode::Err => 0,
            }
        }
    }

    struct IntComputer {
        initial_state: Vec<i32>,
        state: Vec<i32>,
        pc: usize,
        in1: isize,
        in2: isize,
        out: isize,
        op: Opcode,
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let correct = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        let input = String::from("1,9,10,3,2,3,11,0,99,30,40,50");
        let result = parse_program(&input);
        assert_eq!(true, result.is_ok());
        assert_eq!(correct, result.unwrap());
    }
}
