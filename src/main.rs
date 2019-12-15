use std::convert::TryFrom;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, PartialEq)]
enum Opcode {
    Add,
    Mult,
    Stop,
    Err,
}

#[derive(Debug)]
struct IntCode {
    opcode: Opcode,
    in1: usize,
    in2: usize,
    out: usize,
}

fn parse_program(input: &str) -> Vec<u8> {
    input.split(',').map(|s| s.parse::<u8>().unwrap()).collect()
}

impl From<u32> for Opcode {
    fn from(x: u32) -> Self {
        match x {
            1 => Opcode::Add,
            2 => Opcode::Mult,
            99 => Opcode::Stop,
            _ => Opcode::Err,
        }
    }
}

impl From<&[u32]> for IntCode {
    fn from(x: &[u32]) -> Self {
        IntCode {
            opcode: Opcode::from(x[0]),
            in1: x[1] as usize,
            in2: x[2] as usize,
            out: x[3] as usize,
        }
    }
}

fn main() {
    let reader = BufReader::new(File::open("input.txt").expect("File not found!"));
    let mut buf = String::new();

    for line in reader.lines() {
        let line = line.unwrap();
        buf.push_str(&line);
    }
    // let mut buf = String::from("1,9,10,3,2,3,11,0,99,30,40,50");
    let mut program_data = parse_program(&buf);

    let mut ga = GravityAssist::new(&program_data);
    println!("{:?}", ga.state);
    while ga.exec_step() != None {
        println!("{}:{}", ga.pc, ga.current_instruction());
    }

    print!("{:?}", ga.state);
}

struct GravityAssist {
    state: Vec<u32>,
    pc: usize,
}

impl GravityAssist {
    pub fn current_instruction(&self) -> String {
        let x = String::from(format!("{:?}", self.parse_current_instruction()));
        x
    }

    pub fn set_1202_error_state(&mut self) {
        self.state[1] = 12;
        self.state[2] = 2;
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
            Opcode::Stop => {
                println!("Stop Code");
                None
            }
            Opcode::Err => {
                println!("Illegal Opcode {}", self.state[self.pc]);
                None
            }
        };
        self.pc += 4;
        if self.pc >= self.state.len() - 3 {
            self.pc = self.state.len() - 3;
            ret = None;
        }
        ret
    }

    fn parse_current_instruction(&self) -> Option<IntCode> {
        if self.state.len() - self.pc < 3 {
            return None;
        }
        let slice = &self.state[self.pc..self.pc + 4];

        IntCode::try_from(slice);
        match IntCode::try_from(slice) {
            Ok(x) => Some(x),
            Err(_) => None,
        }
    }

    // fn parse_instructions(data: &Vec<u8>) -> Option<Vec<IntCode>> {
    //     let index = data.iter().position(|x| *x == 99).expect("Opcode Stop not found!");
    //     let (dat_short, _) = data.split_at(index);

    //     let instructions = dat_short
    //         .chunks_exact(4)
    //         .map(|x| IntCode {
    //             opcode: match x[0] {
    //                 1 => Some(Opcode::Add),
    //                 2 => Some(Opcode::Mult),
    //                 99 => Some(Opcode::Stop),
    //                 _ => None,
    //             }
    //             .expect("illegal Opcode"),
    //             in1: x[1] as usize,
    //             in2: x[2] as usize,
    //             out: x[3] as usize,
    //         })
    //         .collect();
    //     Some(instructions)
    // }

    pub fn new(data: &Vec<u8>) -> Self {
        // let test = data.iter();
        let state: Vec<u32> = data.iter().map(|x| *x as u32).collect();
        GravityAssist {
            state: state.to_owned(),
            // instructions: GravityAssist::parse_instructions(data).unwrap(),
            pc: 0,
        }
    }
}

mod tests {

    #[test]
    fn test_new() {
        // let test_input = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
    }
}
