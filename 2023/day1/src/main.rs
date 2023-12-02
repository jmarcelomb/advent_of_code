use std::u64;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn part1(file_name: &str) -> u64 {
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);

    let mut sum: u64 = 0;

    for line in reader.lines() {
        let mut first_entry: Option<u64> = None;
        let mut last_entry: u64 = 0;
        let line = line.unwrap();
        //pintln!("{:?}", line);
        for character in line.chars() {
            if character.is_ascii_digit() {
                let digit = character.to_digit(10).unwrap() as u64;
                if first_entry.is_none() {
                    first_entry = Some(digit);
                }
                last_entry = digit;
            }
        }
        //println!("First: {}, Last: {last_entry}\n", first_entry.unwrap());
        sum = sum + first_entry.unwrap() * 10 + last_entry;
    }
    sum
}

fn part2(file_name: &str) -> u64 {
    let string_digits: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);

    let mut sum: u64 = 0;

    for line in reader.lines() {
        let mut first_entry: Option<u64> = None;
        let mut last_entry: u64 = 0;
        let line = line.unwrap();
        println!("{:?}", line);
        for i in 0..line.len() {
            let character: char = line.as_bytes()[i] as char;
            if character.is_ascii_digit() {
                let digit = character.to_digit(10).unwrap() as u64;
                if first_entry.is_none() {
                    first_entry = Some(digit);
                }
                last_entry = digit;
            } else {
                for string_digit_i in 0..string_digits.len() {
                    if line[i..].starts_with(string_digits[string_digit_i]) {
                        if first_entry.is_none() {
                            first_entry = Some((string_digit_i + 1) as u64);
                        }
                        last_entry = (string_digit_i + 1) as u64;
                    }
                }
            }
        }
        println!("First: {}, Last: {last_entry}\n", first_entry.unwrap());
        sum = sum + first_entry.unwrap() * 10 + last_entry;
    }
    sum
}

fn main() {
    let part1_res = part1("input.txt");
    let part2_res = part2("input.txt");

    println!("Part 1 sum: {part1_res}");
    println!("Part 2 sum: {part2_res}");
}

