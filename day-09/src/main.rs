use std::error::Error;
use std::io::{self, Read, Write};
use std::result;

type Result<T> = result::Result<T, Box<dyn Error>>;

fn has_sum(slice: &[i64], target: i64) -> bool {

    for i in 0..slice.len() {
        for j in (i + 1)..slice.len() {
            if slice[i] + slice[j] == target {
                return true;
            }
        }
    }

    return false;
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let message: Vec<i64> = input.lines().map(|n| n.parse().unwrap()).collect();

    let prefix: usize = 25;

    let mut invalid_num = 0;
    for i in prefix..message.len() {
        if !has_sum(&message[(i-prefix)..i], message[i]) {
            invalid_num = message[i];
            break;
        }
    }

    writeln!(io::stdout(), "Answer 1: {}", invalid_num)?;

    let mut cum_sum: Vec<i64> = vec![0; message.len()];
    cum_sum[0] = message[0];
    for i in 1..cum_sum.len() {
        cum_sum[i] = message[i] + cum_sum[i - 1];
    }

    for i in 0..cum_sum.len() {
        for j in (i + 2)..cum_sum.len() {
            if cum_sum[j] - cum_sum[i] == invalid_num {

                let ans = **(&message[(i + 1)..=j].iter().max().unwrap()) + **(&message[(i + 1)..=j].iter().min().unwrap());

                writeln!(io::stdout(), "Answer 2: {}", ans)?;
            }
        }
    }

    Ok(())
}
