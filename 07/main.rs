use std::io::{self, BufRead};
use std::collections::HashMap;
use std::cmp::Ordering;

struct Constants {
    card_to_index: HashMap<char, usize>
}

struct Game {
    cards: Vec<i32>,
    bid: i32,
    score: i32,
}

fn compare_games(a: &Game, b: &Game) -> Ordering {
    let mut ia: usize = 0;
    let mut ha: i32 = 0;
    let mut ib: usize = 0;
    let mut hb: i32 = 0;

    if a.score > b.score {
        return Ordering::Greater;
    } else if a.score < b.score {
        return Ordering::Less;
    }

    for (i, h) in a.cards.clone().into_iter().enumerate() {
        if h >= ha {
            ha = h;
            ia = i;
        }
        if b.cards[i] >= hb {
            hb = b.cards[i];
            ib = i;
        }
    }

    return if ia > ib {
        Ordering::Greater
    } else {
        Ordering::Less
    };
}

fn parse_game(inlet: String, constants: &Constants) -> Game {
    let game: Game;
    let fields: Vec<&str> = inlet.split(" ").collect();
    let raw_cards: &str = fields[0];
    let bid: i32 = fields[1].parse().unwrap();
    let mut cards: Vec<i32> = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0].into_iter().collect();
    let score: i32;
    let mut scores: Vec<i32>;

    for card in raw_cards.chars() {
        cards[constants.card_to_index[&card]] += 1;
    }

    // cache score and highest cards
    scores = cards.clone().into_iter().collect();
    scores.sort();
    scores.reverse();
    score = if scores[0] == 5 {
        7
    } else if scores[0] == 4 {
        6
    } else if scores[0] == 3 && scores[1] == 2 {
        5
    } else if scores[0] == 3 && scores[1] == 1 {
        4
    } else if scores[0] == 2 && scores[1] == 2 {
        3
    } else if scores[0] == 2 && scores[1] == 1 {
        2
    } else {
        1
    };

    game = Game {
        cards,
        bid,
        score
    };

    return game;
}

fn new_card_to_index() -> HashMap<char, usize> {
    let mut card_to_index: HashMap<char, usize> = HashMap::new();

    card_to_index.insert('2', 0);
    card_to_index.insert('3', 1);
    card_to_index.insert('4', 2);
    card_to_index.insert('5', 3);
    card_to_index.insert('6', 4);
    card_to_index.insert('7', 5);
    card_to_index.insert('8', 6);
    card_to_index.insert('9', 7);
    card_to_index.insert('T', 8);
    card_to_index.insert('J', 9);
    card_to_index.insert('Q', 10);
    card_to_index.insert('K', 11);
    card_to_index.insert('A', 12);

    return card_to_index;
}

fn main() {
    let stdin = io::stdin();
    let mut outlet: i32 = 0;
    let mut inlet: String;
    let mut games: Vec<Game> = Vec::new();
    let mut game: Game;
    let card_to_index: HashMap<char, usize> = new_card_to_index();
    let constants = Constants {
        card_to_index
    };

	for line in stdin.lock().lines() {
        inlet = line.unwrap();
        if inlet.len() > 0 {
            game = parse_game(inlet.clone(), &constants);
            games.push(game); 
        }
	}

    games.sort_by(|a, b| compare_games(a, b));
    for (i, game) in games.into_iter().enumerate() {
        outlet += game.bid * (1 + i as i32);
    }

    println!("{}", outlet);
}
