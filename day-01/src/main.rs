use std::error::Error;
use std::io::{self, Read, Write};
use std::result;

type Result<T> = result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut nums: Vec<i32> = Vec::new();
    for line in input.lines() {
        let num = line.parse::<i32>()?;
        nums.push(num);
    }

    // Brute-force since input only has 200 lines

    for i in 0..nums.len() {
        for j in i..nums.len() {
            if nums[i] + nums[j] == 2020 {
                writeln!(io::stdout(), "Answer 1: {}", (nums[i] * nums[j]))?;
            }
        }
    }

    for i in 0..nums.len() {
        for j in i..nums.len() {
            for y in j..nums.len() {
                if nums[i] + nums[j] + nums[y] == 2020 {
                    writeln!(io::stdout(), "Answer 2: {}", (nums[i] * nums[j] * nums[y]))?;
                }
            }
        }
    }

    return Ok(());
}
