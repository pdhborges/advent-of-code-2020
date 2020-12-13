use std::error::Error;
use std::io::{self, Read, Write};
use std::result;
use std::ops::{Index, IndexMut};
use std::cmp::{Eq, PartialEq};
use std::mem::swap;

type Result<T> = result::Result<T, Box<dyn Error>>;

struct Board {
    vec: Vec<char>,
    height: usize,
    width: usize
}

impl Board {

    const FLOOR: char = '.';
    const CHAIR: char = 'L';
    const OCCUPIED: char = '#';

    fn new(height: usize, width: usize) -> Board {
        Board {
            vec: vec![Board::FLOOR; height * width],
            height,
            width
        }
    }

    fn around2(&self, x: usize, y: usize) -> Vec<char> {

        let sx = x as i64;
        let sy = y as i64;

        let mut around = vec![];

        let directions = vec![
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1)];


        for (dx, dy) in directions {
            let mut tx = sx + dx;
            let mut ty = sy + dy;

            while tx >= 0 && tx < self.height as i64 && ty >= 0 && ty < self.width as i64 {
                if self[tx as usize][ty as usize] != Board::FLOOR {
                    around.push(self[tx as usize][ty as usize]);
                    break;
                }
                tx += dx;
                ty += dy;
            }
        }

        return around;
    }


    fn next_val2(&self, x: usize, y: usize) -> char {

        let around = self.around2(x, y);

        if self[x][y] == Board::CHAIR && !around.iter().any(|&pos| pos == Board::OCCUPIED) {
            return Board::OCCUPIED;
        }

        if self[x][y] == Board::OCCUPIED && around.iter().filter(|&&pos| pos == Board::OCCUPIED).count() >= 5 {
            return Board::CHAIR;
        }

        return self[x][y];
    }

    fn around(&self, x: usize, y: usize) -> Vec<char> {

        let sx = x as i64;
        let sy = y as i64;

        let positions = vec![
            (sx - 1, sy - 1),
            (sx - 1, sy),
            (sx - 1, sy + 1),
            (sx, sy - 1),
            (sx, sy + 1),
            (sx + 1, sy - 1),
            (sx + 1, sy),
            (sx + 1, sy + 1)];

        let mut around = vec![];

        for (px, py) in positions {
            if px >= 0 && px < self.height as i64 && py >= 0 && py < self.width as i64 {
                around.push(self[px as usize][py as usize])
            }
        }

        return around;
    }

    fn next_val(&self, x: usize, y: usize) -> char {

        let around = self.around(x, y);

        if self[x][y] == Board::CHAIR && !around.iter().any(|&pos| pos == Board::OCCUPIED) {
            return Board::OCCUPIED;
        }

        if self[x][y] == Board::OCCUPIED && around.iter().filter(|&&pos| pos == Board::OCCUPIED).count() >= 4 {
            return Board::CHAIR;
        }

        return self[x][y];
    }

    fn count(&self, element: char) -> usize {
        return self.vec.iter().filter(|e| **e == element).count();
    }
}

impl Index<usize> for Board {
    type Output = [char];
    fn index<'a>(&'a self, index: usize) -> &'a [char] {
        let i = index * self.width;
        &self.vec[i..i + self.width]
    }
}

impl IndexMut<usize> for Board {
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut [char] {
        let i = index * self.width;
        &mut self.vec[i..i + self.width]
    }
}

impl Clone for Board {
    fn clone(&self) -> Board {
        return Board {
            vec: self.vec.clone(),
            height: self.height,
            width: self.width
        }
    }
}

impl PartialEq for Board {
    fn eq(&self, other: &Self) -> bool {
        return self.width == other.width && self.height == other.height && self.vec == other.vec;
    }
}

impl Eq for Board {}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let height = input.lines().count();
    let width = input.lines().next().unwrap().chars().count();

    let mut board = Board::new(height, width);

    for (x, line) in input.lines().enumerate() {
        for (y, c) in line.chars().enumerate() {
            board[x][y] = c;
        }
    }

    let mut board_v1 = board.clone();
    let mut board_next = Board::new(height, width);

    while board_v1 != board_next {
        for x in 0..board_v1.height {
            for y in 0..board_v1.width {
                board_next[x][y] = board_v1.next_val(x, y);
            }
        }
        swap(&mut board_v1, &mut board_next);
    }

    writeln!(io::stdout(), "Answer 1: {}", board_v1.count(Board::OCCUPIED))?;


    let mut board_v2 = board.clone();
    let mut board_next = Board::new(height, width);

    while board_v2 != board_next {
        for x in 0..board_v2.height {
            for y in 0..board_v2.width {
                board_next[x][y] = board_v2.next_val2(x, y);
            }
        }
        swap(&mut board_v2, &mut board_next);
    }

    writeln!(io::stdout(), "Answer 2: {}", board_v2.count(Board::OCCUPIED))?;


    Ok(())
}
