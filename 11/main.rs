use std::io::{self, BufRead};

fn is_empty(line: String) -> bool {
    for c in line.chars() {
        if c == '#' {
            return false;
        }
    }
    return true;
}

fn expand_image_vertically(original_image: &Vec<String>) -> Vec<String> {
    let mut image: Vec<String> = Vec::new();
    let mut line: String;
    let mut spaces: Vec<usize> = Vec::new();
    let original_width: usize = original_image[0].len();
    let original_height: usize = original_image.len();
    let mut c: char;

    for i in 0..original_width {
        line = "".to_string();
        for j in 0..original_height {
            c = original_image[j].chars().nth(i).unwrap();
            line.push(c);
        }
        if is_empty(line) {
            spaces.push(i);
        }
    }

    for original_line in original_image {
        line = "".to_string();
        for (i, obj) in original_line.chars().enumerate() {
            line.push(obj);
            if spaces.contains(&i) {
                line.push('.');
            }
        }
        image.push(line);
    }

    return image;
}

fn manhattan_distance(a: &Vec<i32>, b: &Vec<i32>) -> i32 {
    return (a[0] - b[0]).abs() + (a[1] - b[1]).abs();
}

fn main() {
    let stdin = io::stdin();
    let mut original_image: Vec<String> = Vec::new();
    let image: Vec<String>;
    let mut galaxies: Vec<Vec<i32>> = Vec::new();
    let mut galaxy: Vec<i32>;
    let mut galaxy_a: &Vec<i32>;
    let mut galaxy_b: &Vec<i32>;
    let mut galaxy_count: i32 = 0;
    let mut inlet: String;
    let mut outlet: i32 = 0;

	for line in stdin.lock().lines() {
        inlet = line.unwrap();
        original_image.push(inlet.clone());
        if is_empty(inlet.clone()) {
            original_image.push(inlet.clone());
        }
	}

    image = expand_image_vertically(&original_image);

    for (i, line) in image.clone().into_iter().enumerate() {
        for (j, obj) in line.chars().enumerate() {
            if obj == '#' {
                galaxy = Vec::new();
                galaxy.push(i as i32);
                galaxy.push(j as i32);
                galaxies.push(galaxy);
                galaxy_count += 1;
            }
        }
    }

    for i in 0..galaxy_count {
        galaxy_a = &galaxies[i as usize];
        for j in (i + 1)..galaxy_count {
            galaxy_b = &galaxies[j as usize];
            outlet += manhattan_distance(galaxy_a, galaxy_b);
        }
    }

    println!("{}", outlet);
}
