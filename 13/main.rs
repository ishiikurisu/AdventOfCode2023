use std::io::{self, BufRead};
use std::cmp::{min};

fn inverse(x: u32) -> u32 {
    return if x == 0x0 {
        0x1
    } else {
        0x0
    };
}

fn find_mirrors(array: &Vec<u32>) -> u32 {
    let size: usize = array.len();
    let mut length: usize;
    let mut sub1: Vec<u32>;
    let mut sub2: Vec<u32>;
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
            return i as u32;
        }
    }

    return 0;
}

fn evaluate_matrix(matrix: &Vec<Vec<u32>>, width: usize, height: usize) -> Vec<u32> {
    let mut array: Vec<u32>;
    let mut number: u32;
    let vertical: u32;
    let horizontal: u32;

    array = Vec::new();
    for i in 0..width {
        number = 0;
        for j in 0..height {
            number = (number << 0x1) | matrix[j][i];
        }
        array.push(number);
    }
    vertical = find_mirrors(&array);
     
    array = Vec::new();
    for j in 0..height {
        number = 0;
        for i in 0..width {
            number = (number << 0x1) | matrix[j][i];
        }
        array.push(number);
    }
    horizontal = find_mirrors(&array);

    return vec![vertical, horizontal];
}

fn evaluate(inlet: &Vec<String>) -> u32 {
    let mut matrix: Vec<Vec<u32>> = Vec::new();
    let horizontal: u32;
    let vertical: u32;
    let mut evaluation: Vec<u32>;
    let mut maybe_horizontal: u32;
    let mut maybe_vertical: u32;
    let mut array: Vec<u32>;
    let width: usize = inlet[0].len();
    let height: usize = inlet.len();
    let mut outlet: u32;

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

    // base evaluation
    evaluation = evaluate_matrix(&matrix, width, height);
    vertical = evaluation[0];
    horizontal = evaluation[1];

    // evaluations from possible fixes
    for j in 0..height {
        for i in 0..width {
            matrix[j][i] = inverse(matrix[j][i]);
            evaluation = evaluate_matrix(&matrix, width, height);
            maybe_vertical = evaluation[0];
            maybe_horizontal = evaluation[1];

            outlet = 0;
            if maybe_vertical != 0 && maybe_vertical != vertical {
                outlet += maybe_vertical;
            }
            if maybe_horizontal != 0 && maybe_horizontal != horizontal {
                outlet += 100 * maybe_horizontal;
            }
            if outlet > 0 {
                return outlet;
            }

            matrix[j][i] = inverse(matrix[j][i]);
        }
    }

    return vertical + 100 * horizontal;
}

fn main() {
    let stdin = io::stdin();
    let mut line: String;
    let mut inlet: Vec<String> = Vec::new();
    let mut outlet: u32 = 0;

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
