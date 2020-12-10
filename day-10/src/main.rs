use std::error::Error;
use std::io::{self, Read, Write};
use std::result;

type Result<T> = result::Result<T, Box<dyn Error>>;

const WALL_SOCKET: i32 = 0;

fn arrangements_to(memo: &mut Vec<i64>, adapters: &Vec<i32>, pos: i64) -> i64 {
    if memo[pos as usize] != -1 {
        return memo[pos as usize];
    }

    let mut arrangements = 0;
    let mut pred = pos - 1;

    while pred >= 0 && adapters[pos as usize] - adapters[pred as usize] < 4 { // sorted assumption
        arrangements += arrangements_to(memo, adapters, pred);
        pred -= 1;
    }

    memo[pos as usize] = arrangements;

    return arrangements;
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut adapters: Vec<i32> = input.lines().map(|line| line.parse().unwrap()).collect();

    let builtin_adapter = adapters.iter().max().unwrap() + 3;

    adapters.push(WALL_SOCKET);
    adapters.push(builtin_adapter);
    adapters.sort();

    let mut one_jolt_diff = 0;
    let mut three_jolt_diff = 0;

    for i in 1..adapters.len() {
        let diff = adapters[i] - adapters[i - 1];
        if diff == 1 {
            one_jolt_diff += 1;
        }
        if diff == 3 {
            three_jolt_diff += 1;
        }
    }

    writeln!(io::stdout(), "Answer 1: {}", one_jolt_diff * three_jolt_diff)?;

    let mut memo: Vec<i64> = vec![-1; adapters.len()];
    memo[0] = 1; // one way do arranje the wall socket
    let ans = arrangements_to(&mut memo, &adapters, (adapters.len() - 1) as i64);
    writeln!(io::stdout(), "Answer 2: {}", ans)?;

    Ok(())
}
