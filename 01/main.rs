use std::io::{self, BufRead};
use std::collections::HashMap;

struct Constants {
    replacements: HashMap<String, String>,
    typed_numbers: Vec<String>,
    reversed_replacements: HashMap<String, String>,
    reversed_typed_numbers: Vec<String>
}

fn reverse_string(inlet: String) -> String {
    return inlet.chars().rev().collect::<String>();
}

fn find_first_digit(
    inlet: String,
    replacements: &HashMap<String, String>,
    typed_numbers: &Vec<String>
) -> char {
    // replacing first typed number that can be found
    let mut number_to_replace: String = "".to_string();
    let mut lowest_index: i32 = 1000;
    let mut found_index: Option<usize>;
    let mut midlet = inlet.clone();

    for typed_number in typed_numbers {
        found_index = inlet.find(typed_number);
        if let Some(index) = found_index {
            if (index as i32) < lowest_index {
                lowest_index = index as i32;
                number_to_replace = typed_number.to_string();
            }
        }
    }

    if number_to_replace.len() > 0 {
        let replacement = replacements.get(&number_to_replace).unwrap();
        midlet = inlet.replace(&number_to_replace, replacement);
    }

    // finding first digit
    for c in midlet.chars() {
        if c >= '0' && c <= '9' {
            return c;
        }
    }
    
    return '0';
}

fn get_first_digit(inlet: String, constants: &Constants) -> char {
    return find_first_digit(
        inlet.clone(),
        &constants.replacements,
        &constants.typed_numbers
    );
}

fn get_last_digit(inlet: String, constants: &Constants) -> char {
    return find_first_digit(
        reverse_string(inlet.clone()),
        &constants.reversed_replacements,
        &constants.reversed_typed_numbers
    );
}

fn new_replacements_hashmap() -> HashMap<String, String> {
    let mut replacements = HashMap::new();

    replacements.insert("one".to_string(), "1".to_string());
    replacements.insert("two".to_string(), "2".to_string());
    replacements.insert("three".to_string(), "3".to_string());
    replacements.insert("four".to_string(), "4".to_string());
    replacements.insert("five".to_string(), "5".to_string());
    replacements.insert("six".to_string(), "6".to_string());
    replacements.insert("seven".to_string(), "7".to_string());
    replacements.insert("eight".to_string(), "8".to_string());
    replacements.insert("nine".to_string(), "9".to_string());

    return replacements;
}

fn handle_line(inlet: String, constants: &Constants) -> i32 {
    let first_digit: char = get_first_digit(inlet.clone(), constants);
    let last_digit: char = get_last_digit(inlet.clone(), constants);
    let midlet: String = vec![first_digit, last_digit].into_iter().collect();
    let outlet = midlet.parse().unwrap();

    return outlet;
}

fn reverse_replacements(
    replacements: &HashMap<String, String>,
    typed_numbers: &Vec<String>
) -> HashMap<String, String> {
    let mut reversed_replacements = HashMap::new();
    let mut reversed_typed_number: String;
    let mut digit: String;

    for typed_number in typed_numbers {
        reversed_typed_number = reverse_string(typed_number.clone());
        digit = replacements.get(typed_number).unwrap().to_string(); 
        reversed_replacements.insert(reversed_typed_number, digit);
    }

    return reversed_replacements;
}

fn main() {
    let stdin = io::stdin();
    let mut inlet: String;
    let mut midlet: i32;
    let mut outlet: i32 = 0;
    let replacements = new_replacements_hashmap();
    let typed_numbers: Vec<String> = vec![
        "one".to_string(),
        "two".to_string(),
        "three".to_string(),
        "four".to_string(),
        "five".to_string(),
        "six".to_string(),
        "seven".to_string(),
        "eight".to_string(),
        "nine".to_string(),
    ].into_iter().collect();
    let reversed_replacements = reverse_replacements(
        &replacements,
        &typed_numbers
    );
    let reversed_typed_numbers: Vec<String> = typed_numbers.clone()
        .into_iter()
        .map(|s| reverse_string(s))
        .collect();
    let constants = Constants {
        replacements,
        typed_numbers,
        reversed_replacements,
        reversed_typed_numbers
    };

    for line in stdin.lock().lines() {
        inlet = line.unwrap();
        if inlet.len() > 0 {
            midlet = handle_line(inlet.clone(), &constants);
            outlet += midlet;
        }
    }

    println!("{}", outlet);
}

