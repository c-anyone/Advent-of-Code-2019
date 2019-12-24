pub mod int_code {
    use std::convert::TryFrom;

    #[derive(Debug, PartialEq)]
    pub enum Opcode {
        Add,
        Mult,
        Stop,
        Input,
        Output,
        Err,
    }

    #[derive(Debug, PartialEq)]
    pub enum Parameter {
        Positional(isize),
        Immediate(isize),
    }

    #[derive(Debug)]
    pub struct IntCode {
        opcode: Opcode,
        in1: usize,
        in2: usize,
        out: usize,
    }
    impl From<u32> for Opcode {
        fn from(x: u32) -> Self {
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
        initial_state: Vec<u32>,
        state: Vec<u32>,
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

        pub fn run(&mut self) -> u32 {
            while self.exec_step() != None {}
            self.state[0]
        }

        pub fn run_with(&mut self, noun: u32, verb: u32) -> u32 {
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
            if self.state.len() - self.pc < 3 {
                return None;
            }
            let slice = &self.state[self.pc..self.pc + 4];

            // IntCode::from(slice);
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
            IntCodeProgram {
                initial_state: state.clone(),
                state: state.to_owned(),
                // instructions: IntCodeProgram::parse_instructions(data).unwrap(),
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
}
