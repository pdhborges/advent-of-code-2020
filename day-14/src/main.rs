use std::error::Error;
use std::io::{self, Read, Write};
use std::result;
use std::collections::HashMap;

type Result<T> = result::Result<T, Box<dyn Error>>;


struct Machine {
    mask: String,
    memory: HashMap<i64, i64>
}

impl Machine {
    fn new() -> Machine {
        Machine {
            mask : "".to_owned(),
            memory : HashMap::new()
        }
    }

    fn set_memory(&mut self, mem_addr: i64, val: i64) {
        let fval = format!("{:0>36}", format!("{:b}", val));

        let mut masked = String::new();
        for (a, b) in self.mask.chars().zip(fval.chars()) {
            if a == 'X' {
                masked.push(b);
            } else {
                masked.push(a);
            }
        }
        self.memory.insert(mem_addr, i64::from_str_radix(masked.as_str(), 2).unwrap());
    }

    fn set_memory_amb(&mut self, addr: &mut String, idx: usize, mask: &[u8], val: i64) {

        if idx == 36 {
            self.memory.insert(i64::from_str_radix(addr.as_str(), 2).unwrap(), val);
            return;
        }

        if mask[idx] == ('X' as u8) {
            addr.push('1');
            self.set_memory_amb(addr, idx + 1, mask, val);
            addr.pop();
            addr.push('0');
            self.set_memory_amb(addr, idx + 1, mask, val);
            addr.pop();
        } else {
            addr.push(mask[idx] as char);
            self.set_memory_amb(addr, idx + 1, mask, val);
            addr.pop();
        }

    }

    fn set_memory_2(&mut self, mem_addr: i64, val: i64) {

        let fmem_addr = format!("{:0>36}", format!("{:b}", mem_addr));

        let mut masked = String::new();
        for (a, b) in fmem_addr.chars().zip(self.mask.chars()) {
            if b == 'X' {
                masked.push(b);
            } else if b == '1' {
                masked.push('1');
            } else {
                masked.push(a)
            }
        }

        let mut addr = String::new();
        self.set_memory_amb(&mut addr, 0, masked.as_bytes(), val);
    }

    fn set_mask(&mut self, mask: &str) {
        self.mask = mask.to_owned()
    }

    fn memory_sum(&self) -> i64 {
         self.memory.values().sum()
    }
}

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut machine = Machine::new();

    for line in input.lines() {
        if line.starts_with("mask") {
            machine.set_mask(&line[7..line.len()]);
        } else {
            let processed_line = line.replace("mem[", "").replace("]", "");
            let mut parts = processed_line.split(" = ");
            machine.set_memory(parts.next().unwrap().parse().unwrap(), parts.next().unwrap().parse().unwrap());
        }
    }

    writeln!(io::stdout(), "Answer 1: {}", machine.memory_sum())?;

    let mut machine = Machine::new();

    for line in input.lines() {
        if line.starts_with("mask") {
            machine.set_mask(&line[7..line.len()]);
        } else {
            let processed_line = line.replace("mem[", "").replace("]", "");
            let mut parts = processed_line.split(" = ");
            machine.set_memory_2(parts.next().unwrap().parse().unwrap(), parts.next().unwrap().parse().unwrap());
        }
    }

    writeln!(io::stdout(), "Answer 2: {}", machine.memory_sum())?;

    Ok(())
}
