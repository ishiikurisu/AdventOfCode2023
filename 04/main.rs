use std::io::{self, BufRead};
use std::cmp::{min};

const WINNING_SET_SIZE: usize = 100;

fn new_empty_winning_set(size: usize) -> Vec<bool> {
    let mut winning_set: Vec<bool> = Vec::new();

    for _ in 0 .. size {
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
                outlet += 1;
            }
        }
    }

    return outlet;
}

fn main() {
    let stdin = io::stdin();
    let mut inlet: String;
    let mut score: i32;
    let mut no_copies: i32;
    let mut outlet: i32 = 0;
    let mut points: Vec<i32> = Vec::new();
    let mut copies: Vec<i32> = Vec::new();
    let mut total_cards: i32 = 0;
    let mut from_index: i32;
    let mut to_index: i32;

	for line in stdin.lock().lines() {
        inlet = line.unwrap();
        if inlet.len() > 0 {
            score = evaluate_line(inlet.clone());
            points.push(score);
            copies.push(1);
            total_cards += 1;
        }
	}

    for i in 0 .. total_cards {
        score = points[i as usize];
        no_copies = copies[i as usize];
        from_index = i + 1;
        to_index = min(total_cards, i + 1 + score);
        for j in from_index .. to_index {
            copies[j as usize] += no_copies;
        }
    }

    for i in 0 .. total_cards {
        outlet += copies[i as usize];
    }

    println!("{}", outlet);
}
