use std::error::Error;
use std::io::{self, Read, Write};
use std::result;

type Result<T> = result::Result<T, Box<dyn Error>>;

fn pos_rem(rem: i128, multiple: i128) -> i128 {
    let mut result = rem;
    while result + multiple < multiple {
        result += multiple;
    }
    return result;
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut lines = input.lines();

    let departure: i64 = lines.next().unwrap().parse().unwrap();
    let schedule: Vec<&str> = lines.next().unwrap().split(",").collect();

    let selected_bus = schedule
        .iter()
        .filter(|bus| **bus != "x")
        .map(|bus| bus.parse::<i64>().unwrap())
        .min_by_key(|bus| bus - (departure % bus))
        .unwrap();

    writeln!(io::stdout(), "Answer 1: {}", selected_bus * (selected_bus - (departure % selected_bus)))?;

    // sieve CRT

    let mut indexed_buses: Vec<(i128, i128)> = schedule
        .iter()
        .enumerate()
        .filter(|bus| *bus.1 != "x")
        .map(|bus| (pos_rem(-(bus.0 as i128), bus.1.parse::<i128>().unwrap()), bus.1.parse::<i128>().unwrap()))
        .collect();
    
    indexed_buses.sort_by_key(|e| -e.1);
    
    let mut curr_sol = indexed_buses[0].0;
    let mut curr_sum = indexed_buses[0].1;

    for i in 1..indexed_buses.len() {
        let mut sol = curr_sol;
        while sol % indexed_buses[i].1 != indexed_buses[i].0 {
            sol += curr_sum;
        }
        curr_sol = sol;
        curr_sum *= indexed_buses[i].1;
    }

    writeln!(io::stdout(), "Answer 2: {}", curr_sol)?;

    Ok(())
}
