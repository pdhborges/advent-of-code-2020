use std::error::Error;
use std::io::{self, Read, Write};
use std::result;

use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;

type Result<T> = result::Result<T, Box<dyn Error>>;

struct Rule {
    c: char,
    lower: i32,
    upper: i32,
}

impl Rule {
    pub fn new(c: char, lower: i32, upper: i32) -> Rule {
        return Rule { c, lower, upper };
    }

    pub fn wrong_matches(&self, password: &str) -> bool {
        let occurences = password.matches(self.c).count() as i32;
        return occurences >= self.lower && occurences <= self.upper;
    }

    pub fn right_matches(&self, password: &str) -> bool {
        let lower_char = password.chars().nth((self.lower - 1) as usize).unwrap();
        let upper_char = password.chars().nth((self.upper - 1) as usize).unwrap();
        return lower_char != upper_char && (lower_char == self.c || upper_char == self.c);
    }
}

impl FromStr for Rule {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Rule> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\d+)-(\d+) (\w)").unwrap();
        }
        let groups = RE.captures(s).unwrap();
        return Ok(Rule::new(
            groups.get(3).unwrap().as_str().parse::<char>().unwrap(),
            groups.get(1).unwrap().as_str().parse::<i32>().unwrap(),
            groups.get(2).unwrap().as_str().parse::<i32>().unwrap(),
        ));
    }
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut valid_passwords = 0;
    let mut right_valid_passwords = 0;

    for line in input.lines() {
        let mut parts = line.split(": ");
        let rule = parts.next().unwrap();

        let password = parts.next().unwrap();
        let rule = rule.parse::<Rule>().unwrap();

        if rule.wrong_matches(password) {
            valid_passwords += 1;
        }

        if rule.right_matches(password) {
            right_valid_passwords += 1;
        }
    }

    writeln!(io::stdout(), "Answer 1: {}", valid_passwords)?;
    writeln!(io::stdout(), "Answer 2: {}", right_valid_passwords)?;
    return Ok(());
}
