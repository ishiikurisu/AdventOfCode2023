use std::io::{self, BufRead};

const WINNING_SET_SIZE: usize = 100;

fn new_empty_winning_set(size: usize) -> Vec<bool> {
    let mut winning_set: Vec<bool> = Vec::new();

    for _ in 0..size {
        winning_set.push(false);
    }

    return winning_set;
}

fn evaluate_line(inlet: String) -> i32 {
    let root_fields: Vec<&str> = inlet.split(": ").collect();
    let number_sets_fields: Vec<&str> = root_fields[1].split(" | ").collect();
    let raw_winning_set: Vec<&str> = number_sets_fields[0].split(" ").collect();
    let raw_playing_set: Vec<&str> = number_sets_fields[1].split(" ").collect();
    let mut winning_set: Vec<bool> = new_empty_winning_set(WINNING_SET_SIZE);
    let mut number: i32;
    let mut outlet: i32 = 0;

    for raw_winning_number in raw_winning_set {
        if raw_winning_number.len() > 0 {
            number = raw_winning_number.parse::<i32>().unwrap() - 1;
            winning_set[number as usize] = true;
        }
    }

    for raw_playing_number in raw_playing_set {
        if raw_playing_number.len() > 0 {
            number = raw_playing_number.parse::<i32>().unwrap() - 1;
            if winning_set[number as usize] == true {
                outlet = if outlet == 0 {
                    0x1
                } else {
                    outlet << 0x1
                }
            }
        }
    }

    return outlet;
}

fn main() {
    let stdin = io::stdin();
    let mut inlet: String;
    let mut outlet: i32 = 0;

	for line in stdin.lock().lines() {
        inlet = line.unwrap();
        if inlet.len() > 0 {
            outlet += evaluate_line(inlet.clone());
        }
	}

    println!("{}", outlet);
}
