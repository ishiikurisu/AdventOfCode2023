use std::io::{self, BufRead};

fn parse_line(inlet: String) -> i64 {
    let mut outlet: String = String::new();
    let fields: Vec<&str> = inlet.split(" ").collect();
    let mut is_header_field: bool = true;

    for field in fields {
        if is_header_field == true {
            is_header_field = false;
        } else if field.len() > 0 {
            outlet.push_str(field);
        }
    }

    return outlet.parse().unwrap();
}

fn evaluate(time: i64, distance: i64) -> i64 {
    let mut result: i64 = 0;

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
    let mut time: i64 = 0;
    let mut distance: i64 = 0;
    let mut line: String;
    let outlet: i64;

	for inlet in stdin.lock().lines() {
        line = inlet.unwrap();
        if line.len() > 0 {
            if time_line == true {
                time = parse_line(line.clone());
                time_line = false;
            } else {
                distance = parse_line(line.clone());
            }
        }
	}

    outlet = evaluate(time, distance);

    println!("{}", outlet);
}

