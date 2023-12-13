use std::io::{self, BufRead};
use std::cmp::{min};

fn find_mirrors(array: &Vec<i32>) ->i32 {
    let size: usize = array.len();
    let mut length: usize;
    let mut sub1: Vec<i32>;
    let mut sub2: Vec<i32>;
    let mut equal: bool;

    for i in 1..size {
        sub1 = array[0..i].to_vec();
        sub1.reverse();
        sub2 = array[i..].to_vec();
        length = min(sub1.len(), sub2.len());
        equal = true;

        for j in 0..length {
            if sub1[j] != sub2[j] {
                equal = false;
            }
        }

        if equal {
            return i as i32;
        }
    }

    return 0;
}

fn evaluate(inlet: &Vec<String>) -> i32 {
    let mut outlet: i32 = 0;
    let mut matrix: Vec<Vec<i32>> = Vec::new();
    let mut array: Vec<i32>;
    let mut number: i32;
    let width: usize = inlet[0].len();
    let height: usize = inlet.len();

    // creating matrix
    for line in inlet {
        array = Vec::new();
        for c in line.chars() {
            if c == '.' {
                array.push(0x0);
            } else {
                array.push(0x1);
            }
        }
        matrix.push(array);
    }

    // vertical evaluation
    array = Vec::new();
    for i in 0..width {
        number = 0;
        for j in 0..height {
            number = (number << 0x1) | matrix[j][i];
        }
        array.push(number);
    }
    number = find_mirrors(&array);
    outlet += number;
     
    // horizontal evaluation
    array = Vec::new();
    for j in 0..height {
        number = 0;
        for i in 0..width {
            number = (number << 0x1) | matrix[j][i];
        }
        array.push(number);
    }
    number = find_mirrors(&array);
    outlet += 100 * number;
 
    return outlet;
}

fn main() {
    let stdin = io::stdin();
    let mut line: String;
    let mut inlet: Vec<String> = Vec::new();
    let mut outlet: i32 = 0;

	for raw_line in stdin.lock().lines() {
        line = raw_line.unwrap();
        if line.len() > 0 {
            inlet.push(line);
        } else {
            outlet += evaluate(&inlet);
            inlet = Vec::new();
        }
	}

    println!("{}", outlet);
}
