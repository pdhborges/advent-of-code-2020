use std::error::Error;
use std::io::{self, Read, Write};
use std::result;
use num_complex::Complex;

type Result<T> = result::Result<T, Box<dyn Error>>;

const NORTH: Complex<i64> = Complex::new(0, 1);
const SOUTH: Complex<i64> = Complex::new(0, -1);
const EAST: Complex<i64> = Complex::new(1, 0);
const WEST: Complex<i64> = Complex::new(-1, 0);

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut position: Complex<i64> = Complex::new(0, 0);
    let mut direction: Complex<i64> = EAST;

    for line in input.lines() {

        let inst = &line[0..1];
        let value: i64 = line[1..line.len()].parse().unwrap();

        if inst == "N" {
            position += NORTH * value;
        }

        if inst == "S" {
            position += SOUTH * value;
        }

        if inst == "E" {
            position += EAST * value;
        }

        if inst == "W" {
            position += WEST * value;
        }

        if inst == "L" {
            direction = direction * Complex::i().powi(value as i32/  90);
        }

        if inst == "R" {
            direction = direction * (-Complex::i()).powi(value as i32 / 90);
        }

        if inst == "F" {
            position += direction * value;
        }
    }

    writeln!(io::stdout(), "Answer 1: {}", position.l1_norm())?;

    let mut position: Complex<i64> = Complex::new(0, 0);
    let mut waypoint: Complex<i64> = Complex::new(10, 1);

    for line in input.lines() {

        let inst = &line[0..1];
        let value: i64 = line[1..line.len()].parse().unwrap();

        if inst == "N" {
            waypoint += NORTH * value;
        }

        if inst == "S" {
            waypoint += SOUTH * value;
        }

        if inst == "E" {
            waypoint += EAST * value;
        }

        if inst == "W" {
            waypoint += WEST * value;
        }

        if inst == "L" {
            waypoint = waypoint * Complex::i().powi(value as i32/  90);
        }

        if inst == "R" {
            waypoint = waypoint * (-Complex::i()).powi(value as i32 / 90);
        }

        if inst == "F" {
            position += waypoint * value;
        }
    }

    writeln!(io::stdout(), "Answer 2: {}", position.l1_norm())?;


    Ok(())
}
