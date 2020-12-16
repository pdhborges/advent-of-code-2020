use std::error::Error;
use std::io::{self, Read, Write};
use std::result;
use std::collections::{HashMap, VecDeque};

type Result<T> = result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut last_spoken = -1 as i64;
    let mut last_pos : HashMap<i64, VecDeque<usize>> = HashMap::new();
    for (i, n) in input.split(",").enumerate() {
        last_spoken = n.parse().unwrap();
        last_pos.entry(last_spoken).or_insert(VecDeque::new()).push_back(i + 1);
    }

    for i in last_pos.len()..30000000 {
        let i = i + 1;
        if last_pos[&last_spoken].len() == 1 {
            last_spoken = 0;
        } else {
            let entry = last_pos.get_mut(&last_spoken).unwrap();
            let diff =  entry[1] - entry[0];
            entry.pop_front();
            last_spoken = diff as i64;
        }
        last_pos.entry(last_spoken).or_insert(VecDeque::new()).push_back(i);
    }

    writeln!(io::stdout(), "Answer 2: {}", last_spoken)?;

    Ok(())
}
