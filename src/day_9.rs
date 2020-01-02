use crate::int_code::{IntComputer, IntComputerState};
use std::convert::TryFrom;

#[cfg(test)]
mod Test {
    use super::*;
    #[test]
    fn test_day_9_quine() {
        let input = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";
        let mut program = IntComputer::try_from(input).unwrap();

        program.run().unwrap();
        let mut result = String::new();
        while let Some(x) = program.get_output() {
            result += format!{"{},", x}.as_str();
        }

        assert_eq!(result, input);
    }

    #[test]
    fn test_day_9_16_digit() {
        let input = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";
        let mut program = IntComputer::try_from(input).unwrap();

        program.run().unwrap();

        let result = program.get_output().unwrap();
        println!("Digit Result: {}", result);


    }

    #[test]
    fn test_day_9_middle() {
        let input = "104,1125899906842624,99";
        let mut program = IntComputer::try_from(input).unwrap();

        program.run().unwrap();

        let result = program.get_output().unwrap();
        assert_eq!(result, 1125899906842624);
    }
}
