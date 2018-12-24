use std::fs::File;
use std::io::{BufReader,BufRead};
use std::collections::HashMap;

fn main() {
    let mut twos = 0;
    let mut threes = 0;
    let f = File::open("input").unwrap();
    for l in BufReader::new(f).lines() {
        let (mut hastwo, mut hasthree) = (0, 0);
        let mut charmap = HashMap::new();
        for c in l.unwrap().chars().filter(|c| c.is_alphabetic()) {
            let count = charmap.entry(c).or_insert(0);
            *count += 1;
        }
        for (ch, count) in charmap {
            match count {
                2 => hastwo = 1,
                3 => hasthree = 1,
                _ => (),
            }
        }
        twos += hastwo;
        threes += hasthree;
    }
    println!("{}", twos*threes);
}
