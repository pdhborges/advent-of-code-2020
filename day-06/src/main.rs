use std::collections::HashSet;
use std::error::Error;
use std::io::{self, Read, Write};
use std::result;

type Result<T> = result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let group_answers = input.split("\n\n").collect::<Vec<&str>>();

    let yeses: usize = group_answers
        .iter()
        .map(|group_answer| {
            group_answer.replace("\n", "").chars().collect::<HashSet<char>>().len()
        })
        .sum();
    
    writeln!(io::stdout(), "Answer 1: {}", yeses)?;

    let all_yeses: usize = group_answers
        .iter()
        .map(|group_answer| {
            let mut yes_answers = ('a'..='z').collect::<HashSet<_>>();
            for answer in group_answer.lines() {
                yes_answers = &yes_answers & &answer.chars().collect::<HashSet<char>>();
            }
            return yes_answers.len();
        })
        .sum();

    writeln!(io::stdout(), "Answer 2: {}", all_yeses)?;

    Ok(())
}
