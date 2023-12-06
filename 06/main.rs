use std::io::{self, BufRead};

fn parse_line(inlet: String) -> Vec<i32> {
    let mut outlet: Vec<i32> = Vec::new();
    let fields: Vec<&str> = inlet.split(" ").collect();
    let mut is_header_field: bool = true;

    for field in fields {
        if is_header_field == true {
            is_header_field = false;
        } else if field.len() > 0 {
            outlet.push(field.parse().unwrap());
        }
    }

    return outlet;
}

fn evaluate(time: i32, distance: i32) -> i32 {
    let mut result: i32 = 0;

    for i in 0..time {
        if i * (time - i) > distance {
            result += 1;
        }
    }

    return result;
}

fn main() {
    let stdin = io::stdin();
    let mut time_line: bool = true;
    let mut times: Vec<i32> = Vec::new();
    let mut distances: Vec<i32> = Vec::new();
    let mut line: String;
    let mut outlet: i32 = 1;

	for inlet in stdin.lock().lines() {
        line = inlet.unwrap();
        if line.len() > 0 {
            if time_line == true {
                times = parse_line(line.clone());
                time_line = false;
            } else {
                distances = parse_line(line.clone());
            }
        }
	}

    for (i, time) in times.into_iter().enumerate() {
        outlet *= evaluate(time, distances[i]);    
    }

    println!("{}", outlet);
}

