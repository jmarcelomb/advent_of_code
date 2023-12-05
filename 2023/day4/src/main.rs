use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}

fn part1(file_name: &str) -> u64 {
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);

    let mut sum: u64 = 0;

    for line_res in reader.lines() {
        let line = line_res.unwrap();
        let dot_pos = line.find(':').unwrap();
        let pipe_pos = line.find('|').unwrap();

        let wining_numbers: Vec<usize> = line[dot_pos + 1..pipe_pos]
            .split(' ')
            .filter(|number_str| *number_str != "")
            .map(|x| x.trim().parse::<usize>().unwrap())
            .collect();

        let numbers: Vec<usize> = line[pipe_pos + 1..]
            .split(' ')
            .filter(|number_str| *number_str != "")
            .map(|x| x.trim().parse::<usize>().unwrap())
            .collect();

        //println!("{:?}", wining_numbers);
        //println!("{:?}", numbers);

        let mut win_cards = 0;
        for wining_number in wining_numbers.into_iter() {
            if numbers.contains(&wining_number) {
                if win_cards == 0 {
                    win_cards = 1;
                } else {
                    win_cards <<= 1;
                }
            }
        }
        //println!("Points: {win_cards}");
        sum += win_cards;
        //println!();
    }
    sum
}

fn get_card_wins(line: &str) -> usize {
    let dot_pos = line.find(':').unwrap();
    let pipe_pos = line.find('|').unwrap();

    let wining_numbers: Vec<usize> = line[dot_pos + 1..pipe_pos]
        .split(' ')
        .filter(|number_str| *number_str != "")
        .map(|x| x.trim().parse::<usize>().unwrap())
        .collect();

    let numbers: Vec<usize> = line[pipe_pos + 1..]
        .split(' ')
        .filter(|number_str| *number_str != "")
        .map(|x| x.trim().parse::<usize>().unwrap())
        .collect();

    let mut win_cards = 0;
    for wining_number in wining_numbers.into_iter() {
        if numbers.contains(&wining_number) {
            win_cards += 1;
        }
    }
    win_cards
}

fn process_cards_won(cards_score: &mut Vec<usize>, lines: &Vec<String>, i: usize) {
    let cards_won = get_card_wins(&lines[i]);
    cards_score[i] += 1;
    for card_won in 1..cards_won + 1 {
        if i + card_won == lines.len() {
            break;
        }
        process_cards_won(cards_score, &lines, i + card_won);
    }
}

fn part2(file_name: &str) -> usize {
    let lines = read_lines(file_name);

    let mut cards_score: Vec<usize> = vec![0; lines.len()];

    for i in 0..lines.len() {
        println!("Doing line {i}..");
        process_cards_won(&mut cards_score, &lines, i);
    }

    let sum = cards_score.iter().sum();
    sum
}

fn main() {
    let part1_small_res = part1("small_input.txt");
    assert_eq!(part1_small_res, 13);

    let part1_res = part1("input.txt");

    let part2_small_res = part2("small_input.txt");
    assert_eq!(part2_small_res, 30);

    let part2_res = part2("input.txt");

    println!("Part 1 sum: {part1_res}");
    println!("Part 2 sum: {part2_res}");
}
