use std::fs::read_to_string;
use std::vec;

use lazy_static::lazy_static;
use std::cmp::Ordering;
use std::collections::HashMap;

lazy_static! {
    static ref CARDS_VALUE_PART1: HashMap<char, usize> = {
        let mut map = HashMap::new();
        map.insert('A', 12);
        map.insert('K', 11);
        map.insert('Q', 10);
        map.insert('J', 9);
        map.insert('T', 8);
        map.insert('9', 7);
        map.insert('8', 6);
        map.insert('7', 5);
        map.insert('6', 4);
        map.insert('5', 3);
        map.insert('4', 2);
        map.insert('3', 1);
        map.insert('2', 0);
        map
    };
}

lazy_static! {
    static ref CARDS_VALUE_PART2: HashMap<char, usize> = {
        let mut map = HashMap::new();
        map.insert('A', 12);
        map.insert('K', 11);
        map.insert('Q', 10);
        map.insert('T', 9);
        map.insert('9', 8);
        map.insert('8', 7);
        map.insert('7', 6);
        map.insert('6', 5);
        map.insert('5', 4);
        map.insert('4', 3);
        map.insert('3', 2);
        map.insert('2', 1);
        map.insert('J', 0);
        map
    };
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone, Copy)]
struct Card {
    card: char,
    value: usize,
}

impl Card {
    fn new(value: char, cards_value: &HashMap<char, usize>) -> Self {
        Self {
            card: value,
            value: cards_value[&value],
        }
    }
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
enum Plays {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
    None = 0,
}

#[derive(Debug, Eq)]
struct Play {
    play: String,
    cards: [Card; 5],
    bid: usize,
    play_type: Plays,
}

impl Play {
    fn new(play: &str, cards_value: &HashMap<char, usize>, use_jokers: bool) -> Self {
        let split_value: Vec<&str> = play.split(" ").collect();
        let cards_string = split_value[0];
        let bid = split_value[1].parse::<usize>().unwrap();

        let mut inst = Self {
            play: cards_string.to_string(),
            cards: [Card::new('2', cards_value); 5],
            bid,
            play_type: Plays::None,
        };
        for (i, card) in cards_string.chars().enumerate() {
            inst.cards[i] = Card::new(card, cards_value);
        }
        if use_jokers {
            inst.play_type = inst._get_play_with_jokers();
        } else {
            inst.play_type = inst._get_play_without_jokers();
        }
        inst
    }

    fn _get_play_without_jokers(&self) -> Plays {
        let mut cards_map: HashMap<char, usize> = HashMap::new();

        let mut cards_string = self.play.clone();
        while cards_string != "" {
            let current_char = cards_string.as_bytes()[0] as char;
            let cards_count = cards_string.matches(current_char).count();
            cards_map.insert(current_char, cards_count);
            cards_string = cards_string.replace(current_char, "");
        }
        let number_of_keys = cards_map.keys().count();
        match number_of_keys {
            1 => return Plays::FiveOfAKind,
            2 => {
                let values = cards_map.into_values().collect::<Vec<usize>>();
                if values.contains(&4) {
                    return Plays::FourOfAKind;
                }
                return Plays::FullHouse;
            }
            3 => {
                let values = cards_map.into_values().collect::<Vec<usize>>();
                if values.contains(&3) {
                    return Plays::ThreeOfAKind;
                }
                return Plays::TwoPair;
            }
            4 => return Plays::OnePair,
            5 => return Plays::HighCard,
            _ => return Plays::None,
        }
    }

    fn _get_play_with_jokers(&self) -> Plays {
        let mut cards_map: HashMap<char, usize> = HashMap::new();

        let mut cards_string = self.play.clone();
        while cards_string != "" {
            let current_char = cards_string.as_bytes()[0] as char;
            let cards_count = cards_string.matches(current_char).count();
            cards_map.insert(current_char, cards_count);
            cards_string = cards_string.replace(current_char, "");
        }
        let jokers = cards_map.get_key_value(&'J');

        if jokers.is_some() {
            let number_of_jokers = cards_map.remove(&'J').unwrap();
            let max_value = cards_map.clone().into_values().max();
            match max_value {
                Some(max_value) => {
                    for (_, number_of_cards) in cards_map.iter_mut() {
                        if *number_of_cards == max_value {
                            *number_of_cards += number_of_jokers;
                            break;
                        }
                    }
                }
                None => {
                    cards_map.insert('J', 5);
                }
            }
        }
        let number_of_keys = cards_map.keys().count();
        match number_of_keys {
            1 => return Plays::FiveOfAKind,
            2 => {
                let values = cards_map.into_values().collect::<Vec<usize>>();
                if values.contains(&4) {
                    return Plays::FourOfAKind;
                }
                return Plays::FullHouse;
            }
            3 => {
                let values = cards_map.into_values().collect::<Vec<usize>>();
                if values.contains(&3) {
                    return Plays::ThreeOfAKind;
                }
                return Plays::TwoPair;
            }
            4 => return Plays::OnePair,
            5 => return Plays::HighCard,
            _ => return Plays::None,
        }
    }
}

impl PartialEq for Play {
    fn eq(&self, other: &Play) -> bool {
        self.bid == other.bid && self.play == other.play
    }
}

impl Ord for Play {
    fn cmp(&self, other: &Play) -> Ordering {
        if self.play_type < other.play_type {
            return Ordering::Less;
        } else if self.play_type > other.play_type {
            return Ordering::Greater;
        }
        return Ordering::Equal;
    }
}

impl PartialOrd for Play {
    fn partial_cmp(&self, other: &Play) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn order_plays(plays: &mut Vec<Play>) {
    plays.sort();

    let mut i = 0;
    while i < plays.len() {
        let current_play = &plays[i];
        let mut decrement_value = false;
        let next_card_i: usize = i + 1;

        if i + 1 >= plays.len() {
            break;
        }
        let next_play = &plays[i + 1];
        if current_play.play_type != next_play.play_type {
            i += 1;
            continue;
        }

        for (card_i, card) in current_play.cards.iter().enumerate() {
            if card.value == next_play.cards[card_i].value {
                continue;
            }
            if card.value > next_play.cards[card_i].value {
                plays.swap(next_card_i, i);
                decrement_value = true;
            }
            break;
        }
        if decrement_value {
            if i > 0 {
                i -= 1;
            }
        } else {
            i += 1;
        }
    }
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

fn part1(file_name: &str) -> usize {
    let lines = read_lines(file_name);

    let mut plays: Vec<Play> = vec![];

    for line in lines.into_iter() {
        let play = Play::new(&line, &CARDS_VALUE_PART1, false);
        plays.push(play);
    }
    order_plays(&mut plays);

    let mut sum: usize = 0;
    for (rank, play) in plays.iter().enumerate() {
        sum += (rank + 1) * play.bid;
    }

    sum
}

fn part2(file_name: &str) -> usize {
    let lines = read_lines(file_name);

    let mut plays: Vec<Play> = vec![];

    for line in lines.into_iter() {
        let play = Play::new(&line, &CARDS_VALUE_PART2, true);
        plays.push(play);
    }
    order_plays(&mut plays);

    let mut sum: usize = 0;
    for (rank, play) in plays.iter().enumerate() {
        sum += (rank + 1) * play.bid;
    }

    sum
}

fn main() {
    let part1_small_res = part1("small_input.txt");
    assert_eq!(part1_small_res, 6440);

    let part1_res = part1("input.txt");

    let part2_small_res = part2("small_input.txt");
    assert_eq!(part2_small_res, 5905);

    let part2_res = part2("input.txt");

    println!("Part 1 sum: {part1_res}");
    println!("Part 2 sum: {part2_res}");
}
