#![feature(test)]
extern crate test;
use std::fs::File;
use std::io::{BufRead, BufReader};

mod int_code;
use int_code::*;

fn main() {
    let reader = BufReader::new(File::open("input.txt").expect("File not found!"));
    let mut buf = String::new();

    for line in reader.lines() {
        let line = line.unwrap();
        buf.push_str(&line);
    }
    // let mut buf = String::from("1,9,10,3,2,3,11,0,99,30,40,50");
    let program_data = parse_program(&buf);

    let mut ga = IntCodeProgram::new(&program_data);
    // println!{"The Result is {}", ga.run()};

    'outer: for noun in 0..99 {
        for verb in 0..99 {
            if ga.run_with(noun, verb) == 19690720 {
                println!("Found solution at {} {}", noun, verb);
                println!("Answer is {}", 100 * noun + verb);
                break 'outer;
            }
            ga.reset();
        }
    }
}

mod tests {
    use test::Bencher;

    #[derive(Debug, PartialEq)]
    pub enum Parameter {
        Positional,
        Immediate,
        Invalid,
    }

    #[bench]
    fn bench_string(b: &mut Bencher) {
        b.iter(|| {
            let iter = (0..999).into_iter();
            for i in iter {
                let s = format!("{:03}", i);
                for val in s.chars() {
                    match val {
                        '0' => Parameter::Positional,
                        '1' => Parameter::Immediate,
                        _ => Parameter::Invalid,
                    };
                }
            }
        });
    }

    #[bench]
    fn bench_modulo(b: &mut Bencher) {
        b.iter(|| {
            let iter = (0..999).into_iter();
            for i in iter {
                let mut result = vec![Parameter::Invalid, Parameter::Invalid, Parameter::Invalid];
                let mut val = i;
                for j in 0..2 {
                    result[j] = match val % 10 {
                        0 => Parameter::Positional,
                        1 => Parameter::Immediate,
                        _ => Parameter::Invalid
                    };
                    val = val / 10;
                }
            }
        });
    }
}
