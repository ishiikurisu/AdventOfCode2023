use std::io::{self, BufRead};

// static EMPTY_SPACE: i32 = 1000000;
static EMPTY_SPACE: i32 = 10;

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
                line.push('+');
            }
        }
        image.push(line);
    }

    return image;
}

fn calculate_distance(image: &Vec<String>, a: &Vec<i32>, b:&Vec<i32>) -> i32 {
    let ai: i32 = a[0];
    let aj: i32 = a[1];
    let bi: i32 = b[0];
    let bj: i32 = b[1];
    let mut outlet: i32 = 0;
    let mut i: i32;
    let mut s: i32;
    let mut c: char;

    i = ai;
    s = if bi > ai {
        1
    } else {
        -1
    };
    while i != bi {
        c = image[aj as usize].chars().nth(i as usize).unwrap();
        outlet += if c == '+' {
            EMPTY_SPACE
        } else {
            1
        };
        i += s;
    }

    i = aj;
    s = if bj > aj {
        1
    } else {
        -1
    };
    while i != bj {
        c = image[i as usize].chars().nth(bi as usize).unwrap();
        outlet += if c == '+' {
            EMPTY_SPACE
        } else {
            1
        };
        i += s;
    }

    return outlet;
}

fn main() {
    let stdin = io::stdin();
    let mut original_image: Vec<String> = Vec::new();
    let image: Vec<String>;
    let mut galaxies: Vec<Vec<i32>> = Vec::new();
    let mut galaxy: Vec<i32>;
    let mut galaxy_a: &Vec<i32>;
    let mut galaxy_b: &Vec<i32>;
    let mut count: i32;
    let mut inlet: String;
    let mut outlet: i32 = 0;

	for line in stdin.lock().lines() {
        inlet = line.unwrap();
        if is_empty(inlet.clone()) {
            count = inlet.len() as i32;
            inlet = "".to_string();
            for _ in 0..count {
                inlet.push('+');
            }
        }
        original_image.push(inlet.clone());
	}

    image = expand_image_vertically(&original_image);
    for line in image.clone().into_iter() {
        println!("{}", line);
    }

    count = 0;
    for (j, line) in image.clone().into_iter().enumerate() {
        for (i, obj) in line.chars().enumerate() {
            if obj == '#' {
                galaxy = Vec::new();
                galaxy.push(i as i32);
                galaxy.push(j as i32);
                galaxies.push(galaxy);
                count += 1;
            }
        }
    }

    for i in 0..(count - 1) {
        galaxy_a = &galaxies[i as usize];
        for j in (i + 1)..count {
            galaxy_b = &galaxies[j as usize];
            outlet += calculate_distance(&image, galaxy_a, galaxy_b);
        }
    }

    println!("{}", outlet);
}

