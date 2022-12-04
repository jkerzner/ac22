use std::io::{BufReader, BufRead};
use std::fs::File;

const WIN: i32 = 6;
const TIE: i32 = 3;
const LOSS: i32 = 0;

#[derive(Debug, PartialEq, Eq)]
enum Object {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

fn letter_to_object(play: &str) -> Object{
    return match play {
        "A" | "X" => Object::Rock,
        "B" | "Y" => Object::Paper,
        "C" | "Z" => Object::Scissors,
        _ => panic!("Not a valid object"),
    };
}

fn throw_down(you: &Object, them: &Object) -> i32 {

    if *you == *them {
        return TIE;
    }
    else if *you == Object::Rock && *them == Object::Paper {
        return LOSS;
    }
    else if *you == Object::Rock && *them == Object::Scissors {
        return WIN;
    }
    else if *you == Object::Paper && *them == Object::Rock {
        return WIN;
    }
    else if *you == Object::Paper && *them == Object::Scissors {
        return LOSS;
    }
    else if *you == Object::Scissors && *them == Object::Rock {
        return LOSS;
    }
    else if *you == Object::Scissors && *them == Object::Paper {
        return WIN;
    }

    panic!("Invalid state");
}

pub fn day2() {
    let input = File::open("input/day2.txt");
    let input: File = match input {
        Ok(file) => file,
        Err(error) => {
            panic!("No file {:?}", error);
        }
    };

    let buffered: BufReader<File> = BufReader::new(input);

    let mut total_points: i32 = 0;
    for line in buffered.lines() {
        let l: String = line.unwrap();
        let r: Vec<&str> = l.split(' ').collect();

        let them: Object = letter_to_object(r[0]);
        let you: Object = letter_to_object(r[1]);

        let this_round: i32 = throw_down(&you, &them);
        let object_points: i32 = you as i32;
        total_points = total_points + (this_round + object_points);
    } 

    println!("Your total points: {}", total_points);
}
