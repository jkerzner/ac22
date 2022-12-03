use std::io::{BufReader, BufRead};
use std::fs::File;

fn main() {
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

            // Setup the next elf
            elf_num = elf_num + 1;
            calories_by_elf.push(0);
        }
        else {
            calories_by_elf[elf_num] = calories_by_elf[elf_num] + val.parse::<i64>().unwrap();
        }
    }

    println!("{:?}", calories_by_elf);
    println!("Highwater mark is {} at {}", highwater_mark, highwater_index);
}
