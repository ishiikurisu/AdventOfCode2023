use std::io::{self, BufRead};

fn read_arrangement(inlet: String) -> Vec<i32> {
    let mut outlet: Vec<i32> = Vec::new();
    let fields: Vec<&str> = inlet.split(",").collect();

    for field in fields {
        outlet.push(field.parse().unwrap());
    }

    return outlet;
}

fn evaluate(inlet: String) -> i32 {
    let fields: Vec<&str> = inlet.split(" ").collect();
    let raw_springs: String = fields[0].to_string();
    let target_arrangement: Vec<i32> = read_arrangement(fields[1].to_string());
    let mut possible_arrangements: i32 = 0;

    // count possible arrangements
    for c in raw_springs.chars() {
        if c == '?' {
            possible_arrangements += 1;
        }
    }
    
    // TODO generate all possible arrangements, counting the valid ones

    return 0;
}

fn main() {
    let stdin = io::stdin();
    let mut inlet: String;
    let mut outlet: i32 = 0;

	for line in stdin.lock().lines() {
        inlet = line.unwrap();
        if inlet.len() > 0 {
            outlet += evaluate(inlet.clone());
        }
	}

    println!("{}", outlet);
}

