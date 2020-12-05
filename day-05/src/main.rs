use std::collections::HashSet;
use std::error::Error;
use std::io::{self, Read, Write};
use std::result;

type Result<T> = result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let seats = input
        .lines()
        .map(|line| {
            let row =
                i32::from_str_radix(&line[0..7].replace("B", "1").replace("F", "0"), 2).unwrap();
            let seat =
                i32::from_str_radix(&line[7..10].replace("R", "1").replace("L", "0"), 2).unwrap();
            row * 8 + seat
        })
        .collect::<HashSet<i32>>();

    let max_seat_id = seats.iter().max().unwrap();

    writeln!(io::stdout(), "Answer 1: {}", max_seat_id)?;

    for seat in 0..1024 {
        if !seats.contains(&seat) && seats.contains(&(seat - 1)) && seats.contains(&(seat + 1)) {
            writeln!(io::stdout(), "Answer 2: {}", seat)?;
            break;
        }
    }

    Ok(())
}
