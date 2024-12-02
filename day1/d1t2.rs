use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

fn main() -> io::Result<()> {
    let mut f = File::open("input.dat")?;
    let mut buffer = String::new();

    f.read_to_string(&mut buffer)?;

    let mut left_col = Vec::new();
    let mut right_col = HashMap::new();

    for s in buffer.lines() {
        let mut split = s.split("   ");
        let left:i32 = split.next().unwrap().parse().unwrap();
        let right:i32 = split.next().unwrap().parse().unwrap();

        left_col.push(left);

        let entries = right_col.entry(right).or_insert(0);
        *entries += 1;
    }

    let mut res = 0;

    for i in 0..left_col.len() {
        let left = left_col[i];

        if let Some(entries) = right_col.get(&left) {
            res += left * *entries;
        }
    }

    println!("Result: {}", res);
    return Ok(());
}
