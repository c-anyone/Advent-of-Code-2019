use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, PartialEq)]
enum Opcode {
    add,
    mult,
    stop,
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

impl IntCode {
    fn verify_len(&self, len: usize) -> bool {
        len > self.in1 as usize && len > self.in2 as usize && len > self.out as usize
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
    let program_data = parse_program(&buf);
    let mut ga = GravityAssist::new(&program_data);
    let mut program_data = program_data;

    while ga.exec_step() != None {
        println!("{}", ga.current_instruction());
    }

    print!("{:?}", ga.state);
}

struct GravityAssist {
    data: Vec<u8>,
    state: Vec<u32>,
    instructions: Vec<IntCode>,
    pc: usize,
}

impl GravityAssist {
    pub fn current_instruction(&self) -> String {
        let x = String::from(format!("{:?}", &self.instructions[self.pc]));
        x
    }

    pub fn exec_step(&mut self) -> Option<Opcode> {
        let inst = &self.instructions[self.pc];
        let mut ret = match inst.opcode {
            Opcode::add => {
                self.state[inst.out] = self.state[inst.in1] + self.state[inst.in2];
                Some(Opcode::add)
            }
            Opcode::mult => {
                self.state[inst.out] = self.state[inst.in1] * self.state[inst.in2];
                Some(Opcode::mult)
            }
            Opcode::stop => {
                println!("ERROR");
                None
            }
        };
        self.pc += 1;
        if self.pc >= self.instructions.len() {
            self.pc = self.instructions.len() - 1;
            ret = None;
        }
        ret
    }

    fn parse_instructions(data: &Vec<u8>) -> Option<Vec<IntCode>> {
        let index = data.iter().position(|x| *x == 99).unwrap();
        let (dat_short, _) = data.split_at(index);

        Some(
            dat_short
                .chunks_exact(4)
                .map(|x| IntCode {
                    opcode: match x[0] {
                        1 => Some(Opcode::add),
                        2 => Some(Opcode::mult),
                        99 => Some(Opcode::stop),
                        _ => None,
                    }
                    .unwrap(),
                    in1: x[1] as usize,
                    in2: x[2] as usize,
                    out: x[3] as usize,
                })
                .collect(),
        )
    }

    pub fn new(data: &Vec<u8>) -> Self {
        // let test = data.iter();
        let state: Vec<u32> = data.iter().map(|x| *x as u32).collect();
        GravityAssist {
            data: data.to_owned(),
            state: state.to_owned(),
            instructions: GravityAssist::parse_instructions(data).unwrap(),
            pc: 0,
        }
    }
}

mod tests {

    #[test]
    fn test_new() {
        let test_input = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
    }
}
