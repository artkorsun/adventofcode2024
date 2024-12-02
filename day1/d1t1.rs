use std::io;
use std::io::prelude::*;
use std::fs::File;

fn main() -> io::Result<()> {
    let mut f = File::open("input.dat")?;
    let mut buffer = String::new();

    f.read_to_string(&mut buffer)?;

    let mut left_col = Vec::new();
    let mut right_col = Vec::new();

    for s in buffer.lines() {
        let mut split = s.split("   ");
        let left:i32 = split.next().unwrap().parse().unwrap();
        let right:i32 = split.next().unwrap().parse().unwrap();

        left_col.push(left);
        right_col.push(right);
    }

    left_col.sort();
    right_col.sort();

    let mut res = 0;

    for i in 0..left_col.len() {
        res += (left_col[i] - right_col[i]).abs();
    }

    println!("Result: {}", res);
    return Ok(());
}
