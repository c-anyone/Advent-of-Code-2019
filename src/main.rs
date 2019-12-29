#![feature(test)]
extern crate test;
use std::convert::TryFrom;
use std::fs::File;
use std::io::{BufRead, BufReader};

mod int_code;
use int_code::*;

mod day_6;
// use day_6;

fn day_5_run() {
    let reader = BufReader::new(File::open("input_day5_part1.txt").expect("File not found!"));
    let mut buf = String::new();
    for line in reader.lines() {
        let line = line.unwrap();
        buf.push_str(&line);
    }
    // let mut buf = String::from("1,9,10,3,2,3,11,0,99,30,40,50");
    let mut int_computer: IntComputer = IntComputer::try_from(buf.as_str()).unwrap();
    int_computer.push_input(1);
    match int_computer.run() {
        Ok(()) => println!("Input 1 OK"),
        Err(x) => println!("{}", x)
    }

    let mut int_computer: IntComputer = IntComputer::try_from(buf.as_str()).unwrap();
    int_computer.push_input(5);
    match int_computer.run() {
        Ok(()) => println!("Input 5 OK"),
        Err(x) => println!("{}", x)
    }
}

fn day_6_run() {
    day_6::run();
}

fn main() {

    day_5_run();
    day_6_run();
}
