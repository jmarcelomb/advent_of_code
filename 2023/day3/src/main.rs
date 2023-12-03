use core::num;
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

fn parse_number(line_str: &str, char_i: usize) -> (usize, usize) {
    let mut number_start = char_i;
    while number_start != 0 && line_str.as_bytes()[number_start].is_ascii_digit() {
        number_start -= 1;
    }
    if !line_str.as_bytes()[number_start].is_ascii_digit() {
        number_start += 1;
    }

    let mut offset = 0;
    for character in line_str[char_i..].as_bytes() {
        if character.is_ascii_digit() {
            offset += 1;
            continue;
        }
        break;
    }

    let number = line_str[number_start..char_i + offset]
        .parse::<usize>()
        .unwrap();
    (offset, number)
}

fn part1(file_name: &str) -> usize {
    let lines = read_lines(file_name);

    let mut sum: usize = 0;

    for (line_i, line) in lines.clone().into_iter().enumerate() {
        let mut char_i = 0;
        while char_i < line.len() {
            if !line.as_bytes()[char_i].is_ascii_digit() {
                char_i += 1;
                continue;
            }
            // Top
            if line_i != 0
                && lines[line_i - 1].as_bytes()[char_i] != b'.'
                && !lines[line_i - 1].as_bytes()[char_i].is_ascii_digit()
            {
                let (offset, number) = parse_number(&line, char_i);
                sum += number;
                char_i += offset;
                continue;
            }
            // Left diagnoal
            if line_i != 0
                && char_i != 0
                && lines[line_i - 1].as_bytes()[char_i - 1] != b'.'
                && !lines[line_i - 1].as_bytes()[char_i - 1].is_ascii_digit()
            {
                let (offset, number) = parse_number(&line, char_i);
                sum += number;
                char_i += offset;
                continue;
            }
            // Left
            if char_i != 0
                && lines[line_i].as_bytes()[char_i - 1] != b'.'
                && !lines[line_i].as_bytes()[char_i - 1].is_ascii_digit()
            {
                let (offset, number) = parse_number(&line, char_i);
                sum += number;
                char_i += offset;
                continue;
            }
            // Bottom Left diagonal
            if char_i != 0
                && line_i + 1 < lines.len()
                && lines[line_i + 1].as_bytes()[char_i - 1] != b'.'
                && !lines[line_i + 1].as_bytes()[char_i - 1].is_ascii_digit()
            {
                let (offset, number) = parse_number(&line, char_i);
                sum += number;
                char_i += offset;
                continue;
            }
            // Bottom
            if line_i + 1 < lines.len()
                && lines[line_i + 1].as_bytes()[char_i] != b'.'
                && !lines[line_i + 1].as_bytes()[char_i].is_ascii_digit()
            {
                let (offset, number) = parse_number(&line, char_i);
                sum += number;
                char_i += offset;
                continue;
            }
            // Bottom Diagonal Right
            if line_i + 1 < lines.len()
                && char_i + 1 < line.len()
                && lines[line_i + 1].as_bytes()[char_i + 1] != b'.'
                && !lines[line_i + 1].as_bytes()[char_i + 1].is_ascii_digit()
            {
                let (offset, number) = parse_number(&line, char_i);
                sum += number;
                char_i += offset;
                continue;
            }
            // Right
            if char_i + 1 < line.len()
                && lines[line_i].as_bytes()[char_i + 1] != b'.'
                && !lines[line_i].as_bytes()[char_i + 1].is_ascii_digit()
            {
                let (offset, number) = parse_number(&line, char_i);
                sum += number;
                char_i += offset;
                continue;
            }
            // Top Diagonal Right
            if char_i + 1 < line.len()
                && line_i != 0
                && lines[line_i - 1].as_bytes()[char_i + 1] != b'.'
                && !lines[line_i - 1].as_bytes()[char_i + 1].is_ascii_digit()
            {
                let (offset, number) = parse_number(&line, char_i);
                sum += number;
                char_i += offset;
                continue;
            }
            char_i += 1;
        }
    }
    sum
}

fn part2(file_name: &str) -> usize {
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);

    let mut sum: usize = 0;

    for line_res in reader.lines() {
        let line = line_res.unwrap();
    }
    sum
}

fn main() {
    let part1_small_res = part1("small_input.txt");
    assert_eq!(part1_small_res, 4361);

    let part1_res = part1("input.txt");
    let part2_res = part2("input.txt");

    println!("Part 1 sum: {part1_res}");
    println!("Part 2 sum: {part2_res}");
}
