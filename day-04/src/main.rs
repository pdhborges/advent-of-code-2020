use std::error::Error;
use std::io::{self, Read, Write};
use std::result;
use regex::Regex;

type Result<T> = result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let passports = input.split("\n\n").map(|line| line.trim()).collect::<Vec<&str>>();

    let mandatory_fields: Vec<&str> = vec!["byr:", "iyr:", "eyr:", "hgt:", "hcl:", "ecl:", "pid:"];

    let mut valid_passports = 0;

    for passport in &passports {
        if mandatory_fields.iter().all(|field| passport.contains(field)) {
            valid_passports += 1;
        }
    }

    writeln!(io::stdout(), "Answer 1: {}", valid_passports)?;

    let mandatory_field_formats: Vec<fn(&str) -> bool> = vec![
        |passport| {
            let r = Regex::new(r"\bbyr:(\d{4})\b").unwrap();
            let c = r.captures(passport);
            if let Some(m) = c {
                if let Some(v) = m.get(1) {
                    let d = v.as_str().parse::<i32>().unwrap();
                    return d >= 1920 && d <= 2002;
                }
                return false;
            }
            return false;
        },
        |passport| {
            let r = Regex::new(r"\biyr:(\d{4})\b").unwrap();
            let c = r.captures(passport);
            if let Some(m) = c {
                if let Some(v) = m.get(1) {
                    let d = v.as_str().parse::<i32>().unwrap();
                    return d >= 2010 && d <= 2020;
                }
                return false;
            } 
            return false;
        },
        |passport| {
            let r = Regex::new(r"\beyr:(\d{4})\b").unwrap();
            let c = r.captures(passport);
            if let Some(m) = c {
                if let Some(v) = m.get(1) {
                    let d = v.as_str().parse::<i32>().unwrap();
                    return d >= 2020 && d <= 2030;
                }
                return false;
            }
            return false;
        },
        |passport| {
            let r = Regex::new(r"\bhgt:(\d+)(cm|in)\b").unwrap();
            let c = r.captures(passport);
            if let Some(m) = c {
                if let Some(v) = m.get(1) {
                    if let Some(u) = m.get(2) {
                        let d = v.as_str().parse::<i32>().unwrap();
                        if u.as_str() == "cm" {
                            return d >= 150 && d <= 193;
                        } else {
                            return d >= 59 && d <= 76;
                        }
                    }
                    return false;
                }
                return false;
            }
            return false;
        },
        |passport| {
            let r = Regex::new(r"\bhcl:#([0-9a-f]{6})\b").unwrap();
            return r.find(passport).is_some();
        },
        |passport| {
            let r = Regex::new(r"\becl:(amb|blu|brn|gry|grn|hzl|oth)\b").unwrap();
            return r.find(passport).is_some();
        },
        |passport| {
            let r = Regex::new(r"\bpid:(\d{9})\b").unwrap();
            return r.find(passport).is_some();
        },
    ];

    valid_passports = 0;

    for passport in &passports {
        if mandatory_field_formats.iter().all(|fmt| fmt(passport)) {
            valid_passports += 1;
        }
    }

    writeln!(io::stdout(), "Answer 2: {}", valid_passports)?;

    Ok(())
}
