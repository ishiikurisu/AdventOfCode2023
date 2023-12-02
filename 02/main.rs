use std::io::{self, BufRead};
use std::cmp::{max};

struct GameSet {
    red: i32,
    green: i32,
    blue: i32
}

struct Game {
    id: i32,
    sets: Vec<GameSet>
}

fn new_game(inlet: String) -> Game {
    let root_fields: Vec<&str> = inlet.split(": ").collect();
    let game_id_field_parts: Vec<&str> = root_fields[0].split(" ").collect();
    let game_id: i32 = game_id_field_parts[1].parse().unwrap();
    let game_sets_parts = root_fields[1].split("; ");
    let mut raw_game_sets: Vec<&str>;
    let mut game_set_parts: Vec<&str>;
    let mut color: String;
    let mut dice: i32;
    let mut game_sets: Vec<GameSet> = Vec::new();
    let mut game_set: GameSet;
    let game: Game;

    for game_sets_part in game_sets_parts {
        raw_game_sets = game_sets_part.split(", ").collect(); 
        game_set = GameSet {
            red: 0,
            green: 0,
            blue: 0
        };

        for raw_game_set in raw_game_sets {
            game_set_parts = raw_game_set.split(" ").collect();
            dice = game_set_parts[0].parse().unwrap();
            color = game_set_parts[1].to_string();

            if color == "red" {
                game_set.red = dice;
            } else if color == "green" {
                game_set.green = dice;
            } else {
                game_set.blue = dice;
            }
        }

        game_sets.push(game_set);
    }

    game = Game {
        id: game_id,
        sets: game_sets
    };

    return game;
}

fn power_set(game: &Game) -> i32 {
    let mut min_red: i32 = 0;
    let mut min_green: i32 = 0;
    let mut min_blue: i32 = 0;

    for set in &game.sets {
        min_red = max(min_red, set.red);
        min_green = max(min_green, set.green);
        min_blue = max(min_blue, set.blue);
    }

    return min_red * min_green * min_blue;
}

fn main() {
    let stdin = io::stdin();
    let mut inlet: String;
    let mut result: i32 = 0;
    let mut game: Game;

	for line in stdin.lock().lines() {
        inlet = line.unwrap();
        if inlet.len() > 0 {
            game = new_game(inlet.clone());
            result += power_set(&game);
        }
	}

    println!("{}", result);
}
