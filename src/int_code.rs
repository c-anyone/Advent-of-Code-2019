// #[derive(Debug, PartialEq)]
// pub enum Parameter {
//     Positional,
//     Immediate,
// }

// #[derive(Debug)]
// pub struct IntCode {
//     opcode: Opcode,
//     in1: usize,
//     in2: usize,
//     out: usize,
// }

// impl From<&[i32]> for IntCode {
//     fn from(x: &[i32]) -> Self {
//         IntCode {
//             opcode: Opcode::from(x[0]),
//             in1: x[1] as usize,
//             in2: x[2] as usize,
//             out: x[3] as usize,
//         }
//     }
// }

// #[derive(Debug, PartialEq)]
// pub enum Opcode {
//     Add,
//     Mult,
//     Stop,
//     Input,
//     Output,
//     Err,
// }

// impl From<i32> for Opcode {
//     fn from(x: i32) -> Self {
//         let op = if x > 99 { x % 100 } else { x };
//         let param = x - op;
//         match op {
//             1 => Opcode::Add,
//             2 => Opcode::Mult,
//             3 => Opcode::Input,
//             4 => Opcode::Output,
//             99 => Opcode::Stop,
//             _ => Opcode::Err,
//         }
//     }
// }

// impl Opcode {
//     pub fn len(&self) -> usize {
//         match self {
//             Opcode::Add => 4,
//             Opcode::Mult => 4,
//             Opcode::Input => 2,
//             Opcode::Output => 2,
//             Opcode::Stop => 1,
//             Opcode::Err => 0,
//         }
//     }
// }

// impl IntCodeProgram {
//     pub fn current_instruction(&self) -> String {
//         let x = String::from(format!("{:?}", self.parse_current_instruction()));
//         x
//     }

//     pub fn exec_step(&mut self) -> Option<()> {
//         // let inst = &self.instructions[self.pc];
//         let inst = self.parse_current_instruction().unwrap();
//         let mut ret = match inst.opcode {
//             Opcode::Add => {
//                 self.state[inst.out] = self.state[inst.in1] + self.state[inst.in2];
//                 Some(())
//             }
//             Opcode::Mult => {
//                 self.state[inst.out] = self.state[inst.in1] * self.state[inst.in2];
//                 Some(())
//             }
//             Opcode::Input => None,
//             Opcode::Output => None,
//             Opcode::Stop => None,
//             Opcode::Err => {
//                 println!("Illegal Opcode {}", self.state[self.pc]);
//                 None
//             }
//         };
//         self.pc += 4;
//         if self.pc >= self.state.len() {
//             self.pc = 0;
//             ret = None;
//         }
//         ret
//     }

//     pub fn run(&mut self) -> i32 {
//         while self.exec_step() != None {}
//         self.state[0]
//     }

//     pub fn run_with(&mut self, noun: i32, verb: i32) -> i32 {
//         self.state[1] = noun;
//         self.state[2] = verb;
//         self.pc = 0;
//         self.run()
//     }

//     pub fn reset(&mut self) {
//         self.state = self.initial_state.clone();
//         self.pc = 0;
//     }

//     fn parse_current_instruction(&self) -> Option<IntCode> {
//         let op: Opcode = self.state[self.pc].into();

//         let slice = &self.state[self.pc..self.pc + op.len()];

//         // IntCode::from(slice);
//         match IntCode::try_from(slice) {
//             Ok(x) => Some(x),
//             Err(_) => None,
//         }
//     }

//     pub fn new(data: &Vec<i32>) -> Self {
//         // let test = data.iter();
//         // let state: Vec<i32> = data.iter().map(|x| (*x).from()).collect();
//         IntCodeProgram {
//             initial_state: data.clone(),
//             state: data.clone(),
//             // instructions: IntCodeProgram::parse_instructions(data).unwrap(),
//             pc: 0,
//         }
//     }
// }

mod int_code_computer {
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

        fn load_param(&self, value: isize, param: Param) -> Result<i32, String> {
            match param {
                Param::Positional => match self.state.get(value as usize) {
                    Some(&p) => Ok(p),
                    None => Err(format!("Index {} out of Bounds", value)),
                },
                Param::Immediate => Ok(value as i32),
                Param::Invalid => Err(format!("Illegal param, value {}", value)),
            }
        }

        fn store_result(
            &mut self,
            value: i32,
            param: Param,
            location: usize,
        ) -> Result<(), String> {
            if location > self.state.len() {
                return Err(format!(
                    "Failed to store value@location: {}@{}",
                    value, location
                ));
            }
            self.state[location] = value;
            Ok(())
        }

        fn execute_loaded_instruction(&mut self) -> Result<Opcode, String> {
            self.op = match self.load_instruction_at_pc() {
                Ok(op) => op,
                Err(e) => return Err(e),
            };
            let result = match self.op {
                Opcode::Add(params) => {
                    let input_iter = self.state.iter().skip(self.pc).take(3);
                    let reg_iter = self.register.iter_mut();
                    let iter = multizip((input_iter, reg_iter, params.iter()));

                    for (&inp, reg, &param) in iter {
                        // *reg = self.load_param(inp as isize, param)?;
                        *reg = match param {
                            Param::Positional => *(self.state.get(inp as usize).unwrap()),
                            Param::Immediate => inp,
                            Param::Invalid => {
                                return Err(format!("Invalid Parameter at {}", self.pc))
                            }
                        };
                    }
                    self.store_result(
                        self.register[0] + self.register[1],
                        params[2],
                        self.register[2] as usize,
                    )?;
                    Ok(())
                }
                Opcode::Mult(params) => {
                    let input_iter = self.state.iter().skip(self.pc).take(3);
                    let reg_iter = self.register.iter_mut();
                    let iter = multizip((input_iter, reg_iter, params.iter()));

                    for (&inp, reg, &param) in iter {
                        // *reg = self.load_param(inp as isize, param)?;
                        *reg = match param {
                            Param::Positional => *(self.state.get(inp as usize).unwrap()),
                            Param::Immediate => inp,
                            Param::Invalid => {
                                return Err(format!("Invalid Parameter at {}", self.pc))
                            }
                        };
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
                        self.load_param(index as isize, Param::Immediate)?
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
            // Err(format!("Fuck"))
        }

        // fn execute_instruction(&self, op: Opcode) -> Result<(), String> {
        //     // let inst = &self.instructions[self.pc];
        //     let mut ret = match op.opcode {
        //         Opcode::Add => {
        //             self.state[inst.out] = self.state[inst.in1] + self.state[inst.in2];
        //             Some(())
        //         }
        //         Opcode::Mult => {
        //             self.state[inst.out] = self.state[inst.in1] * self.state[inst.in2];
        //             Some(())
        //         }
        //         Opcode::Input => None,
        //         Opcode::Output => None,
        //         Opcode::Stop => None,
        //         Opcode::Err => {
        //             println!("Illegal Opcode {}", self.state[self.pc]);
        //             None
        //         }
        //     };
        //     self.pc += 4;
        //     if self.pc >= self.state.len() {
        //         self.pc = 0;
        //         ret = None;
        //     }
        //     ret
        //     // Err(format!("Fuck"))
        // }
    }
    // opcode = (*data.first().unwrap()).into()

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

            match op {
                1 => {
                    let mut array = [Param::Invalid; 3];
                    array.copy_from_slice(&result);
                    Opcode::Add(array)
                }
                2 => {
                    let mut array = [Param::Invalid; 3];
                    array.copy_from_slice(&result);
                    Opcode::Mult(array)
                }
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

    struct IntComputer {
        initial_state: Vec<i32>,
        state: Vec<i32>,
        pc: usize,
        register: [i32; 3],
        op: Opcode,
    }
}

mod tests {
    use super::int_code_computer::*;

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
