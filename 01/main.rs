use std::io::{self, BufRead};

fn get_first_digit(inlet: String) -> char {
    for c in inlet.chars() {
        if c >= '0' && c <= '9' {
            return c;
        }
    }

    return '0';
}

fn get_last_digit(inlet: String) -> char {
    return get_first_digit(inlet.chars().rev().collect::<String>());
}

fn handle_line(inlet: String) -> i32 {
    let first_digit: char = get_first_digit(inlet.clone());
    let last_digit: char = get_last_digit(inlet.clone());
    let midlet: String = vec![first_digit, last_digit].into_iter().collect();
    let outlet: i32 = midlet.parse().unwrap();

    return outlet;
}

fn main() {
    let stdin = io::stdin();
    let mut inlet: String;
    let mut midlet: i32;
    let mut outlet: i32 = 0;

    for line in stdin.lock().lines() {
        inlet = line.unwrap();
        if inlet.len() > 0 {
            midlet = handle_line(inlet.clone());
            outlet += midlet;
        }
    }

    println!("{}", outlet);
}

