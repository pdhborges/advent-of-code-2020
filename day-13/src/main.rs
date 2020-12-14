use std::error::Error;
use std::io::{self, Read, Write};
use std::result;

type Result<T> = result::Result<T, Box<dyn Error>>;

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


    let indexed_buses: Vec<(usize, i64)> = schedule
        .iter()
        .enumerate()
        .filter(|bus| *bus.1 != "x")
        .map(|bus| (bus.0, bus.1.parse::<i64>().unwrap()))
        .collect();

    let mut wolphram_command = String::new();
    wolphram_command.push_str("ChineseRemainder[{");
    wolphram_command.push_str(&indexed_buses.iter().map(|e| (e.1 - e.0 as i64).to_string()).collect::<Vec<String>>().join(","));
    wolphram_command.push_str("}, {");
    wolphram_command.push_str(&indexed_buses.iter().map(|e| e.1.to_string()).collect::<Vec<String>>().join(","));
    wolphram_command.push_str("}]");

    writeln!(io::stdout(), "Answer 2: {}", wolphram_command.as_str())?;
    
    Ok(())
}
