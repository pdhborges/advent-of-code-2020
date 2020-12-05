use std::error::Error;
use std::io::{self, Read, Write};
use std::result;

type Result<T> = result::Result<T, Box<dyn Error>>;

static TREE : u8 = '#' as u8;

fn hit_trees(map: &Vec<&[u8]>, slope_x: usize, slope_y: usize) -> i64 {

    let mut trees_hit = 0;

    let mut x: usize = 0;
    let mut y: usize = 0;

    let y_len = map[0].len();

    while x < map.len() {

        if map[x][y % y_len] == TREE {
            trees_hit += 1;
        }

        x += slope_x;
        y += slope_y;
    }

    return trees_hit;
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let map = input.split_whitespace().map(|line| line.as_bytes()).collect::<Vec<&[u8]>>();

    writeln!(io::stdout(), "Answer 1: {}", hit_trees(&map, 1, 3))?;
    writeln!(io::stdout(), "Answer 12 {}", hit_trees(&map, 1, 3) * hit_trees(&map, 1, 1) * hit_trees(&map, 1, 5) * hit_trees(&map, 1, 7) * hit_trees(&map, 2, 1))?;

    Ok(())
}
