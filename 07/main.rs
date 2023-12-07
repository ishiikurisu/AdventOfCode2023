use std::io::{self, BufRead};
use std::collections::HashMap;
use std::cmp::Ordering;

struct Game {
    cards: Vec<Vec<i32>>,
    bid: i32,
    score: i32,
    hand: String,
    card_to_index: HashMap<char, usize>,
}

fn new_card_to_index() -> HashMap<char, usize> {
    let mut card_to_index: HashMap<char, usize> = HashMap::new();

    for (i, c) in "23456789TJQKA".chars().enumerate() {
        card_to_index.insert(c, i);
    }

    return card_to_index;
}

fn compare_games(a: &Game, b: &Game) -> Ordering {
    let mut card_a: usize;
    let mut card_b: usize;

    if a.score > b.score {
        return Ordering::Greater;
    } else if a.score < b.score {
        return Ordering::Less;
    }

    for (i, ca) in a.hand.chars().enumerate() {
        card_a = *a.card_to_index.get(&ca).unwrap();
        card_b = *b.card_to_index.get(&b.hand.chars().nth(i).unwrap()).unwrap();
        if card_a > card_b {
            return Ordering::Greater;
        } else if card_a < card_b {
            return Ordering::Less;
        }
    }

    return Ordering::Equal;
}

fn compare_card_scores(a: &Vec<i32>, b: &Vec<i32>) -> Ordering {
    // a, b are [card, count]
    return if a[1] > b[1] {
        Ordering::Greater
    } else if a[1] < b[1] {
        Ordering::Less
    } else if a[0] > b[0] {
        Ordering::Greater
    } else if a[0] < b[0] {
        Ordering::Less
    } else {
        Ordering::Equal
    };
}

fn parse_game(inlet: String) -> Game {
    let game: Game;
    let fields: Vec<&str> = inlet.split(" ").collect();
    let hand: String = fields[0].to_string();
    let bid: i32 = fields[1].parse().unwrap();
    let mut card_count: Vec<i32> = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0].into_iter().collect();
    let score: i32;
    let mut scores: Vec<i32>;
    let mut cards: Vec<Vec<i32>> = Vec::new();
    let mut card_count_pair: Vec<i32>;
    let card_to_index: HashMap<char, usize> = new_card_to_index();

    for card in hand.chars() {
        card_count[card_to_index[&card]] += 1;
    }

    // creating score cache
    scores = card_count.clone().into_iter().collect();
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

    // creating card order cache
    for (card, count) in card_count.clone().into_iter().enumerate() {
        card_count_pair = Vec::new();
        card_count_pair.push(card as i32);
        card_count_pair.push(count);
        cards.push(card_count_pair);
    }
    cards.sort_by(|a, b| compare_card_scores(a, b));
    cards.reverse();

    game = Game {
        cards,
        bid,
        score,
        hand,
        card_to_index,
    };

    return game;
}

fn main() {
    let stdin = io::stdin();
    let mut outlet: i32 = 0;
    let mut inlet: String;
    let mut games: Vec<Game> = Vec::new();
    let mut game: Game;

	for line in stdin.lock().lines() {
        inlet = line.unwrap();
        if inlet.len() > 0 {
            game = parse_game(inlet.clone());
            games.push(game); 
        }
	}

    games.sort_by(|a, b| compare_games(a, b));
    for (i, game) in games.into_iter().enumerate() {
        outlet += game.bid * (1 + i as i32);
    }

    println!("{}", outlet);
}
