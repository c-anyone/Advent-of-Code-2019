#![feature(test)]
extern crate test;
use std::convert::TryFrom;
use std::fs::File;
use std::io::{BufRead, BufReader};

mod int_code;
use int_code::*;

fn main() {
    let reader = BufReader::new(File::open("input_day5_part1.txt").expect("File not found!"));
    let mut buf = String::new();

    for line in reader.lines() {
        let line = line.unwrap();
        buf.push_str(&line);
    }
    // let mut buf = String::from("1,9,10,3,2,3,11,0,99,30,40,50");

    let mut int_computer: int_code::IntComputer =
        IntComputer::try_from(buf.as_str()).unwrap();

    loop {
        let x = int_computer.step();
        match x {
            Ok(true) => continue,
            Ok(false) => (),
            Err(x) => println!("Execution halted with an Error {}", x),
        };
        break;
    }

    // println!("Execution resulted in {}", int_computer.get(0).unwrap());
    // let program_data = parse_program(&buf).unwrap();

    // let mut ga = IntCodeProgram::new(&program_data);
    // // println!{"The Result is {}", ga.run()};

    // 'outer: for noun in 0..99 {
    //     for verb in 0..99 {
    //         if ga.run_with(noun, verb) == 19690720 {
    //             println!("Found solution at {} {}", noun, verb);
    //             println!("Answer is {}", 100 * noun + verb);
    //             break 'outer;
    //         }
    //         ga.reset();
    //     }
    // }
}

