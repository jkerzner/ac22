use std::collections::HashSet;
use std::io::{BufReader,BufRead};
use std::fs::File;

const LC_OFFSET: u32 = 96;
const UC_OFFSET: u32 = 38;

fn get_buffered_reader(path: &str) -> BufReader<File> {
    let input = File::open(path);
    let input: File = match input {
        Ok(file) => file,
        Err(error) => {
            panic!("No file {:?}", error);
        }
    };

    return BufReader::new(input);
}

pub fn day3 () {
    day3part1();
    day3part2();
}

fn day3part2() {
    println!("placeholder");
}

fn day3part1() {

    let buffered: BufReader<File> = get_buffered_reader("input/day3.txt");

    let mut total_value: u32 = 0;
    for line in buffered.lines() {
        let ln: String = line.unwrap();
        let (top, bottom) = ln.split_at(ln.len() / 2);

        let top_hash: HashSet<char> = HashSet::from_iter(top.chars());
        let bottom_hash: HashSet<char> = HashSet::from_iter(bottom.chars());

        for c in top_hash.intersection(&bottom_hash) {
            let c_ascii_val: u32 = *c as u32;
            total_value += match c_ascii_val {
                65..=90 => c_ascii_val - UC_OFFSET,
                _ => c_ascii_val - LC_OFFSET
            };
        }
    }
    println!("total value is {}", total_value);
}