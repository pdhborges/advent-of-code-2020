use std::error::Error;
use std::io::{self, Read, Write};
use std::result;

use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;

type Result<T> = result::Result<T, Box<dyn Error>>;

enum Op {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

impl FromStr for Op {

    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Op> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\w+) ([-+]\d+)").unwrap();
        }
        let groups = RE.captures(s).unwrap();

        let op = groups.get(1).unwrap().as_str();

        if op == "nop" {
            return Ok(Op::Nop(groups.get(2).unwrap().as_str().parse::<i32>().unwrap()));
        } else if op == "acc" {
            return Ok(Op::Acc(groups.get(2).unwrap().as_str().parse::<i32>().unwrap()));
        } else {
            return Ok(Op::Jmp(groups.get(2).unwrap().as_str().parse::<i32>().unwrap()));
        }

    }


}

type Program = Vec<Op>;

enum ProgramResult {
    Acc(i32),
    Loop,
    Crash
}

fn run_program_until_loop(program: &Program) -> i32 {
    let mut acc = 0;
    let mut visited_instructions = vec![false; program.len()];
    let mut pc: usize = 0;
    while !visited_instructions[pc] {
        visited_instructions[pc] = true;
        match program[pc] {
            Op::Nop(_) => {
                pc += 1;
            }
            Op::Acc(x) => {
                pc += 1;
                acc += x; 
            }
            Op::Jmp(x) => {
                pc = (pc as i32 + x) as usize;
            }
        }
    }
    return acc;
}

fn run_program(program: &Program) -> ProgramResult {

    let mut acc = 0;
    let mut visited_instructions = vec![false; program.len()];
    let mut pc: i32 = 0;

    while pc >= 0 && pc < program.len() as i32 && !visited_instructions[pc as usize] {
        visited_instructions[pc as usize] = true;
        match program[pc as usize] {
            Op::Nop(_) => {
                pc += 1;
            }
            Op::Acc(x) => {
                pc += 1;
                acc += x; 
            }
            Op::Jmp(x) => {
                pc += x;
            }
        }
    }

    if pc < 0 {
        return ProgramResult::Crash;
    }

    if pc == program.len() as i32 {
        return ProgramResult::Acc(acc);
    }

    if pc > program.len() as i32 {
        return ProgramResult::Crash;
    }

    return ProgramResult::Loop;
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut program = Program::new();
    for line in input.lines() {
        program.push(line.parse::<Op>().unwrap());
    }

    writeln!(io::stdout(), "Answer 1: {}", run_program_until_loop(&program))?;

    for inst_pos in 0..program.len() {

        program[inst_pos] = match program[inst_pos] {
            Op::Nop(x) => Op::Jmp(x),
            Op::Jmp(x) => Op::Nop(x),
            Op::Acc(x) => Op::Acc(x)
        };

        match run_program(&program) {
            ProgramResult::Crash => (),
            ProgramResult::Loop => (),
            ProgramResult::Acc(x) => {
                writeln!(io::stdout(), "Answer 2: {}", x)?;
                break;
            }
        }

        program[inst_pos] = match program[inst_pos] {
            Op::Nop(x) => Op::Jmp(x),
            Op::Jmp(x) => Op::Nop(x),
            Op::Acc(x) => Op::Acc(x)
        };
    }

    return Ok(());
}
