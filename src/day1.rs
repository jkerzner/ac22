use std::io::{BufReader, BufRead};
use std::fs::File;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Elf {
    index: usize,
    calories: i64
}

impl Elf {
    pub fn new(index: usize, calories: i64) -> Self {
        Elf {
            index,
            calories
            }
    }
}

pub fn day1() {
    let path: &str = "input/day1.txt";

    let input = File::open(path);
    let input: File = match input {
        Ok(file) => file,
        Err(error) => {
            panic!("No file {:?}", error);
        }
    };
    let buffered: BufReader<File> = BufReader::new(input);

    let mut calories_by_elf: Vec<i64> = vec![];

    // Avoid relying on indexes to know which elf is which
    let mut elves: Vec<Elf> = vec![];
    let mut elf_num: usize = 0;
    let mut highwater_mark: i64 = 0;
    let mut highwater_index: usize = 0;
    calories_by_elf.push(0);

    for line in buffered.lines() {
        let val: String = line.unwrap();

        if val.is_empty() {

            if calories_by_elf[elf_num] > highwater_mark {
                highwater_mark = calories_by_elf[elf_num];
                highwater_index = elf_num;
            }

            elves.push(Elf::new(elf_num, calories_by_elf[elf_num]));

            // Setup the next elf
            elf_num = elf_num + 1;
            calories_by_elf.push(0);
        }
        else {
            calories_by_elf[elf_num] = calories_by_elf[elf_num] + val.parse::<i64>().unwrap();
        }
    }

    // Day 1 Part 1
    println!("Highwater mark is {} at {}", highwater_mark, highwater_index);

    // Sort for Day 1 Part 2
    elves.sort_by(|a, b| b.calories.cmp(&a.calories));
    calories_by_elf.sort();
    calories_by_elf.reverse();

    // Day 1 Part 2
    println!("top 3 carry {} calories total", calories_by_elf.drain(..3).sum::<i64>());
    println!("Calories of 3 most loaded elves is {:?}", elves.drain(..3).map(|e| e.calories).sum::<i64>());

}
