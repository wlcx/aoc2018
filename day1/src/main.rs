use std::io::*;
use std::fs::File;
use std::str::FromStr;

fn main() {
    let f = File::open("input.txt").unwrap();
    let mut accumulator: i32 = 0;
    for line in BufReader::new(f).lines() {
        let l = line.unwrap();
        if l != "" {
            accumulator += i32::from_str(&l).unwrap();
        }
    }
    println!("{}", accumulator);
}
