use crate::int_code::{IntComputer, IntComputerState, ValueType};
use std::convert::TryFrom;
use std::io::{prelude::*, BufRead, BufReader};
use std::fs::File;

pub fn day_9_run() -> std::io::Result<()> {
    let file = File::open("puzzle_input.txt")?;
    let mut input = String::new();
    let length = BufReader::new(file).read_to_string(&mut input)?;

    let mut program = IntComputer::try_from(input.as_str()).unwrap();
    program.push_input(1);
    program.run().unwrap();

    let result = program.get_output().unwrap();

    println!("BOOST keycode {}", result);
    Ok(())
}

#[cfg(test)]
mod Test {
    use super::*;
    #[test]
    fn test_day_9_quine() {
        let input = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";
        let mut program = IntComputer::try_from(input).unwrap();

        program.run().unwrap();
        let mut result = Vec::new();
        while let Some(x) = program.get_output() {
            // result += format! {"{},", x}.as_str();
            result.push(x);
        }
        println!("{:?}", result);
        result.pop();
        let inx: Vec<ValueType> = input.split(',').map(|x| x.parse::<ValueType>().unwrap()).collect();
        assert_eq!(result, inx);
    }

    #[test]
    fn test_day_9_16_digit() {
        let input = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";
        let mut program = IntComputer::try_from(input).unwrap();

        program.run().unwrap();

        let result = program.get_output().unwrap();

        let digits = result.to_string().len();

        println!("Digit Result: {}", result);

        assert_eq!(result, 1219070632396864);
        assert_eq!(digits, 16);
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
